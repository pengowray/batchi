use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys;
use wasm_bindgen_futures::JsFuture;
use web_sys::{File, FileReader};
use crate::audio::loader::load_audio;
use crate::dsp::fft::{compute_preview, compute_spectrogram_partial};
use crate::state::{AppState, LoadedFile};
use crate::types::SpectrogramData;

pub(super) async fn read_and_load_file(file: File, state: AppState) -> Result<(), String> {
    let name = file.name();
    let bytes = read_file_bytes(&file).await?;
    load_named_bytes(name, &bytes, None, state).await
}

async fn load_named_bytes(name: String, bytes: &[u8], xc_metadata: Option<Vec<(String, String)>>, state: AppState) -> Result<(), String> {
    let audio = load_audio(bytes)?;
    log::info!(
        "Loaded {}: {} samples, {} Hz, {:.2}s",
        name,
        audio.samples.len(),
        audio.sample_rate,
        audio.duration_secs
    );

    // Phase 1: fast preview
    let preview = compute_preview(&audio, 256, 128);
    let audio_for_stft = audio.clone();
    let name_check = name.clone();

    let placeholder_spec = SpectrogramData {
        columns: Vec::new(),
        freq_resolution: 0.0,
        time_resolution: 0.0,
        max_freq: audio.sample_rate as f64 / 2.0,
        sample_rate: audio.sample_rate,
    };

    let file_index;
    {
        let mut idx = 0;
        state.files.update(|files| {
            idx = files.len();
            files.push(LoadedFile {
                name,
                audio,
                spectrogram: placeholder_spec,
                preview: Some(preview),
                xc_metadata,
                is_recording: false,
            });
            if files.len() == 1 {
                state.current_file_index.set(Some(0));
            }
        });
        file_index = idx;
    }

    // Yield to let the UI render the preview
    let yield_promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback(&resolve)
            .unwrap();
    });
    JsFuture::from(yield_promise).await.ok();

    // Phase 2: full spectrogram — computed in small chunks so the browser
    // stays responsive.  Each chunk yields via setTimeout(0) before continuing.
    const FFT_SIZE: usize = 2048;
    const HOP_SIZE: usize = 512;
    const CHUNK_COLS: usize = 32; // ~50 ms of work per chunk on typical hardware

    let total_cols = if audio_for_stft.samples.len() >= FFT_SIZE {
        (audio_for_stft.samples.len() - FFT_SIZE) / HOP_SIZE + 1
    } else {
        0
    };

    let mut all_columns: Vec<crate::types::SpectrogramColumn> = Vec::with_capacity(total_cols);

    let mut chunk_start = 0;
    while chunk_start < total_cols {
        // Check the file is still loaded (user may have removed it)
        let still_present = state.files.get_untracked()
            .get(file_index)
            .map(|f| f.name == name_check)
            .unwrap_or(false);
        if !still_present { return Ok(()); }

        let chunk = compute_spectrogram_partial(
            &audio_for_stft,
            FFT_SIZE,
            HOP_SIZE,
            chunk_start,
            CHUNK_COLS,
        );
        all_columns.extend(chunk);
        chunk_start += CHUNK_COLS;

        // Yield so the browser can process events / paint between chunks
        let p = js_sys::Promise::new(&mut |resolve, _| {
            web_sys::window().unwrap().set_timeout_with_callback(&resolve).unwrap();
        });
        JsFuture::from(p).await.ok();
    }

    let freq_resolution = audio_for_stft.sample_rate as f64 / FFT_SIZE as f64;
    let time_resolution = HOP_SIZE as f64 / audio_for_stft.sample_rate as f64;
    let max_freq = audio_for_stft.sample_rate as f64 / 2.0;

    let spectrogram = SpectrogramData {
        columns: all_columns,
        freq_resolution,
        time_resolution,
        max_freq,
        sample_rate: audio_for_stft.sample_rate,
    };

    log::info!(
        "Spectrogram: {} columns, freq_res={:.1} Hz, time_res={:.4}s",
        spectrogram.columns.len(),
        spectrogram.freq_resolution,
        spectrogram.time_resolution
    );

    state.files.update(|files| {
        if let Some(f) = files.get_mut(file_index) {
            if f.name == name_check {
                f.spectrogram = spectrogram;
            }
        }
    });

    Ok(())
}

const DEMO_SOUNDS_BASE: &str =
    "https://raw.githubusercontent.com/pengowray/batchi-demo-sounds/main";

pub(super) async fn fetch_bytes(url: &str) -> Result<Vec<u8>, String> {
    let window = web_sys::window().ok_or("No window")?;
    let resp_value = JsFuture::from(window.fetch_with_str(url))
        .await
        .map_err(|e| format!("fetch error: {e:?}"))?;
    let resp: web_sys::Response = resp_value
        .dyn_into()
        .map_err(|_| "Response cast failed".to_string())?;
    if !resp.ok() {
        return Err(format!("HTTP {}", resp.status()));
    }
    let buf = JsFuture::from(resp.array_buffer().map_err(|e| format!("{e:?}"))?)
        .await
        .map_err(|e| format!("array_buffer: {e:?}"))?;
    let uint8 = js_sys::Uint8Array::new(&buf);
    Ok(uint8.to_vec())
}

async fn fetch_text(url: &str) -> Result<String, String> {
    let window = web_sys::window().ok_or("No window")?;
    let resp_value = JsFuture::from(window.fetch_with_str(url))
        .await
        .map_err(|e| format!("fetch error: {e:?}"))?;
    let resp: web_sys::Response = resp_value
        .dyn_into()
        .map_err(|_| "Response cast failed".to_string())?;
    if !resp.ok() {
        return Err(format!("HTTP {}", resp.status()));
    }
    let text = JsFuture::from(resp.text().map_err(|e| format!("{e:?}"))?)
        .await
        .map_err(|e| format!("text: {e:?}"))?;
    text.as_string().ok_or("Not a string".to_string())
}

fn parse_xc_metadata(json: &serde_json::Value) -> Vec<(String, String)> {
    let mut fields = Vec::new();
    let s = |key: &str| json[key].as_str().unwrap_or("").to_string();

    let en = s("en");
    if !en.is_empty() {
        fields.push(("Species".into(), en));
    }
    let gen = s("gen");
    let sp = s("sp");
    if !gen.is_empty() && !sp.is_empty() {
        fields.push(("Scientific name".into(), format!("{} {}", gen, sp)));
    }
    for (key, label) in [
        ("rec", "Recordist"),
        ("lic", "License"),
        ("attribution", "Attribution"),
        ("cnt", "Country"),
        ("loc", "Location"),
    ] {
        let v = s(key);
        if !v.is_empty() {
            fields.push((label.into(), v));
        }
    }
    let lat = s("lat");
    let lon = s("lon");
    if !lat.is_empty() && !lon.is_empty() {
        fields.push(("Coordinates".into(), format!("{}, {}", lat, lon)));
    }
    for (key, label) in [
        ("date", "Date"),
        ("type", "Sound type"),
        ("q", "Quality"),
        ("url", "URL"),
    ] {
        let v = s(key);
        if !v.is_empty() {
            fields.push((label.into(), v));
        }
    }
    fields
}

#[derive(Clone, Debug)]
pub(super) struct DemoEntry {
    pub filename: String,
    pub metadata_file: Option<String>,
}

pub(super) async fn fetch_demo_index() -> Result<Vec<DemoEntry>, String> {
    let index_url = format!("{}/index.json", DEMO_SOUNDS_BASE);
    let index_text = fetch_text(&index_url).await?;
    let index: serde_json::Value =
        serde_json::from_str(&index_text).map_err(|e| format!("index parse: {e}"))?;

    let sounds = index["sounds"]
        .as_array()
        .ok_or("No sounds array in index")?;

    let entries = sounds
        .iter()
        .filter_map(|sound| {
            let filename = sound["filename"].as_str()?.to_string();
            let metadata_file = sound["metadata"].as_str().map(|s| s.to_string());
            Some(DemoEntry { filename, metadata_file })
        })
        .collect();

    Ok(entries)
}

pub(super) async fn load_single_demo(entry: &DemoEntry, state: AppState) -> Result<(), String> {
    // Fetch XC metadata sidecar if available
    let xc_metadata = if let Some(meta_file) = &entry.metadata_file {
        let encoded = js_sys::encode_uri_component(meta_file);
        let meta_url = format!(
            "{}/sounds/{}",
            DEMO_SOUNDS_BASE,
            encoded.as_string().unwrap_or_default()
        );
        match fetch_text(&meta_url).await {
            Ok(text) => {
                match serde_json::from_str::<serde_json::Value>(&text) {
                    Ok(json) => Some(parse_xc_metadata(&json)),
                    Err(e) => {
                        log::warn!("Failed to parse XC metadata for {}: {}", entry.filename, e);
                        None
                    }
                }
            }
            Err(e) => {
                log::warn!("Failed to fetch XC metadata for {}: {}", entry.filename, e);
                None
            }
        }
    } else {
        None
    };

    let encoded = js_sys::encode_uri_component(&entry.filename);
    let audio_url = format!(
        "{}/sounds/{}",
        DEMO_SOUNDS_BASE,
        encoded.as_string().unwrap_or_default()
    );
    log::info!("Fetching demo: {}", entry.filename);
    let bytes = fetch_bytes(&audio_url).await?;
    load_named_bytes(entry.filename.clone(), &bytes, xc_metadata, state).await
}

async fn read_file_bytes(file: &File) -> Result<Vec<u8>, String> {
    let reader = FileReader::new().map_err(|e| format!("FileReader: {e:?}"))?;
    let reader_clone = reader.clone();

    let promise = js_sys::Promise::new(&mut |resolve, reject| {
        let resolve_clone = resolve.clone();
        let reject_clone = reject.clone();

        let onload = Closure::once(move |_: web_sys::Event| {
            resolve_clone.call0(&JsValue::NULL).unwrap();
        });
        let onerror = Closure::once(move |_: web_sys::Event| {
            reject_clone.call0(&JsValue::NULL).unwrap();
        });

        reader_clone.set_onloadend(Some(onload.as_ref().unchecked_ref()));
        reader_clone.set_onerror(Some(onerror.as_ref().unchecked_ref()));

        onload.forget();
        onerror.forget();
    });

    reader
        .read_as_array_buffer(file)
        .map_err(|e| format!("read_as_array_buffer: {e:?}"))?;

    JsFuture::from(promise)
        .await
        .map_err(|e| format!("FileReader await: {e:?}"))?;

    let result = reader.result().map_err(|e| format!("result: {e:?}"))?;
    let array_buffer = result
        .dyn_into::<js_sys::ArrayBuffer>()
        .map_err(|_| "Expected ArrayBuffer".to_string())?;
    let uint8_array = js_sys::Uint8Array::new(&array_buffer);
    Ok(uint8_array.to_vec())
}

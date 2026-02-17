use leptos::prelude::*;
use leptos::task::spawn_local;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{Clamped, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{CanvasRenderingContext2d, DragEvent, File, FileReader, HtmlCanvasElement, ImageData};
use crate::audio::loader::load_audio;
use crate::dsp::fft::{compute_preview, compute_spectrogram};
use crate::state::{AppState, LoadedFile, SidebarTab, SpectrogramDisplay};
use crate::types::{PreviewImage, SpectrogramData};

#[component]
pub fn FileSidebar() -> impl IntoView {
    let state = expect_context::<AppState>();

    view! {
        <div class="sidebar">
            <div class="sidebar-tabs">
                <button
                    class=move || if state.sidebar_tab.get() == SidebarTab::Files { "sidebar-tab active" } else { "sidebar-tab" }
                    on:click=move |_| state.sidebar_tab.set(SidebarTab::Files)
                >
                    "Files"
                </button>
                <button
                    class=move || if state.sidebar_tab.get() == SidebarTab::Spectrogram { "sidebar-tab active" } else { "sidebar-tab" }
                    on:click=move |_| state.sidebar_tab.set(SidebarTab::Spectrogram)
                >
                    "Display"
                </button>
            </div>
            {move || match state.sidebar_tab.get() {
                SidebarTab::Files => view! { <FilesPanel /> }.into_any(),
                SidebarTab::Spectrogram => view! { <SpectrogramSettingsPanel /> }.into_any(),
            }}
        </div>
    }
}

#[component]
fn FilesPanel() -> impl IntoView {
    let state = expect_context::<AppState>();
    let drag_over = RwSignal::new(false);
    let files = state.files;
    let current_idx = state.current_file_index;
    let loading_count = state.loading_count;

    let on_dragover = move |ev: DragEvent| {
        ev.prevent_default();
        drag_over.set(true);
    };

    let on_dragleave = move |_: DragEvent| {
        drag_over.set(false);
    };

    let state_for_drop = state.clone();
    let on_drop = move |ev: DragEvent| {
        ev.prevent_default();
        drag_over.set(false);

        let Some(dt) = ev.data_transfer() else { return };
        let Some(file_list) = dt.files() else { return };

        for i in 0..file_list.length() {
            let Some(file) = file_list.get(i) else { continue };
            let state = state_for_drop.clone();
            state.loading_count.update(|c| *c += 1);
            spawn_local(async move {
                match read_and_load_file(file, state.clone()).await {
                    Ok(()) => {}
                    Err(e) => log::error!("Failed to load file: {e}"),
                }
                state.loading_count.update(|c| *c = c.saturating_sub(1));
            });
        }
    };

    view! {
        <div
            class=move || if drag_over.get() { "drop-zone drag-over" } else { "drop-zone" }
            on:dragover=on_dragover
            on:dragleave=on_dragleave
            on:drop=on_drop
        >
            {move || {
                let file_vec = files.get();
                let lc = loading_count.get();
                if file_vec.is_empty() && lc == 0 {
                    view! {
                        <div class="drop-hint">"Drop WAV/FLAC files here"</div>
                    }.into_any()
                } else {
                    let items: Vec<_> = file_vec.iter().enumerate().map(|(i, f)| {
                        let name = f.name.clone();
                        let dur = f.audio.duration_secs;
                        let sr = f.audio.sample_rate;
                        let preview = f.preview.clone();
                        let is_active = move || current_idx.get() == Some(i);
                        let on_click = move |_| {
                            current_idx.set(Some(i));
                        };
                        view! {
                            <div
                                class=move || if is_active() { "file-item active" } else { "file-item" }
                                on:click=on_click
                            >
                                {preview.map(|pv| view! { <PreviewCanvas preview=pv /> })}
                                <div class="file-item-name">{name}</div>
                                <div class="file-item-info">
                                    {format!("{:.1}s  {}kHz", dur, sr / 1000)}
                                </div>
                            </div>
                        }
                    }).collect();
                    view! {
                        <div class="file-list">
                            {items}
                            {move || {
                                let lc = loading_count.get();
                                if lc > 0 {
                                    view! {
                                        <div class="file-item loading">
                                            <div class="loading-spinner"></div>
                                            {format!("Loading {} file{}...", lc, if lc > 1 { "s" } else { "" })}
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <span></span> }.into_any()
                                }
                            }}
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}

#[component]
fn SpectrogramSettingsPanel() -> impl IntoView {
    let state = expect_context::<AppState>();

    let on_toggle_change = move |ev: web_sys::Event| {
        let target = ev.target().unwrap();
        let input: web_sys::HtmlInputElement = target.unchecked_into();
        state.mv_enabled.set(input.checked());
    };

    let on_display_change = move |ev: web_sys::Event| {
        let target = ev.target().unwrap();
        let select: web_sys::HtmlSelectElement = target.unchecked_into();
        let mode = match select.value().as_str() {
            "centroid" => SpectrogramDisplay::MovementCentroid,
            "gradient" => SpectrogramDisplay::MovementGradient,
            _ => SpectrogramDisplay::MovementFlow,
        };
        state.spectrogram_display.set(mode);
    };

    let on_intensity_gate_change = move |ev: web_sys::Event| {
        let target = ev.target().unwrap();
        let input: web_sys::HtmlInputElement = target.unchecked_into();
        if let Ok(val) = input.value().parse::<f32>() {
            state.mv_intensity_gate.set(val / 100.0);
        }
    };

    let on_movement_gate_change = move |ev: web_sys::Event| {
        let target = ev.target().unwrap();
        let input: web_sys::HtmlInputElement = target.unchecked_into();
        if let Ok(val) = input.value().parse::<f32>() {
            state.mv_movement_gate.set(val / 100.0);
        }
    };

    let on_opacity_change = move |ev: web_sys::Event| {
        let target = ev.target().unwrap();
        let input: web_sys::HtmlInputElement = target.unchecked_into();
        if let Ok(val) = input.value().parse::<f32>() {
            state.mv_opacity.set(val / 100.0);
        }
    };

    let on_max_freq_change = move |ev: web_sys::Event| {
        let target = ev.target().unwrap();
        let select: web_sys::HtmlSelectElement = target.unchecked_into();
        let freq = match select.value().as_str() {
            "auto" => None,
            v => v.parse::<f64>().ok().map(|khz| khz * 1000.0),
        };
        state.max_display_freq.set(freq);
    };

    view! {
        <div class="sidebar-panel">
            // Max display frequency section (first)
            <div class="setting-group">
                <div class="setting-group-title">"Display"</div>
                <div class="setting-row">
                    <span class="setting-label">"Max freq"</span>
                    <select
                        class="setting-select"
                        on:change=on_max_freq_change
                        prop:value=move || match state.max_display_freq.get() {
                            None => "auto".to_string(),
                            Some(hz) => format!("{}", (hz / 1000.0) as u32),
                        }
                    >
                        <option value="auto">"Auto"</option>
                        <option value="50">"50 kHz"</option>
                        <option value="100">"100 kHz"</option>
                        <option value="150">"150 kHz"</option>
                        <option value="200">"200 kHz"</option>
                        <option value="250">"250 kHz"</option>
                    </select>
                </div>
            </div>

            // Movement overlay section
            <div class="setting-group">
                <div class="setting-group-title">"Movement overlay"</div>
                <div class="setting-row">
                    <span class="setting-label">"Enabled"</span>
                    <input
                        type="checkbox"
                        class="setting-checkbox"
                        prop:checked=move || state.mv_enabled.get()
                        on:change=on_toggle_change
                    />
                </div>
                {move || {
                    if state.mv_enabled.get() {
                        view! {
                            <div class="setting-row">
                                <span class="setting-label">"Algorithm"</span>
                                <select
                                    class="setting-select"
                                    on:change=on_display_change
                                    prop:value=move || match state.spectrogram_display.get() {
                                        SpectrogramDisplay::MovementCentroid => "centroid",
                                        SpectrogramDisplay::MovementGradient => "gradient",
                                        SpectrogramDisplay::MovementFlow => "flow",
                                    }
                                >
                                    <option value="flow">"Flow"</option>
                                    <option value="centroid">"Centroid"</option>
                                    <option value="gradient">"Gradient"</option>
                                </select>
                            </div>
                            <div class="setting-row">
                                <span class="setting-label">"Intensity gate"</span>
                                <div class="setting-slider-row">
                                    <input
                                        type="range"
                                        class="setting-range"
                                        min="0"
                                        max="100"
                                        step="1"
                                        prop:value=move || (state.mv_intensity_gate.get() * 100.0).round().to_string()
                                        on:input=on_intensity_gate_change
                                    />
                                    <span class="setting-value">{move || format!("{}%", (state.mv_intensity_gate.get() * 100.0).round() as u32)}</span>
                                </div>
                            </div>
                            <div class="setting-row">
                                <span class="setting-label">"Movement gate"</span>
                                <div class="setting-slider-row">
                                    <input
                                        type="range"
                                        class="setting-range"
                                        min="0"
                                        max="100"
                                        step="1"
                                        prop:value=move || (state.mv_movement_gate.get() * 100.0).round().to_string()
                                        on:input=on_movement_gate_change
                                    />
                                    <span class="setting-value">{move || format!("{}%", (state.mv_movement_gate.get() * 100.0).round() as u32)}</span>
                                </div>
                            </div>
                            <div class="setting-row">
                                <span class="setting-label">"Opacity"</span>
                                <div class="setting-slider-row">
                                    <input
                                        type="range"
                                        class="setting-range"
                                        min="0"
                                        max="100"
                                        step="1"
                                        prop:value=move || (state.mv_opacity.get() * 100.0).to_string()
                                        on:input=on_opacity_change
                                    />
                                    <span class="setting-value">{move || format!("{}%", (state.mv_opacity.get() * 100.0) as u32)}</span>
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }
                }}
            </div>
        </div>
    }
}

#[component]
fn PreviewCanvas(preview: PreviewImage) -> impl IntoView {
    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();
    let pv = preview.clone();

    Effect::new(move || {
        let Some(el) = canvas_ref.get() else { return };
        let canvas: &HtmlCanvasElement = el.as_ref();
        canvas.set_width(pv.width);
        canvas.set_height(pv.height);
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        let clamped = Clamped(pv.pixels.as_slice());
        if let Ok(img) = ImageData::new_with_u8_clamped_array_and_sh(clamped, pv.width, pv.height) {
            let _ = ctx.put_image_data(&img, 0.0, 0.0);
        }
    });

    view! {
        <canvas
            node_ref=canvas_ref
            class="file-preview-canvas"
        />
    }
}

async fn read_and_load_file(file: File, state: AppState) -> Result<(), String> {
    let name = file.name();
    let bytes = read_file_bytes(&file).await?;

    let audio = load_audio(&bytes)?;
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

    // Phase 2: full spectrogram
    let spectrogram = compute_spectrogram(&audio_for_stft, 2048, 512);
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

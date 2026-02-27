use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use crate::state::AppState;
use crate::dsp::notch::{self, NoiseProfile, DetectionConfig};

async fn yield_to_browser() {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        if let Some(w) = web_sys::window() {
            let _ = w.set_timeout_with_callback(&resolve);
        }
    });
    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
}

#[component]
pub(crate) fn NotchPanel() -> impl IntoView {
    let state = expect_context::<AppState>();
    let sensitivity = RwSignal::new(6.0f64); // prominence threshold

    // Detect noise bands
    let on_detect = move |_: web_sys::MouseEvent| {
        let files = state.files.get_untracked();
        let idx = state.current_file_index.get_untracked();
        let Some(file) = idx.and_then(|i| files.get(i).cloned()) else {
            state.show_error_toast("No file loaded");
            return;
        };

        state.notch_detecting.set(true);
        let threshold = sensitivity.get_untracked();
        let samples = file.audio.samples.clone();
        let sample_rate = file.audio.sample_rate;
        let duration = file.audio.duration_secs;

        spawn_local(async move {
            yield_to_browser().await;

            let config = DetectionConfig {
                analysis_duration_secs: if duration > 30.0 { 10.0 } else { duration },
                prominence_threshold: threshold,
                ..DetectionConfig::default()
            };

            let bands = notch::detect_noise_bands_async(&samples, sample_rate, &config).await;
            let count = bands.len();
            state.notch_bands.set(bands);
            if count > 0 {
                state.notch_enabled.set(true);
                state.show_info_toast(format!("Found {} noise band{}", count, if count == 1 { "" } else { "s" }));
            } else {
                state.show_info_toast("No persistent noise bands detected");
            }
            state.notch_detecting.set(false);
        });
    };

    // Toggle individual band
    let toggle_band = move |index: usize| {
        state.notch_bands.update(|bands| {
            if let Some(band) = bands.get_mut(index) {
                band.enabled = !band.enabled;
            }
        });
    };

    // Remove individual band
    let remove_band = move |index: usize| {
        state.notch_bands.update(|bands| {
            if index < bands.len() {
                bands.remove(index);
            }
        });
    };

    // Enable/disable all
    let set_all_enabled = move |enabled: bool| {
        state.notch_bands.update(|bands| {
            for band in bands.iter_mut() {
                band.enabled = enabled;
            }
        });
    };

    // Clear all bands
    let clear_all = move |_: web_sys::MouseEvent| {
        state.notch_bands.set(Vec::new());
        state.notch_enabled.set(false);
    };

    // Export profile
    let on_export = move |_: web_sys::MouseEvent| {
        let bands = state.notch_bands.get_untracked();
        let noise_floor = state.noise_reduce_floor.get_untracked();
        if bands.is_empty() && noise_floor.is_none() {
            state.show_error_toast("Nothing to export");
            return;
        }

        let files = state.files.get_untracked();
        let idx = state.current_file_index.get_untracked();
        let sample_rate = idx
            .and_then(|i| files.get(i))
            .map(|f| f.audio.sample_rate)
            .unwrap_or(0);

        let name = state.notch_profile_name.get_untracked();
        let profile_name = if name.is_empty() { "Noise Profile".to_string() } else { name };

        let created = js_sys::Date::new_0()
            .to_iso_string()
            .as_string()
            .unwrap_or_default();

        let profile = NoiseProfile {
            name: profile_name.clone(),
            bands,
            source_sample_rate: sample_rate,
            created,
            noise_floor,
        };

        let Ok(json) = serde_json::to_string_pretty(&profile) else {
            state.show_error_toast("Failed to serialize profile");
            return;
        };

        // Trigger browser download via JS interop
        let arr = js_sys::Array::of1(&JsValue::from_str(&json));
        let Ok(blob) = web_sys::Blob::new_with_str_sequence(&arr) else {
            return;
        };
        let Ok(url) = web_sys::Url::create_object_url_with_blob(&blob) else {
            return;
        };

        let doc = web_sys::window().unwrap().document().unwrap();
        let a: web_sys::HtmlAnchorElement = doc
            .create_element("a")
            .unwrap()
            .unchecked_into();
        a.set_href(&url);
        let filename = format!("{}.json", profile_name.replace(' ', "_").to_lowercase());
        a.set_download(&filename);
        a.click();
        let _ = web_sys::Url::revoke_object_url(&url);
    };

    // Import profile
    let on_import = move |_: web_sys::MouseEvent| {
        let doc = web_sys::window().unwrap().document().unwrap();
        let input: web_sys::HtmlInputElement = doc
            .create_element("input")
            .unwrap()
            .unchecked_into();
        input.set_type("file");
        input.set_attribute("accept", ".json").unwrap();

        let on_change = Closure::<dyn FnMut(web_sys::Event)>::new(move |ev: web_sys::Event| {
            let target: web_sys::HtmlInputElement = ev.target().unwrap().unchecked_into();
            let Some(file_list) = target.files() else { return };
            let Some(file) = file_list.get(0) else { return };

            let reader = web_sys::FileReader::new().unwrap();
            let reader_clone = reader.clone();
            let on_load = Closure::<dyn FnMut(web_sys::Event)>::new(move |_: web_sys::Event| {
                let result = reader_clone.result().unwrap();
                let text = result.as_string().unwrap_or_default();
                match serde_json::from_str::<NoiseProfile>(&text) {
                    Ok(profile) => {
                        // Check for bands above Nyquist and disable them
                        let files = state.files.get_untracked();
                        let idx = state.current_file_index.get_untracked();
                        let nyquist = idx
                            .and_then(|i| files.get(i))
                            .map(|f| f.audio.sample_rate as f64 / 2.0)
                            .unwrap_or(f64::MAX);

                        let mut bands = profile.bands;
                        for band in bands.iter_mut() {
                            if band.center_hz >= nyquist {
                                band.enabled = false;
                            }
                        }

                        let count = bands.len();
                        let has_floor = profile.noise_floor.is_some();
                        state.notch_bands.set(bands);
                        state.notch_profile_name.set(profile.name);
                        if count > 0 {
                            state.notch_enabled.set(true);
                        }

                        // Restore spectral subtraction noise floor if present
                        if let Some(floor) = profile.noise_floor {
                            state.noise_reduce_floor.set(Some(floor));
                            state.noise_reduce_enabled.set(true);
                        }

                        let msg = match (count > 0, has_floor) {
                            (true, true) => format!("Imported {} band{} + noise floor", count, if count == 1 { "" } else { "s" }),
                            (true, false) => format!("Imported {} band{}", count, if count == 1 { "" } else { "s" }),
                            (false, true) => "Imported noise floor".to_string(),
                            (false, false) => "Profile was empty".to_string(),
                        };
                        state.show_info_toast(msg);
                    }
                    Err(e) => {
                        state.show_error_toast(format!("Invalid profile: {e}"));
                    }
                }
            });
            reader.set_onload(Some(on_load.as_ref().unchecked_ref()));
            on_load.forget();
            let _ = reader.read_as_text(&file);
        });
        input.set_onchange(Some(on_change.as_ref().unchecked_ref()));
        on_change.forget();
        input.click();
    };

    // Learn noise floor for spectral subtraction
    let on_learn_floor = move |_: web_sys::MouseEvent| {
        let files = state.files.get_untracked();
        let idx = state.current_file_index.get_untracked();
        let Some(file) = idx.and_then(|i| files.get(i).cloned()) else {
            state.show_error_toast("No file loaded");
            return;
        };

        state.noise_reduce_learning.set(true);
        let samples = file.audio.samples.clone();
        let sample_rate = file.audio.sample_rate;
        let duration = file.audio.duration_secs;

        spawn_local(async move {
            yield_to_browser().await;
            let analysis_secs = if duration > 30.0 { 10.0 } else { duration };
            let floor = crate::dsp::spectral_sub::learn_noise_floor_async(
                &samples, sample_rate, analysis_secs,
            ).await;
            if let Some(f) = floor {
                state.noise_reduce_floor.set(Some(f));
                state.noise_reduce_enabled.set(true);
                state.show_info_toast("Noise floor learned");
            } else {
                state.show_error_toast("Not enough audio to learn noise floor");
            }
            state.noise_reduce_learning.set(false);
        });
    };

    // Noise reduction strength slider handler
    let on_strength_change = move |ev: web_sys::Event| {
        let target: web_sys::HtmlInputElement = ev.target().unwrap().unchecked_into();
        if let Ok(val) = target.value().parse::<f64>() {
            state.noise_reduce_strength.set(val / 100.0); // slider 0–300 → 0.0–3.0
        }
    };

    // Sensitivity slider handler
    let on_sensitivity_change = move |ev: web_sys::Event| {
        let target: web_sys::HtmlInputElement = ev.target().unwrap().unchecked_into();
        if let Ok(val) = target.value().parse::<f64>() {
            sensitivity.set((150.0 - val) / 10.0); // slider 30–120 → threshold 12.0–3.0 (inverted: higher sensitivity = lower threshold = more bands)
        }
    };

    // Profile name handler
    let on_name_change = move |ev: web_sys::Event| {
        let target: web_sys::HtmlInputElement = ev.target().unwrap().unchecked_into();
        state.notch_profile_name.set(target.value());
    };

    view! {
        <div class="sidebar-panel notch-panel">
            // Master toggle
            <div class="setting-group">
                <div class="setting-row">
                    <label class="setting-label" style="flex: 1; cursor: pointer;">
                        <input
                            type="checkbox"
                            prop:checked=move || state.notch_enabled.get()
                            on:change=move |ev: web_sys::Event| {
                                let target: web_sys::HtmlInputElement = ev.target().unwrap().unchecked_into();
                                state.notch_enabled.set(target.checked());
                            }
                        />
                        " Notch Filter"
                    </label>
                </div>
            </div>

            // Detection
            <div class="setting-group">
                <div class="setting-group-title">"Detection"</div>
                <div class="setting-row" style="gap: 4px;">
                    <button
                        class="sidebar-btn"
                        style="flex: 1;"
                        on:click=on_detect
                        disabled=move || state.notch_detecting.get() || state.current_file_index.get().is_none()
                    >
                        {move || if state.notch_detecting.get() {
                            "Detecting..."
                        } else {
                            "Detect Noise"
                        }}
                    </button>
                </div>
                <div class="setting-row">
                    <span class="setting-label">"Sensitivity"</span>
                    <input
                        type="range"
                        class="setting-slider"
                        min="30"
                        max="120"
                        step="5"
                        prop:value=move || (150.0 - sensitivity.get() * 10.0) as i32
                        on:input=on_sensitivity_change
                        title=move || format!("Threshold: {:.1}x ({:.0} dB)", sensitivity.get(), 20.0 * sensitivity.get().log10())
                    />
                </div>
            </div>

            // Band list
            <div class="setting-group">
                <div class="setting-group-title">
                    {move || {
                        let bands = state.notch_bands.get();
                        let enabled = bands.iter().filter(|b| b.enabled).count();
                        if bands.is_empty() {
                            "Bands".to_string()
                        } else {
                            format!("Bands ({}/{})", enabled, bands.len())
                        }
                    }}
                </div>
                {move || {
                    let bands = state.notch_bands.get();
                    if bands.is_empty() {
                        view! {
                            <div class="setting-row" style="opacity: 0.5; font-size: 11px;">
                                "No bands detected yet"
                            </div>
                        }.into_any()
                    } else {
                        let items: Vec<_> = bands.iter().enumerate().map(|(i, band)| {
                            let center = band.center_hz;
                            let strength = band.strength_db;
                            let enabled = band.enabled;
                            let bandwidth = band.bandwidth_hz;
                            view! {
                                <div class="notch-band-row" style="display: flex; align-items: center; gap: 4px; padding: 2px 0; font-size: 11px;">
                                    <input
                                        type="checkbox"
                                        checked=enabled
                                        on:change=move |_| toggle_band(i)
                                        style="margin: 0;"
                                    />
                                    <span style="flex: 1; white-space: nowrap;">
                                        {if center >= 1000.0 {
                                            format!("{:.1} kHz", center / 1000.0)
                                        } else {
                                            format!("{:.0} Hz", center)
                                        }}
                                    </span>
                                    <span style="opacity: 0.6; font-size: 10px; white-space: nowrap;" title=format!("BW: {:.0} Hz", bandwidth)>
                                        {format!("+{:.0}dB", strength)}
                                    </span>
                                    <button
                                        class="notch-remove-btn"
                                        style="background: none; border: none; color: inherit; opacity: 0.4; cursor: pointer; padding: 0 2px; font-size: 12px;"
                                        on:click=move |_: web_sys::MouseEvent| remove_band(i)
                                        title="Remove band"
                                    >
                                        {"\u{00D7}"}
                                    </button>
                                </div>
                            }
                        }).collect();
                        view! {
                            <div class="notch-band-list" style="max-height: 200px; overflow-y: auto;">
                                {items}
                            </div>
                        }.into_any()
                    }
                }}
                {move || {
                    let bands = state.notch_bands.get();
                    if bands.is_empty() {
                        view! { <span></span> }.into_any()
                    } else {
                        view! {
                            <div class="setting-row" style="gap: 4px; margin-top: 4px;">
                                <button
                                    class="sidebar-btn"
                                    style="flex: 1; font-size: 10px;"
                                    on:click=move |_: web_sys::MouseEvent| set_all_enabled(true)
                                >
                                    "All On"
                                </button>
                                <button
                                    class="sidebar-btn"
                                    style="flex: 1; font-size: 10px;"
                                    on:click=move |_: web_sys::MouseEvent| set_all_enabled(false)
                                >
                                    "All Off"
                                </button>
                                <button
                                    class="sidebar-btn"
                                    style="flex: 1; font-size: 10px;"
                                    on:click=clear_all
                                >
                                    "Clear"
                                </button>
                            </div>
                        }.into_any()
                    }
                }}
            </div>

            // Profile management
            <div class="setting-group">
                <div class="setting-group-title">"Profile"</div>
                <div class="setting-row">
                    <input
                        type="text"
                        class="setting-input"
                        style="flex: 1; font-size: 11px; padding: 2px 4px; background: var(--bg-secondary, #333); color: inherit; border: 1px solid #555; border-radius: 3px;"
                        placeholder="Profile name"
                        prop:value=move || state.notch_profile_name.get()
                        on:input=on_name_change
                    />
                </div>
                <div class="setting-row" style="gap: 4px;">
                    <button
                        class="sidebar-btn"
                        style="flex: 1;"
                        on:click=on_export
                        disabled=move || state.notch_bands.get().is_empty() && state.noise_reduce_floor.get().is_none()
                    >
                        "Export"
                    </button>
                    <button
                        class="sidebar-btn"
                        style="flex: 1;"
                        on:click=on_import
                    >
                        "Import"
                    </button>
                </div>
            </div>

            // Spectral subtraction noise reduction
            <div class="setting-group">
                <div class="setting-row">
                    <label class="setting-label" style="flex: 1; cursor: pointer;">
                        <input
                            type="checkbox"
                            prop:checked=move || state.noise_reduce_enabled.get()
                            on:change=move |ev: web_sys::Event| {
                                let target: web_sys::HtmlInputElement = ev.target().unwrap().unchecked_into();
                                state.noise_reduce_enabled.set(target.checked());
                            }
                        />
                        " Noise Reduction"
                    </label>
                </div>
                <div class="setting-row" style="gap: 4px;">
                    <button
                        class="sidebar-btn"
                        style="flex: 1;"
                        on:click=on_learn_floor
                        disabled=move || state.noise_reduce_learning.get() || state.current_file_index.get().is_none()
                    >
                        {move || if state.noise_reduce_learning.get() {
                            "Learning..."
                        } else {
                            "Learn Noise Floor"
                        }}
                    </button>
                </div>
                <div class="setting-row">
                    <span class="setting-label">"Strength"</span>
                    <input
                        type="range"
                        class="setting-slider"
                        min="0"
                        max="300"
                        step="5"
                        prop:value=move || (state.noise_reduce_strength.get() * 100.0) as i32
                        on:input=on_strength_change
                        title=move || format!("{:.0}%", state.noise_reduce_strength.get() * 100.0)
                    />
                </div>
                {move || {
                    let floor = state.noise_reduce_floor.get();
                    if let Some(f) = floor {
                        view! {
                            <div class="setting-row" style="font-size: 11px; opacity: 0.7;">
                                {format!("{} bins, {:.1}s analyzed", f.bin_magnitudes.len(), f.analysis_duration_secs)}
                            </div>
                            <div class="setting-row" style="gap: 4px; margin-top: 2px;">
                                <button
                                    class="sidebar-btn"
                                    style="flex: 1; font-size: 10px;"
                                    on:click=move |_: web_sys::MouseEvent| {
                                        state.noise_reduce_floor.set(None);
                                        state.noise_reduce_enabled.set(false);
                                    }
                                >
                                    "Clear Floor"
                                </button>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="setting-row" style="opacity: 0.5; font-size: 11px;">
                                "No noise floor learned"
                            </div>
                        }.into_any()
                    }
                }}
            </div>
        </div>
    }
}

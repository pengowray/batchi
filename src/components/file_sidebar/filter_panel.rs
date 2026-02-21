use leptos::prelude::*;
use wasm_bindgen::JsCast;
use crate::state::{AppState, PlaybackMode};
use crate::audio::playback;

#[component]
pub(super) fn FilterPanel() -> impl IntoView {
    let state = expect_context::<AppState>();

    // Replay ZC audio when EQ settings change during playback
    let maybe_replay_zc = move || {
        if state.is_playing.get_untracked()
            && state.playback_mode.get_untracked() == PlaybackMode::ZeroCrossing
            && state.filter_enabled.get_untracked()
        {
            playback::replay(&state);
        }
    };

    let on_enable_change = move |ev: web_sys::Event| {
        let target = ev.target().unwrap();
        let input: web_sys::HtmlInputElement = target.unchecked_into();
        state.filter_enabled.set(input.checked());
        maybe_replay_zc();
    };

    let set_band_mode = move |mode: u8| {
        state.filter_band_mode.set(mode);
        maybe_replay_zc();
    };

    let on_set_from_selection = move |_: web_sys::MouseEvent| {
        if let Some(sel) = state.selection.get_untracked() {
            if sel.freq_low > 0.0 && sel.freq_high > sel.freq_low {
                state.filter_freq_low.set(sel.freq_low);
                state.filter_freq_high.set(sel.freq_high);
                state.filter_set_from_selection.set(true);
                maybe_replay_zc();
            }
        }
    };

    let make_db_handler = |signal: RwSignal<f64>| {
        move |ev: web_sys::Event| {
            let target = ev.target().unwrap();
            let input: web_sys::HtmlInputElement = target.unchecked_into();
            if let Ok(val) = input.value().parse::<f64>() {
                signal.set(val);
                maybe_replay_zc();
            }
        }
    };

    let on_below_change = make_db_handler(state.filter_db_below);
    let on_selected_change = make_db_handler(state.filter_db_selected);
    let on_harmonics_change = make_db_handler(state.filter_db_harmonics);
    let on_above_change = make_db_handler(state.filter_db_above);

    let hover_signal = state.filter_hovering_band;
    let make_hover_enter = move |band: u8| {
        move |_: web_sys::MouseEvent| {
            hover_signal.set(Some(band));
        }
    };
    let on_hover_leave = move |_: web_sys::MouseEvent| {
        hover_signal.set(None);
    };

    let has_selection = move || state.selection.get().is_some();
    let band_mode = move || state.filter_band_mode.get();
    let show_harmonics = move || band_mode() >= 4;
    let show_above = move || band_mode() >= 3;

    let quality = move || state.filter_quality.get();
    let set_quality = move |q: crate::state::FilterQuality| {
        state.filter_quality.set(q);
        maybe_replay_zc();
    };

    let on_het_cutoff_change = move |ev: web_sys::Event| {
        let target = ev.target().unwrap();
        let input: web_sys::HtmlInputElement = target.unchecked_into();
        if let Ok(val) = input.value().parse::<f64>() {
            state.het_cutoff.set(val * 1000.0);
        }
    };

    view! {
        <div class="sidebar-panel filter-panel">
            <div class="setting-group">
                <div class="setting-row">
                    <span class="setting-label">"Enable pre-processing"</span>
                    <input
                        type="checkbox"
                        class="setting-checkbox"
                        prop:checked=move || state.filter_enabled.get()
                        on:change=on_enable_change
                    />
                </div>
            </div>

            {move || {
                if !state.filter_enabled.get() {
                    return view! { <span></span> }.into_any();
                }

                view! {
                    <div class="setting-group">
                        <div class="setting-group-title">"Bands"</div>
                        <div class="filter-band-mode">
                            <button
                                class=move || if band_mode() == 3 { "mode-btn active" } else { "mode-btn" }
                                on:click=move |_| set_band_mode(3)
                            >"3"</button>
                            <button
                                class=move || if band_mode() == 4 { "mode-btn active" } else { "mode-btn" }
                                on:click=move |_| set_band_mode(4)
                            >"4"</button>
                        </div>
                    </div>

                    <div class="setting-group">
                        <div class="setting-group-title">"Quality"</div>
                        <div class="filter-band-mode">
                            <button
                                class=move || if quality() == crate::state::FilterQuality::Fast { "mode-btn active" } else { "mode-btn" }
                                on:click=move |_| set_quality(crate::state::FilterQuality::Fast)
                                title="IIR band-split — low latency, softer band edges"
                            >"Fast"</button>
                            <button
                                class=move || if quality() == crate::state::FilterQuality::HQ { "mode-btn active" } else { "mode-btn" }
                                on:click=move |_| set_quality(crate::state::FilterQuality::HQ)
                                title="FFT spectral EQ — sharp band edges, higher latency"
                            >"HQ"</button>
                        </div>
                    </div>

                    <div class="setting-group">
                        <button
                            class="filter-set-btn"
                            on:click=on_set_from_selection
                            disabled=move || !has_selection()
                            title="Set frequency range from current spectrogram selection"
                        >
                            "Set from selection"
                        </button>
                        <div class="filter-freq-display">
                            {move || format!("{:.0} – {:.0} kHz",
                                state.filter_freq_low.get() / 1000.0,
                                state.filter_freq_high.get() / 1000.0
                            )}
                        </div>
                    </div>

                    <div class="setting-group">
                        <div class="setting-group-title">"Pre-processing EQ"</div>

                        // Above slider (3+ band) — top, highest freq
                        {move || {
                            if show_above() {
                                view! {
                                    <div class="setting-row"
                                        on:mouseenter=make_hover_enter(3)
                                        on:mouseleave=on_hover_leave
                                    >
                                        <span class="setting-label">"Above"</span>
                                        <div class="setting-slider-row">
                                            <input
                                                type="range"
                                                class="setting-range"
                                                min="-60"
                                                max="6"
                                                step="1"
                                                prop:value=move || state.filter_db_above.get().to_string()
                                                on:input=on_above_change
                                            />
                                            <span class="setting-value">{move || format!("{:.0} dB", state.filter_db_above.get())}</span>
                                        </div>
                                    </div>
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }
                        }}

                        // Harmonics slider (4-band only, selection < 1 octave)
                        {move || {
                            if show_harmonics() {
                                view! {
                                    <div class="setting-row"
                                        on:mouseenter=make_hover_enter(2)
                                        on:mouseleave=on_hover_leave
                                    >
                                        <span class="setting-label">"Harmonics"</span>
                                        <div class="setting-slider-row">
                                            <input
                                                type="range"
                                                class="setting-range"
                                                min="-60"
                                                max="6"
                                                step="1"
                                                prop:value=move || state.filter_db_harmonics.get().to_string()
                                                on:input=on_harmonics_change
                                            />
                                            <span class="setting-value">{move || format!("{:.0} dB", state.filter_db_harmonics.get())}</span>
                                        </div>
                                    </div>
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }
                        }}

                        // Selected slider
                        <div class="setting-row"
                            on:mouseenter=make_hover_enter(1)
                            on:mouseleave=on_hover_leave
                        >
                            <span class="setting-label">"Selected"</span>
                            <div class="setting-slider-row">
                                <input
                                    type="range"
                                    class="setting-range"
                                    min="-60"
                                    max="6"
                                    step="1"
                                    prop:value=move || state.filter_db_selected.get().to_string()
                                    on:input=on_selected_change
                                />
                                <span class="setting-value">{move || format!("{:.0} dB", state.filter_db_selected.get())}</span>
                            </div>
                        </div>

                        // Below slider — bottom, lowest freq
                        <div class="setting-row"
                            on:mouseenter=make_hover_enter(0)
                            on:mouseleave=on_hover_leave
                        >
                            <span class="setting-label">"Below"</span>
                            <div class="setting-slider-row">
                                <input
                                    type="range"
                                    class="setting-range"
                                    min="-60"
                                    max="6"
                                    step="1"
                                    prop:value=move || state.filter_db_below.get().to_string()
                                    on:input=on_below_change
                                />
                                <span class="setting-value">{move || format!("{:.0} dB", state.filter_db_below.get())}</span>
                            </div>
                        </div>
                    </div>

                }.into_any()
            }}

            // Mode-specific filter chain (always visible, not gated by EQ enable)
            {move || {
                let mode = state.playback_mode.get();
                match mode {
                    crate::state::PlaybackMode::Heterodyne => {
                        view! {
                            <div class="setting-group">
                                <div class="setting-group-title">"Mode filters"</div>
                                <div class="setting-row"
                                    on:mouseenter=move |_| state.het_interacting.set(true)
                                    on:mouseleave=move |_| state.het_interacting.set(false)
                                >
                                    <span class="setting-label">"HET LP"</span>
                                    <div class="setting-slider-row">
                                        <input
                                            type="range"
                                            class="setting-range"
                                            min="1"
                                            max="30"
                                            step="1"
                                            prop:value=move || (state.het_cutoff.get() / 1000.0).to_string()
                                            on:input=on_het_cutoff_change
                                        />
                                        <span class="setting-value">{move || format!("{:.0} kHz", state.het_cutoff.get() / 1000.0)}</span>
                                    </div>
                                </div>
                            </div>
                        }.into_any()
                    }
                    crate::state::PlaybackMode::ZeroCrossing => {
                        let filter_on = state.filter_enabled.get();
                        view! {
                            <div class="setting-group">
                                <div class="setting-group-title">"Mode filters"</div>
                                <div class="filter-mode-info">
                                    {if filter_on {
                                        "ZC: using pre-processing EQ"
                                    } else {
                                        "ZC: bandpass 15\u{2013}150 kHz"
                                    }}
                                </div>
                            </div>
                        }.into_any()
                    }
                    _ => view! { <span></span> }.into_any(),
                }
            }}
        </div>
    }
}

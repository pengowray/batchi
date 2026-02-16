use leptos::prelude::*;
use wasm_bindgen::JsCast;
use crate::state::{AppState, PlaybackMode};
use crate::audio::playback;

#[component]
pub fn Toolbar() -> impl IntoView {
    let state = expect_context::<AppState>();

    let on_play = move |_| {
        let files = state.files.get_untracked();
        let idx = state.current_file_index.get_untracked();
        let Some(file) = idx.and_then(|i| files.get(i)) else { return };

        let mode = state.playback_mode.get_untracked();
        let selection = state.selection.get_untracked();
        let het_freq = state.het_frequency.get_untracked();

        match mode {
            PlaybackMode::Normal => {
                playback::play_normal(&file.audio, selection);
            }
            PlaybackMode::Heterodyne => {
                playback::play_heterodyne(&file.audio, het_freq, selection);
            }
        }
        state.is_playing.set(true);
    };

    let on_stop = move |_| {
        playback::stop();
        state.is_playing.set(false);
    };

    let is_het = move || state.playback_mode.get() == PlaybackMode::Heterodyne;

    let toggle_mode = move |_| {
        state.playback_mode.update(|m| {
            *m = match m {
                PlaybackMode::Normal => PlaybackMode::Heterodyne,
                PlaybackMode::Heterodyne => PlaybackMode::Normal,
            };
        });
    };

    let het_freq_display = move || {
        format!("{:.0} kHz", state.het_frequency.get() / 1000.0)
    };

    let on_het_freq_change = move |ev: web_sys::Event| {
        let target = ev.target().unwrap();
        let input: web_sys::HtmlInputElement = target.unchecked_into();
        if let Ok(val) = input.value().parse::<f64>() {
            state.het_frequency.set(val * 1000.0);
        }
    };

    let has_file = move || state.current_file_index.get().is_some();
    let is_playing = move || state.is_playing.get();

    view! {
        <div class="toolbar">
            <span style="color: #eee; font-weight: 600">"Batgram"</span>
            <div class="toolbar-sep"></div>
            <button
                on:click=move |ev| { if is_playing() { on_stop(ev) } else { on_play(ev) } }
                disabled=move || !has_file()
            >
                {move || if is_playing() { "Stop" } else { "Play" }}
            </button>
            <button
                class=move || if is_het() { "active" } else { "" }
                on:click=toggle_mode
            >
                {move || if is_het() { "HET" } else { "Normal" }}
            </button>
            {move || {
                if is_het() {
                    view! {
                        <label>
                            {het_freq_display}
                            <input
                                type="range"
                                min="10"
                                max="150"
                                step="1"
                                prop:value=move || (state.het_frequency.get() / 1000.0).to_string()
                                on:input=on_het_freq_change
                            />
                        </label>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}
        </div>
    }
}

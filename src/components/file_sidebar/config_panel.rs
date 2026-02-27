use leptos::prelude::*;
use wasm_bindgen::JsCast;
use crate::state::{AppState, ColormapPreference};

#[component]
pub(super) fn ConfigPanel() -> impl IntoView {
    let state = expect_context::<AppState>();

    let on_auto_advance = move |ev: web_sys::Event| {
        let target = ev.target().unwrap();
        let input: web_sys::HtmlInputElement = target.unchecked_into();
        state.auto_advance.set(input.checked());
    };

    let on_join_files = move |ev: web_sys::Event| {
        let target = ev.target().unwrap();
        let input: web_sys::HtmlInputElement = target.unchecked_into();
        state.join_files.set(input.checked());
    };

    let on_follow_cursor = move |ev: web_sys::Event| {
        let target = ev.target().unwrap();
        let input: web_sys::HtmlInputElement = target.unchecked_into();
        state.follow_cursor.set(input.checked());
    };

    let on_colormap_change = move |ev: web_sys::Event| {
        let target = ev.target().unwrap();
        let select: web_sys::HtmlSelectElement = target.unchecked_into();
        let pref = match select.value().as_str() {
            "inferno" => ColormapPreference::Inferno,
            "greyscale" => ColormapPreference::Greyscale,
            _ => ColormapPreference::Viridis,
        };
        state.colormap_preference.set(pref);
        // Trigger spectrogram redraw
        state.tile_ready_signal.update(|n| *n = n.wrapping_add(1));
    };

    view! {
        <div class="sidebar-panel">
            <div class="setting-group">
                <div class="setting-group-title">"Playback"</div>
                <div class="setting-row">
                    <span class="setting-label">"Auto-advance"</span>
                    <input
                        type="checkbox"
                        class="setting-checkbox"
                        prop:checked=move || state.auto_advance.get()
                        on:change=on_auto_advance
                    />
                </div>
                <div class="setting-row">
                    <span class="setting-label">"Join files"</span>
                    <input
                        type="checkbox"
                        class="setting-checkbox"
                        prop:checked=move || state.join_files.get()
                        on:change=on_join_files
                    />
                </div>
                <div class="setting-row">
                    <span class="setting-label">"Follow cursor"</span>
                    <input
                        type="checkbox"
                        class="setting-checkbox"
                        prop:checked=move || state.follow_cursor.get()
                        on:change=on_follow_cursor
                    />
                </div>
            </div>

            <div class="setting-group">
                <div class="setting-group-title">"Display"</div>
                <div class="setting-row">
                    <span class="setting-label">"Color scheme"</span>
                    <select
                        class="setting-select"
                        on:change=on_colormap_change
                        prop:value=move || match state.colormap_preference.get() {
                            ColormapPreference::Viridis => "viridis",
                            ColormapPreference::Inferno => "inferno",
                            ColormapPreference::Greyscale => "greyscale",
                        }
                    >
                        <option value="viridis">"Viridis"</option>
                        <option value="inferno">"Inferno"</option>
                        <option value="greyscale">"Greyscale"</option>
                    </select>
                </div>
            </div>
        </div>
    }
}

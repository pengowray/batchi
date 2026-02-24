use leptos::prelude::*;
use leptos::task::spawn_local;
use wasm_bindgen::prelude::*;
use crate::state::AppState;
use crate::tauri_bridge::tauri_invoke;

const XC_GROUPS: &[&str] = &["bats", "birds", "frogs", "grasshoppers", "land mammals"];

// ── Helper to call tauri_invoke with a JS object of args ─────────────

async fn invoke_with(cmd: &str, args: &js_sys::Object) -> Result<JsValue, String> {
    tauri_invoke(cmd, &args.into()).await
}

fn js_obj() -> js_sys::Object {
    js_sys::Object::new()
}

fn set_str(obj: &js_sys::Object, key: &str, val: &str) {
    js_sys::Reflect::set(obj, &JsValue::from_str(key), &JsValue::from_str(val)).ok();
}

fn set_u64(obj: &js_sys::Object, key: &str, val: u64) {
    js_sys::Reflect::set(obj, &JsValue::from_str(key), &JsValue::from_f64(val as f64)).ok();
}

fn set_opt_str(obj: &js_sys::Object, key: &str, val: &Option<String>) {
    match val {
        Some(v) => { set_str(obj, key, v); }
        None => { js_sys::Reflect::set(obj, &JsValue::from_str(key), &JsValue::NULL).ok(); }
    }
}

fn set_opt_u32(obj: &js_sys::Object, key: &str, val: Option<u32>) {
    match val {
        Some(v) => { js_sys::Reflect::set(obj, &JsValue::from_str(key), &JsValue::from_f64(v as f64)).ok(); }
        None => { js_sys::Reflect::set(obj, &JsValue::from_str(key), &JsValue::NULL).ok(); }
    }
}

// ── Data types (mirror Tauri response shapes) ────────────────────────

#[derive(Clone, Debug)]
struct SpeciesInfo {
    gen: String,
    sp: String,
    en: String,
    fam: String,
    recording_count: u32,
}

#[derive(Clone, Debug)]
struct RecordingInfo {
    id: u64,
    en: String,
    gen: String,
    sp: String,
    q: String,
    length: String,
    cnt: String,
    loc: String,
    rec: String,
    date: String,
    sound_type: String,
    smp: String,
}

#[derive(Clone, Debug)]
struct CachedFile {
    path: String,
    filename: String,
    xc_id: u64,
    metadata: Vec<(String, String)>,
}

// ── Parse helpers ────────────────────────────────────────────────────

fn parse_species_list(val: &JsValue) -> Vec<SpeciesInfo> {
    let species_arr = js_sys::Reflect::get(val, &"species".into())
        .ok()
        .and_then(|v| js_sys::Array::try_from(v).ok());
    let Some(arr) = species_arr else { return Vec::new() };
    let mut result = Vec::new();
    for i in 0..arr.length() {
        let item = arr.get(i);
        let s = |k: &str| js_sys::Reflect::get(&item, &k.into())
            .ok()
            .and_then(|v| v.as_string())
            .unwrap_or_default();
        let n = |k: &str| js_sys::Reflect::get(&item, &k.into())
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0) as u32;
        result.push(SpeciesInfo {
            gen: s("gen"),
            sp: s("sp"),
            en: s("en"),
            fam: s("fam"),
            recording_count: n("recording_count"),
        });
    }
    result
}

fn parse_recordings(val: &JsValue) -> Vec<RecordingInfo> {
    let rec_arr = js_sys::Reflect::get(val, &"recordings".into())
        .ok()
        .and_then(|v| js_sys::Array::try_from(v).ok());
    let Some(arr) = rec_arr else { return Vec::new() };
    let mut result = Vec::new();
    for i in 0..arr.length() {
        let item = arr.get(i);
        let s = |k: &str| js_sys::Reflect::get(&item, &k.into())
            .ok()
            .and_then(|v| v.as_string())
            .unwrap_or_default();
        let id = js_sys::Reflect::get(&item, &"id".into())
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0) as u64;
        result.push(RecordingInfo {
            id,
            en: s("en"),
            gen: s("gen"),
            sp: s("sp"),
            q: s("q"),
            length: s("length"),
            cnt: s("cnt"),
            loc: s("loc"),
            rec: s("rec"),
            date: s("date"),
            sound_type: s("sound_type"),
            smp: s("smp"),
        });
    }
    result
}

fn parse_num_pages(val: &JsValue) -> u32 {
    js_sys::Reflect::get(val, &"num_pages".into())
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(1.0) as u32
}

fn parse_current_page(val: &JsValue) -> u32 {
    js_sys::Reflect::get(val, &"page".into())
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(1.0) as u32
}

fn parse_num_recordings(val: &JsValue) -> u32 {
    js_sys::Reflect::get(val, &"num_recordings".into())
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0) as u32
}

fn format_sample_rate(smp: &str) -> String {
    match smp.parse::<u64>() {
        Ok(hz) if hz >= 1000 => format!("{}kHz", hz / 1000),
        Ok(hz) => format!("{hz}Hz"),
        Err(_) => smp.to_string(),
    }
}

fn parse_cached_file(val: &JsValue) -> Option<CachedFile> {
    let path = js_sys::Reflect::get(val, &"path".into()).ok()?.as_string()?;
    let filename = js_sys::Reflect::get(val, &"filename".into()).ok()?.as_string()?;
    let xc_id = js_sys::Reflect::get(val, &"xc_id".into()).ok()?.as_f64()? as u64;
    let meta_arr = js_sys::Reflect::get(val, &"metadata".into())
        .ok()
        .and_then(|v| js_sys::Array::try_from(v).ok());
    let metadata = meta_arr.map(|arr| {
        let mut pairs = Vec::new();
        for i in 0..arr.length() {
            let pair = arr.get(i);
            let pair_arr = js_sys::Array::from(&pair);
            if pair_arr.length() >= 2 {
                let k = pair_arr.get(0).as_string().unwrap_or_default();
                let v = pair_arr.get(1).as_string().unwrap_or_default();
                pairs.push((k, v));
            }
        }
        pairs
    }).unwrap_or_default();
    Some(CachedFile { path, filename, xc_id, metadata })
}

// ── View states ──────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
enum BrowserView {
    ApiKeyPrompt,
    GroupBrowse,
    SpeciesRecordings { genus: String, species: String, en: String },
    SearchResults,
}

// ── Component ────────────────────────────────────────────────────────

#[component]
pub fn XcBrowser() -> impl IntoView {
    let state = expect_context::<AppState>();

    let view = RwSignal::new(BrowserView::ApiKeyPrompt);
    let api_key_input = RwSignal::new(String::new());
    let has_key = RwSignal::new(false);
    let selected_group = RwSignal::new("bats".to_string());
    let country_input = RwSignal::new(String::new());
    let species_list: RwSignal<Vec<SpeciesInfo>> = RwSignal::new(Vec::new());
    let recordings: RwSignal<Vec<RecordingInfo>> = RwSignal::new(Vec::new());
    let loading = RwSignal::new(false);
    let error_msg: RwSignal<Option<String>> = RwSignal::new(None);
    let taxonomy_age: RwSignal<Option<String>> = RwSignal::new(None);
    let search_input = RwSignal::new(String::new());
    let recordings_page = RwSignal::new(1u32);
    let recordings_total_pages = RwSignal::new(1u32);
    let downloading: RwSignal<Option<u64>> = RwSignal::new(None);
    let recordings_total: RwSignal<u32> = RwSignal::new(0);
    let cached_ids: RwSignal<std::collections::HashSet<u64>> = RwSignal::new(std::collections::HashSet::new());

    // Check if API key is already set
    spawn_local(async move {
        match crate::tauri_bridge::tauri_invoke_no_args("xc_get_api_key").await {
            Ok(val) => {
                if val.is_string() && !val.as_string().unwrap_or_default().is_empty() {
                    has_key.set(true);
                    view.set(BrowserView::GroupBrowse);
                }
            }
            Err(_) => {}
        }
    });

    let on_close = move |_: web_sys::MouseEvent| {
        state.xc_browser_open.set(false);
    };

    // Prevent click on modal content from closing it
    let on_content_click = move |ev: web_sys::MouseEvent| {
        ev.stop_propagation();
    };

    let on_save_key = move |_: web_sys::MouseEvent| {
        let key = api_key_input.get_untracked().trim().to_string();
        if key.is_empty() {
            return;
        }
        spawn_local(async move {
            let args = js_obj();
            set_str(&args, "key", &key);
            match invoke_with("xc_set_api_key", &args).await {
                Ok(_) => {
                    has_key.set(true);
                    view.set(BrowserView::GroupBrowse);
                    error_msg.set(None);
                }
                Err(e) => error_msg.set(Some(format!("Failed to save key: {e}"))),
            }
        });
    };

    let load_group = move || {
        let group = selected_group.get_untracked();
        let country = {
            let c = country_input.get_untracked().trim().to_string();
            if c.is_empty() { None } else { Some(c) }
        };
        loading.set(true);
        error_msg.set(None);
        species_list.set(Vec::new());

        spawn_local(async move {
            let args = js_obj();
            set_str(&args, "group", &group);
            set_opt_str(&args, "country", &country);

            match invoke_with("xc_browse_group", &args).await {
                Ok(val) => {
                    species_list.set(parse_species_list(&val));
                }
                Err(e) => error_msg.set(Some(e)),
            }

            // Get cache age
            let age_args = js_obj();
            set_str(&age_args, "group", &group);
            set_opt_str(&age_args, "country", &country);
            if let Ok(val) = invoke_with("xc_taxonomy_age", &age_args).await {
                taxonomy_age.set(val.as_string());
            }

            loading.set(false);
        });
    };

    let on_load_group = move |_: web_sys::MouseEvent| {
        load_group();
    };

    let on_refresh = move |_: web_sys::MouseEvent| {
        let group = selected_group.get_untracked();
        let country = {
            let c = country_input.get_untracked().trim().to_string();
            if c.is_empty() { None } else { Some(c) }
        };
        loading.set(true);
        error_msg.set(None);

        spawn_local(async move {
            let args = js_obj();
            set_str(&args, "group", &group);
            set_opt_str(&args, "country", &country);

            match invoke_with("xc_refresh_taxonomy", &args).await {
                Ok(val) => {
                    species_list.set(parse_species_list(&val));
                    taxonomy_age.set(Some("just now".to_string()));
                }
                Err(e) => error_msg.set(Some(e)),
            }
            loading.set(false);
        });
    };

    let check_cached = move |ids: Vec<u64>| {
        spawn_local(async move {
            let mut set = std::collections::HashSet::new();
            for id in ids {
                let args = js_obj();
                set_u64(&args, "id", id);
                if let Ok(val) = invoke_with("xc_is_cached", &args).await {
                    if val.as_bool().unwrap_or(false) {
                        set.insert(id);
                    }
                }
            }
            cached_ids.set(set);
        });
    };

    let load_species_recordings = move |genus: String, species: String, en: String| {
        view.set(BrowserView::SpeciesRecordings {
            genus: genus.clone(),
            species: species.clone(),
            en,
        });
        loading.set(true);
        recordings.set(Vec::new());
        recordings_page.set(1);
        error_msg.set(None);

        spawn_local(async move {
            let args = js_obj();
            set_str(&args, "genus", &genus);
            set_str(&args, "species", &species);
            set_opt_u32(&args, "page", None);

            match invoke_with("xc_species_recordings", &args).await {
                Ok(val) => {
                    recordings.set(parse_recordings(&val));
                    recordings_page.set(parse_current_page(&val));
                    recordings_total_pages.set(parse_num_pages(&val));
                    recordings_total.set(parse_num_recordings(&val));
                    let ids: Vec<u64> = recordings.get_untracked().iter().map(|r| r.id).collect();
                    check_cached(ids);
                }
                Err(e) => error_msg.set(Some(e)),
            }
            loading.set(false);
        });
    };

    let on_search = move |_: web_sys::MouseEvent| {
        let query = search_input.get_untracked().trim().to_string();
        if query.is_empty() {
            return;
        }
        view.set(BrowserView::SearchResults);
        loading.set(true);
        recordings.set(Vec::new());
        error_msg.set(None);

        spawn_local(async move {
            let args = js_obj();
            set_str(&args, "query", &query);
            set_opt_u32(&args, "page", None);

            match invoke_with("xc_search", &args).await {
                Ok(val) => {
                    recordings.set(parse_recordings(&val));
                    recordings_page.set(parse_current_page(&val));
                    recordings_total_pages.set(parse_num_pages(&val));
                    recordings_total.set(parse_num_recordings(&val));
                    let ids: Vec<u64> = recordings.get_untracked().iter().map(|r| r.id).collect();
                    check_cached(ids);
                }
                Err(e) => error_msg.set(Some(e)),
            }
            loading.set(false);
        });
    };

    let on_search_keydown = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" {
            on_search(ev.unchecked_into());
        }
    };

    let on_country_keydown = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" {
            load_group();
        }
    };

    let on_back = move |_: web_sys::MouseEvent| {
        view.set(BrowserView::GroupBrowse);
        recordings.set(Vec::new());
    };

    let load_recordings_page = move |page_num: u32| {
        let current_view = view.get_untracked();
        loading.set(true);
        recordings.set(Vec::new());
        error_msg.set(None);

        spawn_local(async move {
            let result = match &current_view {
                BrowserView::SpeciesRecordings { genus, species, .. } => {
                    let args = js_obj();
                    set_str(&args, "genus", genus);
                    set_str(&args, "species", species);
                    set_opt_u32(&args, "page", Some(page_num));
                    invoke_with("xc_species_recordings", &args).await
                }
                BrowserView::SearchResults => {
                    let args = js_obj();
                    set_str(&args, "query", &search_input.get_untracked());
                    set_opt_u32(&args, "page", Some(page_num));
                    invoke_with("xc_search", &args).await
                }
                _ => return,
            };

            match result {
                Ok(val) => {
                    recordings.set(parse_recordings(&val));
                    recordings_page.set(parse_current_page(&val));
                    recordings_total_pages.set(parse_num_pages(&val));
                    recordings_total.set(parse_num_recordings(&val));
                    let ids: Vec<u64> = recordings.get_untracked().iter().map(|r| r.id).collect();
                    check_cached(ids);
                }
                Err(e) => error_msg.set(Some(e)),
            }
            loading.set(false);
        });
    };

    let download_and_load = move |id: u64| {
        downloading.set(Some(id));
        error_msg.set(None);
        spawn_local(async move {
            let args = js_obj();
            set_u64(&args, "id", id);

            match invoke_with("xc_download", &args).await {
                Ok(val) => {
                    if let Some(cached) = parse_cached_file(&val) {
                        // Read the file bytes and load into the app
                        let path_args = js_obj();
                        set_str(&path_args, "path", &cached.path);
                        match invoke_with("audio_decode_full", &path_args).await {
                            Ok(decode_result) => {
                                load_from_tauri_decode(
                                    &cached.filename,
                                    &decode_result,
                                    cached.metadata,
                                    state,
                                );
                                cached_ids.update(|s| { s.insert(id); });
                                state.xc_browser_open.set(false);
                            }
                            Err(e) => {
                                log::error!("Failed to decode XC audio: {e}");
                                error_msg.set(Some(format!("Failed to decode: {e}")));
                            }
                        }
                    }
                }
                Err(e) => {
                    log::error!("Failed to download XC{id}: {e}");
                    error_msg.set(Some(format!("Download failed: {e}")));
                }
            }
            downloading.set(None);
        });
    };

    view! {
        <div class="xc-modal-overlay" on:click=on_close>
            <div class="xc-modal" on:click=on_content_click>
                <div class="xc-modal-header">
                    <span class="xc-modal-title">"Explore Xeno-Canto"</span>
                    <button class="xc-modal-close" on:click=on_close>{"\u{00D7}"}</button>
                </div>

                // Error display
                {move || error_msg.get().map(|msg| view! {
                    <div class="xc-error">
                        <span>{msg}</span>
                        <button class="xc-error-dismiss" on:click=move |_| error_msg.set(None)>{"\u{00D7}"}</button>
                    </div>
                })}

                // Download progress indicator
                {move || downloading.get().map(|id| view! {
                    <div class="xc-downloading">{format!("Downloading XC{id}\u{2026}")}</div>
                })}

                // API key prompt
                {move || {
                    if view.get() == BrowserView::ApiKeyPrompt {
                        Some(view! {
                            <div class="xc-section">
                                <p class="xc-info">
                                    "Enter your Xeno-Canto API key. You can get one by creating a free account at "
                                    <a href="https://xeno-canto.org" target="_blank">"xeno-canto.org"</a>
                                    " and going to your account settings."
                                </p>
                                <div class="xc-key-form">
                                    <input
                                        type="password"
                                        class="xc-input"
                                        placeholder="API key"
                                        on:input=move |ev| {
                                            let val = event_target_value(&ev);
                                            api_key_input.set(val);
                                        }
                                    />
                                    <button class="xc-btn" on:click=on_save_key>"Save key"</button>
                                </div>
                            </div>
                        })
                    } else {
                        None
                    }
                }}

                // Main browse/search view
                {move || {
                    let current_view = view.get();
                    if !has_key.get() { return None; }

                    Some(view! {
                        <div class="xc-section">
                            // Search bar
                            <div class="xc-search-bar">
                                <input
                                    type="text"
                                    class="xc-input xc-search-input"
                                    placeholder="Search (e.g. Myotis, bat, Australia...)"
                                    prop:value=move || search_input.get()
                                    on:input=move |ev| search_input.set(event_target_value(&ev))
                                    on:keydown=on_search_keydown
                                />
                                <button class="xc-btn" on:click=on_search>"Search"</button>
                            </div>

                            // Group/country filters
                            {move || {
                                if matches!(current_view, BrowserView::GroupBrowse) {
                                    Some(view! {
                                        <div class="xc-filters">
                                            <label>"Group: "</label>
                                            <select
                                                class="xc-select"
                                                on:change=move |ev| {
                                                    selected_group.set(event_target_value(&ev));
                                                }
                                            >
                                                {XC_GROUPS.iter().map(|g| {
                                                    let g = g.to_string();
                                                    let g2 = g.clone();
                                                    let g3 = g.clone();
                                                    view! {
                                                        <option
                                                            value=g.clone()
                                                            selected=move || selected_group.get() == g2
                                                        >{g3}</option>
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </select>
                                            <label>" Country: "</label>
                                            <input
                                                type="text"
                                                class="xc-input xc-country-input"
                                                placeholder="All"
                                                prop:value=move || country_input.get()
                                                on:input=move |ev| country_input.set(event_target_value(&ev))
                                                on:keydown=on_country_keydown
                                            />
                                            <button class="xc-btn" on:click=on_load_group>"Go"</button>
                                        </div>
                                        <div class="xc-cache-info">
                                            {move || taxonomy_age.get().map(|age| view! {
                                                <span class="xc-cache-age">{"Cached: "}{age}</span>
                                            })}
                                            <button class="xc-btn xc-btn-small" on:click=on_refresh>"Refresh"</button>
                                        </div>
                                    })
                                } else {
                                    None
                                }
                            }}

                            // Back button for sub-views
                            {move || {
                                if !matches!(view.get(), BrowserView::GroupBrowse | BrowserView::ApiKeyPrompt) {
                                    Some(view! {
                                        <button class="xc-btn xc-btn-back" on:click=on_back>
                                            {"\u{2190} Back to species"}
                                        </button>
                                    })
                                } else {
                                    None
                                }
                            }}
                        </div>
                    })
                }}

                // Loading indicator
                {move || loading.get().then(|| view! {
                    <div class="xc-loading">"Loading..."</div>
                })}

                // Species list (group browse view)
                {move || {
                    if view.get() != BrowserView::GroupBrowse { return None; }
                    let list = species_list.get();
                    if list.is_empty() && !loading.get() { return None; }

                    let count = list.len();
                    Some(view! {
                        <div class="xc-result-summary">{format!("{count} species")}</div>
                        <div class="xc-species-list">
                            <div class="xc-list-header">
                                <span class="xc-col-name">"Species"</span>
                                <span class="xc-col-sci">"Scientific name"</span>
                                <span class="xc-col-count">"Recs"</span>
                            </div>
                            {list.into_iter().map(|sp| {
                                let gen = sp.gen.clone();
                                let species = sp.sp.clone();
                                let en = sp.en.clone();
                                let load_sp = load_species_recordings.clone();
                                view! {
                                    <button
                                        class="xc-species-row"
                                        on:click=move |_| {
                                            load_sp(gen.clone(), species.clone(), en.clone());
                                        }
                                    >
                                        <span class="xc-col-name">{sp.en}</span>
                                        <span class="xc-col-sci">{format!("{} {}", sp.gen, sp.sp)}</span>
                                        <span class="xc-col-count">{sp.recording_count}</span>
                                    </button>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    })
                }}

                // Recordings list (species or search view)
                {move || {
                    let current_view = view.get();
                    if !matches!(current_view, BrowserView::SpeciesRecordings { .. } | BrowserView::SearchResults) {
                        return None;
                    }
                    let recs = recordings.get();
                    let title = match &current_view {
                        BrowserView::SpeciesRecordings { en, genus, species } => {
                            format!("{en} ({genus} {species})")
                        }
                        BrowserView::SearchResults => "Search results".to_string(),
                        _ => String::new(),
                    };

                    Some(view! {
                        <div class="xc-recordings-header">
                            {title}
                            <span class="xc-result-count">
                                {move || {
                                    let total = recordings_total.get();
                                    if total > 0 {
                                        format!(" \u{2014} {} recordings", total)
                                    } else {
                                        String::new()
                                    }
                                }}
                            </span>
                        </div>
                        <div class="xc-recordings-list">
                            <div class="xc-rec-header">
                                <span class="xc-rec-id">"ID"</span>
                                <span class="xc-rec-species">"Species"</span>
                                <span class="xc-rec-quality">"Q"</span>
                                <span class="xc-rec-length">"Len"</span>
                                <span class="xc-rec-loc">"Location"</span>
                                <span class="xc-rec-action"></span>
                            </div>
                            {recs.into_iter().map(|rec| {
                                let id = rec.id;
                                let dl = download_and_load.clone();
                                let q_class = match rec.q.as_str() {
                                    "A" => "xc-rec-quality xc-q-a",
                                    "B" => "xc-rec-quality xc-q-b",
                                    "C" => "xc-rec-quality xc-q-c",
                                    "D" => "xc-rec-quality xc-q-d",
                                    "E" => "xc-rec-quality xc-q-e",
                                    _ => "xc-rec-quality",
                                };
                                let has_details = !rec.sound_type.is_empty()
                                    || !rec.smp.is_empty()
                                    || !rec.date.is_empty()
                                    || !rec.rec.is_empty();
                                view! {
                                    <div class="xc-rec-row">
                                        <div class="xc-rec-main">
                                            <span class="xc-rec-id">
                                                <a
                                                    href=format!("https://xeno-canto.org/{}", rec.id)
                                                    target="_blank"
                                                    class="xc-rec-link"
                                                    on:click=move |ev: web_sys::MouseEvent| ev.stop_propagation()
                                                >
                                                    {format!("XC{}", rec.id)}
                                                </a>
                                            </span>
                                            <span class="xc-rec-species">{rec.en}</span>
                                            <span class=q_class>{rec.q}</span>
                                            <span class="xc-rec-length">{rec.length}</span>
                                            <span class="xc-rec-loc" title=rec.loc.clone()>{rec.cnt}</span>
                                            <span class="xc-rec-action">
                                                {move || cached_ids.get().contains(&id).then(|| view! {
                                                    <span class="xc-rec-cached" title="Cached locally">{"\u{2713}"}</span>
                                                })}
                                                <button
                                                    class="xc-btn xc-btn-load"
                                                    disabled=move || downloading.get().is_some()
                                                    on:click=move |_| dl(id)
                                                >
                                                    {move || if downloading.get() == Some(id) { "Downloading\u{2026}" } else { "Load" }}
                                                </button>
                                            </span>
                                        </div>
                                        {has_details.then(|| view! {
                                            <div class="xc-rec-detail">
                                                {(!rec.sound_type.is_empty()).then(|| view! {
                                                    <span class="xc-rec-tag">{rec.sound_type}</span>
                                                })}
                                                {(!rec.smp.is_empty()).then(|| view! {
                                                    <span class="xc-rec-tag xc-rec-smp">{format_sample_rate(&rec.smp)}</span>
                                                })}
                                                {(!rec.date.is_empty()).then(|| view! {
                                                    <span class="xc-rec-tag">{rec.date}</span>
                                                })}
                                                {(!rec.rec.is_empty()).then(|| view! {
                                                    <span class="xc-rec-tag xc-rec-recordist">{rec.rec}</span>
                                                })}
                                            </div>
                                        })}
                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                        </div>

                        // Pagination
                        {move || {
                            let total = recordings_total_pages.get();
                            if total <= 1 { return None; }
                            Some(view! {
                                <div class="xc-pagination">
                                    <button
                                        class="xc-btn xc-btn-small"
                                        disabled=move || recordings_page.get() <= 1
                                        on:click=move |_| load_recordings_page(recordings_page.get_untracked().saturating_sub(1))
                                    >
                                        {"\u{2190} Prev"}
                                    </button>
                                    <span class="xc-page-info">
                                        {move || format!("Page {} of {}", recordings_page.get(), recordings_total_pages.get())}
                                    </span>
                                    <button
                                        class="xc-btn xc-btn-small"
                                        disabled=move || recordings_page.get() >= recordings_total_pages.get()
                                        on:click=move |_| load_recordings_page(recordings_page.get_untracked() + 1)
                                    >
                                        {"Next \u{2192}"}
                                    </button>
                                </div>
                            })
                        }}
                    })
                }}
            </div>
        </div>
    }
}

/// Load audio from Tauri's decode result into the app file list.
fn load_from_tauri_decode(
    filename: &str,
    decode_result: &JsValue,
    xc_metadata: Vec<(String, String)>,
    state: AppState,
) {
    let samples_val = js_sys::Reflect::get(decode_result, &"samples".into()).ok();
    let sample_rate = js_sys::Reflect::get(decode_result, &"sample_rate".into())
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(44100.0) as u32;
    let samples: Vec<f32> = if let Some(val) = samples_val {
        // dyn_into consumes val, so on Err we get it back
        match val.dyn_into::<js_sys::Float32Array>() {
            Ok(arr) => arr.to_vec(),
            Err(val) => {
                match val.dyn_into::<js_sys::Array>() {
                    Ok(arr) => (0..arr.length())
                        .filter_map(|i| arr.get(i).as_f64().map(|v| v as f32))
                        .collect(),
                    Err(_) => Vec::new(),
                }
            }
        }
    } else {
        Vec::new()
    };

    if samples.is_empty() {
        log::error!("No samples in decode result");
        return;
    }

    let duration_secs = samples.len() as f64 / sample_rate as f64;
    let audio = crate::types::AudioData {
        samples: std::sync::Arc::new(samples),
        sample_rate,
        channels: 1,
        duration_secs,
        metadata: crate::types::FileMetadata {
            file_size: 0,
            format: "wav",
            bits_per_sample: 16,
            is_float: false,
            guano: None,
        },
    };

    let name = filename.to_string();
    let name_clone = name.clone();

    // Use loading pipeline
    state.loading_count.update(|c| *c += 1);
    spawn_local(async move {
        use crate::dsp::fft::{compute_preview, compute_spectrogram_partial};
        use crate::types::SpectrogramData;
        use wasm_bindgen_futures::JsFuture;

        let preview = compute_preview(&audio, 256, 128);
        let audio_for_stft = audio.clone();

        let placeholder_spec = SpectrogramData {
            columns: std::sync::Arc::new(Vec::new()),
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
                files.push(crate::state::LoadedFile {
                    name: name_clone.clone(),
                    audio,
                    spectrogram: placeholder_spec,
                    preview: Some(preview),
                    xc_metadata: Some(xc_metadata),
                    is_recording: false,
                });
                if files.len() == 1 {
                    state.current_file_index.set(Some(0));
                }
            });
            file_index = idx;
        }

        // Yield
        let p = js_sys::Promise::new(&mut |resolve, _| {
            web_sys::window().unwrap().set_timeout_with_callback(&resolve).unwrap();
        });
        JsFuture::from(p).await.ok();

        // Compute spectrogram in chunks
        const FFT_SIZE: usize = 2048;
        const HOP_SIZE: usize = 512;
        const CHUNK_COLS: usize = 32;

        let total_cols = if audio_for_stft.samples.len() >= FFT_SIZE {
            (audio_for_stft.samples.len() - FFT_SIZE) / HOP_SIZE + 1
        } else {
            0
        };

        let mut all_columns: Vec<Option<crate::types::SpectrogramColumn>> =
            (0..total_cols).map(|_| None).collect();

        let chunks = (total_cols + CHUNK_COLS - 1) / CHUNK_COLS;
        for chunk_idx in 0..chunks {
            let chunk_start = chunk_idx * CHUNK_COLS;
            if chunk_start >= total_cols { continue; }

            let still_present = state.files.get_untracked()
                .get(file_index)
                .map(|f| f.name == name_clone)
                .unwrap_or(false);
            if !still_present { state.loading_count.update(|c| *c = c.saturating_sub(1)); return; }

            let chunk = compute_spectrogram_partial(
                &audio_for_stft, FFT_SIZE, HOP_SIZE, chunk_start, CHUNK_COLS,
            );
            for (i, col) in chunk.into_iter().enumerate() {
                let idx = chunk_start + i;
                if idx < total_cols { all_columns[idx] = Some(col); }
            }

            let p = js_sys::Promise::new(&mut |resolve, _| {
                web_sys::window().unwrap().set_timeout_with_callback(&resolve).unwrap();
            });
            JsFuture::from(p).await.ok();
        }

        let final_columns: Vec<crate::types::SpectrogramColumn> = all_columns
            .into_iter()
            .map(|opt| opt.unwrap_or_else(|| crate::types::SpectrogramColumn {
                magnitudes: Vec::new(),
                time_offset: 0.0,
            }))
            .collect();

        let freq_resolution = audio_for_stft.sample_rate as f64 / FFT_SIZE as f64;
        let max_freq = audio_for_stft.sample_rate as f64 / 2.0;
        let time_resolution = HOP_SIZE as f64 / audio_for_stft.sample_rate as f64;

        let spectrogram = SpectrogramData {
            columns: std::sync::Arc::new(final_columns),
            freq_resolution,
            time_resolution,
            max_freq,
            sample_rate: audio_for_stft.sample_rate,
        };

        state.files.update(|files| {
            if let Some(f) = files.get_mut(file_index) {
                if f.name == name_clone {
                    f.spectrogram = spectrogram;
                }
            }
        });

        state.loading_count.update(|c| *c = c.saturating_sub(1));
    });
}

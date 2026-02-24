use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::Manager;
use xc_lib::{api, cache, taxonomy, XcGroupTaxonomy, XcRecording, XcSearchResult};

/// Shared state for XC operations.
pub struct XcState {
    pub client: reqwest::Client,
    pub cache_root: PathBuf,
}

// ── API Key management ────────────────────────────────────────────────

fn api_key_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir.join("xc_api_key.txt"))
}

#[tauri::command]
pub fn xc_set_api_key(app: tauri::AppHandle, key: String) -> Result<(), String> {
    let path = api_key_path(&app)?;
    std::fs::write(&path, key.trim()).map_err(|e| format!("Failed to save API key: {e}"))
}

#[tauri::command]
pub fn xc_get_api_key(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let path = api_key_path(&app)?;
    if path.exists() {
        let key = std::fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read API key: {e}"))?;
        let key = key.trim().to_string();
        if key.is_empty() {
            Ok(None)
        } else {
            Ok(Some(key))
        }
    } else {
        Ok(None)
    }
}

fn require_api_key(app: &tauri::AppHandle) -> Result<String, String> {
    let path = api_key_path(app)?;
    if !path.exists() {
        return Err("No XC API key configured. Please set your key first.".into());
    }
    let key = std::fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read API key: {e}"))?;
    let key = key.trim().to_string();
    if key.is_empty() {
        Err("XC API key is empty. Please set your key.".into())
    } else {
        Ok(key)
    }
}

// ── Taxonomy browsing ─────────────────────────────────────────────────

#[tauri::command]
pub async fn xc_browse_group(
    app: tauri::AppHandle,
    state: tauri::State<'_, Mutex<XcState>>,
    group: String,
    country: Option<String>,
) -> Result<XcGroupTaxonomy, String> {
    let api_key = require_api_key(&app)?;
    let (client, cache_root) = {
        let s = state.lock().map_err(|e| e.to_string())?;
        (s.client.clone(), s.cache_root.clone())
    };

    let country_ref = country.as_deref();

    // Try cache first
    if let Ok(Some(cached)) = cache::load_taxonomy(&cache_root, &group, country_ref) {
        return Ok(cached);
    }

    // Fetch from API
    let result = taxonomy::build_species_list(
        &client,
        &api_key,
        &group,
        country_ref,
        |_, _| {},
    )
    .await?;

    // Cache result
    let _ = cache::save_taxonomy(&cache_root, &group, country_ref, &result);

    Ok(result)
}

#[tauri::command]
pub async fn xc_refresh_taxonomy(
    app: tauri::AppHandle,
    state: tauri::State<'_, Mutex<XcState>>,
    group: String,
    country: Option<String>,
) -> Result<XcGroupTaxonomy, String> {
    let api_key = require_api_key(&app)?;
    let (client, cache_root) = {
        let s = state.lock().map_err(|e| e.to_string())?;
        (s.client.clone(), s.cache_root.clone())
    };

    let country_ref = country.as_deref();

    let result = taxonomy::build_species_list(
        &client,
        &api_key,
        &group,
        country_ref,
        |_, _| {},
    )
    .await?;

    let _ = cache::save_taxonomy(&cache_root, &group, country_ref, &result);

    Ok(result)
}

#[tauri::command]
pub fn xc_taxonomy_age(
    state: tauri::State<'_, Mutex<XcState>>,
    group: String,
    country: Option<String>,
) -> Result<Option<String>, String> {
    let cache_root = {
        let s = state.lock().map_err(|e| e.to_string())?;
        s.cache_root.clone()
    };
    Ok(cache::taxonomy_age_string(&cache_root, &group, country.as_deref()))
}

// ── Search ────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn xc_search(
    app: tauri::AppHandle,
    state: tauri::State<'_, Mutex<XcState>>,
    query: String,
    page: Option<u32>,
) -> Result<XcSearchResult, String> {
    let api_key = require_api_key(&app)?;
    let client = {
        let s = state.lock().map_err(|e| e.to_string())?;
        s.client.clone()
    };

    api::search(&client, &api_key, &query, page.unwrap_or(1), 100).await
}

#[tauri::command]
pub async fn xc_species_recordings(
    app: tauri::AppHandle,
    state: tauri::State<'_, Mutex<XcState>>,
    genus: String,
    species: String,
    page: Option<u32>,
) -> Result<XcSearchResult, String> {
    let api_key = require_api_key(&app)?;
    let client = {
        let s = state.lock().map_err(|e| e.to_string())?;
        s.client.clone()
    };

    let query = format!("gen:{genus} sp:{species}");
    api::search(&client, &api_key, &query, page.unwrap_or(1), 100).await
}

// ── Download & cache ──────────────────────────────────────────────────

/// Result of downloading/loading a cached XC recording.
#[derive(Serialize, Deserialize)]
pub struct XcCachedFile {
    pub path: String,
    pub filename: String,
    pub xc_id: u64,
    pub metadata: Vec<(String, String)>,
}

#[tauri::command]
pub async fn xc_download(
    app: tauri::AppHandle,
    state: tauri::State<'_, Mutex<XcState>>,
    id: u64,
) -> Result<XcCachedFile, String> {
    let api_key = require_api_key(&app)?;
    let (client, cache_root) = {
        let s = state.lock().map_err(|e| e.to_string())?;
        (s.client.clone(), s.cache_root.clone())
    };

    // Check if already cached
    if let Some(audio_path) = cache::cached_audio_path(&cache_root, id) {
        let filename = audio_path
            .file_name()
            .map(|f| f.to_string_lossy().to_string())
            .unwrap_or_default();
        // Load metadata from sidecar
        let metadata = load_sidecar_metadata(&cache_root, id);
        return Ok(XcCachedFile {
            path: audio_path.to_string_lossy().to_string(),
            filename,
            xc_id: id,
            metadata,
        });
    }

    // Fetch recording info
    let rec = api::fetch_recording(&client, &api_key, id).await?;

    // Download audio
    let audio_bytes = api::download_audio(&client, &rec.file_url).await?;

    // Save to cache
    let audio_path = cache::save_recording(&cache_root, &rec, &audio_bytes)?;

    let filename = audio_path
        .file_name()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or_default();
    let metadata = recording_to_metadata(&rec);

    Ok(XcCachedFile {
        path: audio_path.to_string_lossy().to_string(),
        filename,
        xc_id: id,
        metadata,
    })
}

#[tauri::command]
pub fn xc_is_cached(
    state: tauri::State<'_, Mutex<XcState>>,
    id: u64,
) -> bool {
    let cache_root = match state.lock() {
        Ok(s) => s.cache_root.clone(),
        Err(_) => return false,
    };
    cache::is_recording_cached(&cache_root, id)
}

// ── Helpers ───────────────────────────────────────────────────────────

fn recording_to_metadata(rec: &XcRecording) -> Vec<(String, String)> {
    let mut fields = Vec::new();
    if !rec.en.is_empty() {
        fields.push(("Species".into(), rec.en.clone()));
    }
    if !rec.gen.is_empty() && !rec.sp.is_empty() {
        fields.push(("Scientific name".into(), format!("{} {}", rec.gen, rec.sp)));
    }
    for (val, label) in [
        (&rec.rec, "Recordist"),
        (&rec.lic, "License"),
        (&rec.cnt, "Country"),
        (&rec.loc, "Location"),
    ] {
        if !val.is_empty() {
            fields.push((label.into(), val.clone()));
        }
    }
    let attribution = format!(
        "{}, XC{}. Accessible at www.xeno-canto.org/{}",
        rec.rec, rec.id, rec.id
    );
    fields.push(("Attribution".into(), attribution));
    if !rec.lat.is_empty() && !rec.lon.is_empty() {
        fields.push(("Coordinates".into(), format!("{}, {}", rec.lat, rec.lon)));
    }
    for (val, label) in [
        (&rec.date, "Date"),
        (&rec.sound_type, "Sound type"),
        (&rec.q, "Quality"),
    ] {
        if !val.is_empty() {
            fields.push((label.into(), val.clone()));
        }
    }
    fields.push(("URL".into(), format!("https://www.xeno-canto.org/{}", rec.id)));
    fields
}

fn load_sidecar_metadata(cache_root: &std::path::Path, id: u64) -> Vec<(String, String)> {
    let sounds_dir = cache_root.join("sounds");
    let prefix = format!("XC{id} -");
    if let Ok(entries) = std::fs::read_dir(&sounds_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with(&prefix) && name.ends_with(".xc.json") {
                if let Ok(content) = std::fs::read_to_string(entry.path()) {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                        return parse_xc_json_metadata(&json);
                    }
                }
            }
        }
    }
    Vec::new()
}

fn parse_xc_json_metadata(json: &serde_json::Value) -> Vec<(String, String)> {
    let mut fields = Vec::new();
    let s = |key: &str| json[key].as_str().unwrap_or("").to_string();

    let en = s("en");
    if !en.is_empty() {
        fields.push(("Species".into(), en));
    }
    let gen = s("gen");
    let sp = s("sp");
    if !gen.is_empty() && !sp.is_empty() {
        fields.push(("Scientific name".into(), format!("{gen} {sp}")));
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
        fields.push(("Coordinates".into(), format!("{lat}, {lon}")));
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

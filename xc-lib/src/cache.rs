use std::fs;
use std::path::{Path, PathBuf};
use crate::types::{XcGroupTaxonomy, XcRecording};

/// Sanitize a string for use in filenames.
pub fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => '_',
            _ => c,
        })
        .collect()
}

/// Build the base filename stem for a recording (no extension).
/// e.g. "XC928094 - Pond Myotis - Myotis dasycneme"
pub fn recording_stem(rec: &XcRecording) -> String {
    sanitize_filename(&format!("XC{} - {} - {} {}", rec.id, rec.en, rec.gen, rec.sp))
}

/// Determine audio file extension from the recording's original filename.
pub fn audio_extension(rec: &XcRecording) -> &str {
    rec.file_name
        .rsplit('.')
        .next()
        .unwrap_or("wav")
}

/// Path to the taxonomy cache file for a group.
pub fn taxonomy_path(root: &Path, group: &str, country: Option<&str>) -> PathBuf {
    let dir = root.join("taxonomy");
    let name = match country {
        Some(cnt) => format!("{}_{}.json", group, sanitize_filename(cnt)),
        None => format!("{}.json", group),
    };
    dir.join(name)
}

/// Load cached taxonomy for a group, if it exists.
pub fn load_taxonomy(root: &Path, group: &str, country: Option<&str>) -> Result<Option<XcGroupTaxonomy>, String> {
    let path = taxonomy_path(root, group, country);
    if !path.exists() {
        return Ok(None);
    }
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read {}: {e}", path.display()))?;
    let taxonomy: XcGroupTaxonomy = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse {}: {e}", path.display()))?;
    Ok(Some(taxonomy))
}

/// Save taxonomy cache for a group.
pub fn save_taxonomy(root: &Path, group: &str, country: Option<&str>, data: &XcGroupTaxonomy) -> Result<(), String> {
    let path = taxonomy_path(root, group, country);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create dir {}: {e}", parent.display()))?;
    }
    let json = serde_json::to_string_pretty(data)
        .map_err(|e| format!("Serialize error: {e}"))?;
    fs::write(&path, format!("{json}\n"))
        .map_err(|e| format!("Failed to write {}: {e}", path.display()))?;
    Ok(())
}

/// Check how old the taxonomy cache is (returns human-readable string).
pub fn taxonomy_age_string(root: &Path, group: &str, country: Option<&str>) -> Option<String> {
    let taxonomy = load_taxonomy(root, group, country).ok()??;
    let updated = chrono::DateTime::parse_from_rfc3339(&taxonomy.last_updated).ok()?;
    let now = chrono::Utc::now();
    let duration = now.signed_duration_since(updated);

    let hours = duration.num_hours();
    if hours < 1 {
        Some("just now".to_string())
    } else if hours < 24 {
        Some(format!("{hours} hour{} ago", if hours == 1 { "" } else { "s" }))
    } else {
        let days = duration.num_days();
        Some(format!("{days} day{} ago", if days == 1 { "" } else { "s" }))
    }
}

/// Check if a recording's audio is already cached.
pub fn is_recording_cached(root: &Path, id: u64) -> bool {
    let sounds_dir = root.join("sounds");
    if !sounds_dir.exists() {
        return false;
    }
    // Look for any file starting with "XC{id} -"
    let prefix = format!("XC{id} -");
    if let Ok(entries) = fs::read_dir(&sounds_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with(&prefix) && !name.ends_with(".xc.json") {
                return true;
            }
        }
    }
    false
}

/// Find the cached audio file path for a recording.
pub fn cached_audio_path(root: &Path, id: u64) -> Option<PathBuf> {
    let sounds_dir = root.join("sounds");
    let prefix = format!("XC{id} -");
    if let Ok(entries) = fs::read_dir(&sounds_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with(&prefix) && !name.ends_with(".xc.json") {
                return Some(entry.path());
            }
        }
    }
    None
}

/// Build the XC metadata sidecar JSON for a recording.
pub fn build_metadata_json(rec: &XcRecording) -> serde_json::Value {
    let attribution = format!(
        "{}, XC{}. Accessible at www.xeno-canto.org/{}",
        rec.rec, rec.id, rec.id
    );
    let now = chrono::Utc::now().format("%Y-%m-%d").to_string();

    serde_json::json!({
        "source": "xeno-canto",
        "xc_id": rec.id,
        "url": format!("https://www.xeno-canto.org/{}", rec.id),
        "file_url": rec.file_url,
        "gen": rec.gen,
        "sp": rec.sp,
        "en": rec.en,
        "rec": rec.rec,
        "cnt": rec.cnt,
        "loc": rec.loc,
        "lat": rec.lat,
        "lon": rec.lon,
        "date": rec.date,
        "time": rec.time,
        "type": rec.sound_type,
        "q": rec.q,
        "length": rec.length,
        "smp": rec.smp.parse::<u64>().ok(),
        "lic": rec.lic,
        "attribution": attribution,
        "retrieved": now,
    })
}

/// Save a recording's audio and metadata to the cache.
/// Returns the path to the saved audio file.
pub fn save_recording(
    root: &Path,
    rec: &XcRecording,
    audio_bytes: &[u8],
) -> Result<PathBuf, String> {
    let sounds_dir = root.join("sounds");
    fs::create_dir_all(&sounds_dir)
        .map_err(|e| format!("Failed to create sounds dir: {e}"))?;

    let stem = recording_stem(rec);
    let ext = audio_extension(rec);
    let audio_filename = format!("{stem}.{ext}");
    let meta_filename = format!("{stem}.xc.json");

    // Write audio
    let audio_path = sounds_dir.join(&audio_filename);
    fs::write(&audio_path, audio_bytes)
        .map_err(|e| format!("Failed to write audio: {e}"))?;

    // Write metadata sidecar
    let meta_path = sounds_dir.join(&meta_filename);
    let metadata = build_metadata_json(rec);
    let json_str = serde_json::to_string_pretty(&metadata)
        .map_err(|e| format!("Serialize error: {e}"))?;
    fs::write(&meta_path, format!("{json_str}\n"))
        .map_err(|e| format!("Failed to write metadata: {e}"))?;

    // Update index.json
    update_index(root, rec, &audio_filename, &meta_filename)?;

    Ok(audio_path)
}

/// Update (or create) index.json with a new recording entry.
fn update_index(
    root: &Path,
    rec: &XcRecording,
    audio_filename: &str,
    meta_filename: &str,
) -> Result<(), String> {
    let index_path = root.join("index.json");
    let mut index: serde_json::Value = if index_path.exists() {
        let content = fs::read_to_string(&index_path)
            .map_err(|e| format!("Failed to read index.json: {e}"))?;
        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse index.json: {e}"))?
    } else {
        serde_json::json!({ "version": 1, "sounds": [] })
    };

    let sounds = index["sounds"]
        .as_array_mut()
        .ok_or("index.json 'sounds' is not an array")?;

    // Skip if already present
    if sounds.iter().any(|s| s["xc_id"].as_u64() == Some(rec.id)) {
        return Ok(());
    }

    sounds.push(serde_json::json!({
        "filename": audio_filename,
        "metadata": meta_filename,
        "xc_id": rec.id,
        "en": rec.en,
        "species": format!("{} {}", rec.gen, rec.sp),
        "source": "xeno-canto"
    }));

    let json_str = serde_json::to_string_pretty(&index)
        .map_err(|e| format!("Serialize error: {e}"))?;
    fs::write(&index_path, format!("{json_str}\n"))
        .map_err(|e| format!("Failed to write index.json: {e}"))?;

    Ok(())
}

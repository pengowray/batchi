use crate::types::{XcRecording, XcSearchResult};

const API_BASE: &str = "https://xeno-canto.org/api/3/recordings";

/// Parse an XC recording from API JSON.
fn parse_recording(rec: &serde_json::Value) -> Option<XcRecording> {
    let id = rec["id"].as_str()?.parse::<u64>().ok()?;
    let s = |key: &str| rec[key].as_str().unwrap_or("").to_string();
    Some(XcRecording {
        id,
        gen: s("gen"),
        sp: s("sp"),
        en: s("en"),
        grp: s("grp"),
        fam: s("fam"),
        rec: s("rec"),
        cnt: s("cnt"),
        loc: s("loc"),
        lat: s("lat"),
        lon: s("lon"),
        date: s("date"),
        time: s("time"),
        sound_type: s("type"),
        q: s("q"),
        length: s("length"),
        smp: s("smp"),
        lic: s("lic"),
        file_url: s("file"),
        file_name: s("file-name"),
        ssp: s("ssp"),
        rmk: s("rmk"),
    })
}

/// Parse a search response from the XC API.
fn parse_search_response(body: &serde_json::Value) -> Result<XcSearchResult, String> {
    if let Some(err) = body.get("error") {
        return Err(format!("API error: {}", err));
    }

    let num_recordings = body["numRecordings"]
        .as_str()
        .and_then(|s| s.parse().ok())
        .or_else(|| body["numRecordings"].as_u64().map(|n| n as u32))
        .unwrap_or(0);
    let num_species = body["numSpecies"]
        .as_str()
        .and_then(|s| s.parse().ok())
        .or_else(|| body["numSpecies"].as_u64().map(|n| n as u32))
        .unwrap_or(0);
    let num_pages = body["numPages"]
        .as_u64()
        .map(|n| n as u32)
        .unwrap_or(1);
    let page = body["page"]
        .as_u64()
        .map(|n| n as u32)
        .unwrap_or(1);

    let recordings = body["recordings"]
        .as_array()
        .map(|arr| arr.iter().filter_map(parse_recording).collect())
        .unwrap_or_default();

    Ok(XcSearchResult {
        num_recordings,
        num_species,
        num_pages,
        page,
        recordings,
    })
}

/// Search the XC API with a free-form query string.
///
/// The query uses XC search tag syntax, e.g.:
/// - `"grp:bats"` — all bat recordings
/// - `"grp:bats cnt:Australia"` — Australian bat recordings
/// - `"nr:928094"` — specific recording by number
/// - `"Myotis dasycneme"` — species name search
pub async fn search(
    client: &reqwest::Client,
    api_key: &str,
    query: &str,
    page: u32,
    per_page: u32,
) -> Result<XcSearchResult, String> {
    let url = format!(
        "{}?query={}&key={}&page={}&per_page={}",
        API_BASE,
        urlencod(query),
        urlencod(api_key),
        page,
        per_page.clamp(50, 500),
    );

    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("HTTP request failed: {e}"))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("HTTP {status}: {body}"));
    }

    let body: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse JSON: {e}"))?;

    parse_search_response(&body)
}

/// Fetch a single recording by XC number.
pub async fn fetch_recording(
    client: &reqwest::Client,
    api_key: &str,
    id: u64,
) -> Result<XcRecording, String> {
    let result = search(client, api_key, &format!("nr:{id}"), 1, 50).await?;
    result
        .recordings
        .into_iter()
        .next()
        .ok_or_else(|| format!("No recording found for XC{id}"))
}

/// Download audio bytes for a recording.
pub async fn download_audio(
    client: &reqwest::Client,
    file_url: &str,
) -> Result<Vec<u8>, String> {
    let resp = client
        .get(file_url)
        .send()
        .await
        .map_err(|e| format!("Download failed: {e}"))?;

    if !resp.status().is_success() {
        return Err(format!("Download HTTP {}", resp.status()));
    }

    resp.bytes()
        .await
        .map(|b| b.to_vec())
        .map_err(|e| format!("Failed to read audio bytes: {e}"))
}

/// Minimal URL encoding for query parameters.
fn urlencod(s: &str) -> String {
    s.replace(' ', "%20")
        .replace('&', "%26")
        .replace('=', "%3D")
        .replace('+', "%2B")
        .replace('#', "%23")
}

/// Parse an XC number from various input formats:
/// "928094", "XC928094", "xc928094", "https://xeno-canto.org/928094"
pub fn parse_xc_number(input: &str) -> Result<u64, String> {
    let s = input.trim();

    if let Ok(n) = s.parse::<u64>() {
        return Ok(n);
    }

    if let Some(rest) = s
        .strip_prefix("XC")
        .or_else(|| s.strip_prefix("xc"))
    {
        return rest
            .parse::<u64>()
            .map_err(|_| format!("Invalid XC number: {s}"));
    }

    if s.contains("xeno-canto.org/") {
        if let Some(last) = s.trim_end_matches('/').rsplit('/').next() {
            if let Ok(n) = last.parse::<u64>() {
                return Ok(n);
            }
        }
    }

    Err(format!("Can't parse XC number from: {s}"))
}

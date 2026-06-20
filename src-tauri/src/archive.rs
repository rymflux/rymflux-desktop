#![allow(dead_code, reason = "serde deserialization fields")]
//! Archive.org resolver — NOT a catalog client. Used only when a
//! LibriVox-provided listen_url fails, and later as the file-manifest
//! source for offline downloads. LibriVox remains the sole catalog/search
//! source; this module never calls archive.org search or scrape endpoints.

use rusqlite::Connection;
use serde::Deserialize;
use std::sync::LazyLock;
use std::sync::Mutex;
use std::time::Duration;

static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .user_agent("rymflux-audiobook-player/0.1")
        .timeout(Duration::from_secs(15))
        .build()
        .expect("failed to build archive.org reqwest client")
});

/// Separate DB connection for archive identifier mappings (avoids coupling
/// rymflux-core's StorageEngine with archive.org-specific data).
static DB: LazyLock<Mutex<Option<Connection>>> = LazyLock::new(|| Mutex::new(None));

#[derive(Deserialize)]
pub(crate) struct ArchiveMetadataResponse {
    pub(crate) files: Vec<ArchiveFile>,
}

#[derive(Deserialize)]
pub(crate) struct ArchiveFile {
    pub(crate) name: String,
    pub(crate) format: Option<String>,
    /// archive.org returns these as JSON strings, not numbers — keep as String.
    pub(crate) size: Option<String>,
    pub(crate) length: Option<String>,
}

/// Extracts the archive.org identifier from a LibriVox `url_iarchive` value.
/// e.g. "https://archive.org/details/prideandprejudice_0911_librivox"
///   -> "prideandprejudice_0911_librivox"
pub fn extract_identifier(url_iarchive: &str) -> Option<String> {
    url_iarchive
        .trim_end_matches('/')
        .rsplit('/')
        .next()
        .map(str::to_string)
        .filter(|s| !s.is_empty())
}

/// Lightweight reachability check for a candidate stream URL. Used to decide
/// whether the LibriVox-provided URL needs a fallback at all.
pub async fn url_is_reachable(url: &str) -> bool {
    CLIENT
        .head(url)
        .send()
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false)
}

async fn get_metadata(identifier: &str) -> Result<ArchiveMetadataResponse, String> {
    let url = format!("https://archive.org/metadata/{identifier}");
    let resp = CLIENT.get(&url).send().await.map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!(
            "archive.org metadata request failed: {}",
            resp.status()
        ));
    }
    resp.json::<ArchiveMetadataResponse>()
        .await
        .map_err(|e| e.to_string())
}

pub fn build_download_url(identifier: &str, filename: &str) -> String {
    format!("https://archive.org/download/{identifier}/{filename}")
}

/// Best-effort fallback: given an identifier and a 1-based section number
/// (matches LibriVox's `section_number`), find the matching mp3 in the
/// item's file manifest and return its archive.org URL.
///
/// LibriVox-derived items conventionally name files like
/// `<slug>_<NN>_<author>_64kb.mp3`, but naming isn't fully consistent across
/// older catalog entries — falls back to positional match (Nth mp3 file,
/// sorted by name) if no name match is found.
pub async fn resolve_fallback_stream_url(
    identifier: &str,
    section_number: u32,
) -> Result<String, String> {
    let meta = get_metadata(identifier).await?;
    let mut mp3_files: Vec<&ArchiveFile> = meta
        .files
        .iter()
        .filter(|f| f.name.to_lowercase().ends_with(".mp3"))
        .collect();
    mp3_files.sort_by(|a, b| a.name.cmp(&b.name));

    let padded = format!("{section_number:02}");
    if let Some(f) = mp3_files.iter().find(|f| f.name.contains(&padded)) {
        return Ok(build_download_url(identifier, &f.name));
    }

    mp3_files
        .get(section_number.saturating_sub(1) as usize)
        .map(|f| build_download_url(identifier, &f.name))
        .ok_or_else(|| {
            format!("no mp3 file found for section {section_number} in item {identifier}")
        })
}

/// Initialize the archive IDs table. Called once at app startup.
pub fn init_tables(db_path: &std::path::Path) -> Result<(), String> {
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS audiobook_archive_ids (
            content_id TEXT PRIMARY KEY,
            archive_identifier TEXT NOT NULL
        );",
    )
    .map_err(|e| e.to_string())?;
    let mut db = DB.lock().map_err(|e| e.to_string())?;
    *db = Some(conn);
    Ok(())
}

/// Persist an archive.org identifier for a content item so fallback resolution
/// works even after the LibriVox catalog entry becomes unavailable.
pub fn store_archive_id(content_id: &str, identifier: &str) -> Result<(), String> {
    let db = DB.lock().map_err(|e| e.to_string())?;
    let conn = db
        .as_ref()
        .ok_or_else(|| "archive DB not initialized".to_string())?;
    conn.execute(
        "INSERT OR REPLACE INTO audiobook_archive_ids (content_id, archive_identifier) VALUES (?1, ?2)",
        rusqlite::params![content_id, identifier],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

/// Retrieve a previously-stored archive.org identifier for a content item.
pub fn get_archive_id(content_id: &str) -> Result<Option<String>, String> {
    let db = DB.lock().map_err(|e| e.to_string())?;
    let conn = db
        .as_ref()
        .ok_or_else(|| "archive DB not initialized".to_string())?;
    let mut stmt = conn
        .prepare("SELECT archive_identifier FROM audiobook_archive_ids WHERE content_id = ?1")
        .map_err(|e| e.to_string())?;
    match stmt.query_row(rusqlite::params![content_id], |row| row.get(0)) {
        Ok(val) => Ok(Some(val)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

#[test]
fn extracts_identifier_from_iarchive_url() {
    assert_eq!(
        extract_identifier("https://archive.org/details/prideandprejudice_0911_librivox"),
        Some("prideandprejudice_0911_librivox".to_string())
    );
    assert_eq!(
        extract_identifier("https://archive.org/details/some_item/"),
        Some("some_item".to_string())
    );
    assert_eq!(extract_identifier(""), None);
}

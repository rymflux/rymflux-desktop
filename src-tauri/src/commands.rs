use rymflux_core::commands;
use rymflux_core::audio::PlaybackEngine;
use rymflux_core::storage::StorageEngine;
use rymflux_core::types::{AudioSource, ContentIdentity, ContentItem, DomainId, PlaybackState, ProgressRecord};
use std::sync::Mutex;

// ── Playback ─────────────────────────────────────────────────────────────────

#[tauri::command]
pub fn play_audio(
    engine_state: tauri::State<'_, Mutex<PlaybackEngine>>,
    storage_state: tauri::State<'_, Mutex<StorageEngine>>,
    source: AudioSource,
    content_id: String,
    position_ms: u64,
) -> Result<PlaybackState, String> {
    let mut engine = engine_state.inner().lock().map_err(|e| e.to_string())?;
    let storage = storage_state.inner().lock().map_err(|e| e.to_string())?;
    commands::playback::play(&storage, &mut engine, &source, &content_id, position_ms)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn pause_audio(
    engine_state: tauri::State<'_, Mutex<PlaybackEngine>>,
    storage_state: tauri::State<'_, Mutex<StorageEngine>>,
    domain_id: String,
    content_id: String,
) -> Result<PlaybackState, String> {
    let mut engine = engine_state.inner().lock().map_err(|e| e.to_string())?;
    let storage = storage_state.inner().lock().map_err(|e| e.to_string())?;
    let domain = DomainId::from(domain_id);
    commands::playback::pause(&storage, &mut engine, &domain, &content_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn seek_audio(
    engine_state: tauri::State<'_, Mutex<PlaybackEngine>>,
    storage_state: tauri::State<'_, Mutex<StorageEngine>>,
    domain_id: String,
    content_id: String,
    position_ms: u64,
) -> Result<PlaybackState, String> {
    let mut engine = engine_state.inner().lock().map_err(|e| e.to_string())?;
    let storage = storage_state.inner().lock().map_err(|e| e.to_string())?;
    let domain = DomainId::from(domain_id);
    commands::playback::seek(&storage, &mut engine, &domain, &content_id, position_ms)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_audio_speed(
    engine_state: tauri::State<'_, Mutex<PlaybackEngine>>,
    rate: f32,
) -> Result<PlaybackState, String> {
    let mut engine = engine_state.inner().lock().map_err(|e| e.to_string())?;
    commands::playback::set_speed(&mut engine, rate).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_audio_volume(
    engine_state: tauri::State<'_, Mutex<PlaybackEngine>>,
    volume: f32,
) -> Result<PlaybackState, String> {
    let mut engine = engine_state.inner().lock().map_err(|e| e.to_string())?;
    Ok(commands::playback::set_volume(&mut engine, volume))
}

#[tauri::command]
pub fn get_audio_state(
    engine_state: tauri::State<'_, Mutex<PlaybackEngine>>,
) -> Result<PlaybackState, String> {
    let engine = engine_state.inner().lock().map_err(|e| e.to_string())?;
    Ok(commands::playback::get_state(&engine))
}

#[tauri::command]
pub fn stop_audio(
    engine_state: tauri::State<'_, Mutex<PlaybackEngine>>,
    storage_state: tauri::State<'_, Mutex<StorageEngine>>,
    domain_id: String,
    content_id: String,
) -> Result<PlaybackState, String> {
    let mut engine = engine_state.inner().lock().map_err(|e| e.to_string())?;
    let storage = storage_state.inner().lock().map_err(|e| e.to_string())?;
    let domain = DomainId::from(domain_id);
    commands::playback::stop(&storage, &mut engine, &domain, &content_id)
        .map_err(|e| e.to_string())
}

// ── Library ──────────────────────────────────────────────────────────────────

#[tauri::command]
pub fn library_list(
    storage_state: tauri::State<'_, Mutex<StorageEngine>>,
    domain_id: String,
) -> Result<Vec<ContentItem>, String> {
    let storage = storage_state.inner().lock().map_err(|e| e.to_string())?;
    let domain = DomainId::from(domain_id);
    commands::library::list(&storage, &domain, None, None).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn library_search(
    storage_state: tauri::State<'_, Mutex<StorageEngine>>,
    domain_id: String,
    query: String,
) -> Result<Vec<ContentItem>, String> {
    let storage = storage_state.inner().lock().map_err(|e| e.to_string())?;
    let domain = DomainId::from(domain_id);
    commands::library::search(&storage, &domain, &query).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn library_get_detail(
    storage_state: tauri::State<'_, Mutex<StorageEngine>>,
    content_id: String,
) -> Result<ContentItem, String> {
    let storage = storage_state.inner().lock().map_err(|e| e.to_string())?;
    commands::library::get_detail(&storage, &content_id).map_err(|e| e.to_string())
}

// ── Progress ─────────────────────────────────────────────────────────────────

#[tauri::command]
pub fn progress_get(
    storage_state: tauri::State<'_, Mutex<StorageEngine>>,
    content_id: String,
) -> Result<ProgressRecord, String> {
    let storage = storage_state.inner().lock().map_err(|e| e.to_string())?;
    commands::progress::get(&storage, &content_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn progress_set(
    storage_state: tauri::State<'_, Mutex<StorageEngine>>,
    domain_id: String,
    content_id: String,
    position_ms: i64,
) -> Result<(), String> {
    let storage = storage_state.inner().lock().map_err(|e| e.to_string())?;
    let domain = DomainId::from(domain_id);
    commands::progress::set(&storage, &domain, &content_id, position_ms)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn progress_sync(
    storage_state: tauri::State<'_, Mutex<StorageEngine>>,
    domain_id: String,
) -> Result<Vec<ProgressRecord>, String> {
    let storage = storage_state.inner().lock().map_err(|e| e.to_string())?;
    let domain = DomainId::from(domain_id);
    commands::progress::sync(&storage, &domain).map_err(|e| e.to_string())
}

use crate::librivox;
use std::time::SystemTime;

// ── Catalog types (frontend-facing) ─────────────────────────────────────────

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CatalogItem {
    pub id: String,
    pub title: String,
    pub author: String,
    pub description: String,
    pub total_time_secs: Option<i64>,
    pub num_sections: Option<u32>,
    pub cover_url: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CatalogDetail {
    pub item: CatalogItem,
    pub sections: Vec<ChapterInfo>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChapterInfo {
    pub id: String,
    pub section_number: u32,
    pub title: String,
    pub listen_url: String,
    pub playtime_secs: Option<u64>,
}

impl From<librivox::LibrivoxBook> for CatalogItem {
    fn from(book: librivox::LibrivoxBook) -> Self {
        let author = book
            .authors
            .first()
            .map(|a| format!("{} {}", a.first_name, a.last_name))
            .unwrap_or_default();
        let num_sections = book
            .num_sections
            .as_deref()
            .and_then(|s| s.parse().ok());
        CatalogItem {
            id: book.id,
            title: book.title,
            author,
            description: book.description,
            total_time_secs: book.totaltimesecs,
            num_sections,
            cover_url: None,
        }
    }
}

// ── Catalog commands ─────────────────────────────────────────────────────────

#[tauri::command]
pub async fn catalog_search(
    query: String,
    limit: Option<u32>,
    offset: Option<u32>,
) -> Result<Vec<CatalogItem>, String> {
    let books = librivox::search_by_title(&query, limit.unwrap_or(20), offset.unwrap_or(0)).await?;
    Ok(books.into_iter().map(|b| b.into()).collect())
}

#[tauri::command]
pub async fn catalog_get_book(id: String) -> Result<CatalogDetail, String> {
    let book = librivox::get_book(&id).await?;
    let sections = book
        .sections
        .as_deref()
        .unwrap_or_default()
        .iter()
        .map(|s| {
            let sec_num: u32 = s.section_number.parse().unwrap_or(0);
            let playtime_secs = s
                .playtime
                .as_deref()
                .and_then(|p| p.parse().ok());
            ChapterInfo {
                id: s.id.clone(),
                section_number: sec_num,
                title: s.title.clone(),
                listen_url: s.listen_url.clone(),
                playtime_secs,
            }
        })
        .collect();
    Ok(CatalogDetail {
        item: book.into(),
        sections,
    })
}

#[tauri::command]
pub async fn library_add_from_catalog(
    storage_state: tauri::State<'_, Mutex<StorageEngine>>,
    catalog_id: String,
) -> Result<(), String> {
    let book = librivox::get_book(&catalog_id).await?;
    let domain_id = DomainId::from("librivox");

    // Build content item
    let content_item = librivox::book_to_content_item(&book);
    let storage = storage_state.inner().lock().map_err(|e| e.to_string())?;

    // Store the content identity
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    let identity = ContentIdentity {
        identity_id: format!("librivox_identity_{}", book.id),
        structural_fingerprint: None,
        source_id: Some(format!("librivox_{}", book.id)),
        file_path: None,
        file_name: None,
        file_size: None,
        duration_ms: book.totaltimesecs,
        domain_id: domain_id.clone(),
        first_seen_at: now.to_string(),
        last_seen_at: now.to_string(),
    };
    storage.store_identity(&identity).map_err(|e| e.to_string())?;

    // Upsert the content item
    storage.upsert_content(&content_item).map_err(|e| e.to_string())?;

    Ok(())
}

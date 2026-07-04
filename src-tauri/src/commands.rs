use rymflux_core::audio::PlaybackEngine;
use rymflux_core::commands;
use rymflux_core::storage::StorageEngine;
use rymflux_core::types::{
    AudioSource, ContentIdentity, ContentItem, DomainId, DomainRecord, PlaybackState,
    ProgressRecord, ProgressWriteContext,
};
use std::collections::HashMap;
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
    chapter_index: Option<u32>,
    chapter_offset_ms: Option<u64>,
) -> Result<PlaybackState, String> {
    let mut engine = engine_state.inner().lock().map_err(|e| e.to_string())?;
    let storage = storage_state.inner().lock().map_err(|e| e.to_string())?;
    let domain = DomainId::from(domain_id);
    let ctx = match (chapter_index, chapter_offset_ms) {
        (Some(index), Some(offset)) => Some(ProgressWriteContext {
            chapter_index: index,
            chapter_offset_ms: offset,
        }),
        _ => None,
    };
    commands::playback::pause(&storage, &mut engine, &domain, &content_id, ctx.as_ref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn seek_audio(
    engine_state: tauri::State<'_, Mutex<PlaybackEngine>>,
    storage_state: tauri::State<'_, Mutex<StorageEngine>>,
    domain_id: String,
    content_id: String,
    position_ms: u64,
    chapter_index: Option<u32>,
    chapter_offset_ms: Option<u64>,
) -> Result<PlaybackState, String> {
    let mut engine = engine_state.inner().lock().map_err(|e| e.to_string())?;
    let storage = storage_state.inner().lock().map_err(|e| e.to_string())?;
    let domain = DomainId::from(domain_id);
    let ctx = match (chapter_index, chapter_offset_ms) {
        (Some(index), Some(offset)) => Some(ProgressWriteContext {
            chapter_index: index,
            chapter_offset_ms: offset,
        }),
        _ => None,
    };
    commands::playback::seek(
        &storage,
        &mut engine,
        &domain,
        &content_id,
        position_ms,
        ctx.as_ref(),
    )
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
    chapter_index: Option<u32>,
    chapter_offset_ms: Option<u64>,
) -> Result<PlaybackState, String> {
    let mut engine = engine_state.inner().lock().map_err(|e| e.to_string())?;
    let storage = storage_state.inner().lock().map_err(|e| e.to_string())?;
    let domain = DomainId::from(domain_id);
    let ctx = match (chapter_index, chapter_offset_ms) {
        (Some(index), Some(offset)) => Some(ProgressWriteContext {
            chapter_index: index,
            chapter_offset_ms: offset,
        }),
        _ => None,
    };
    commands::playback::stop(&storage, &mut engine, &domain, &content_id, ctx.as_ref())
        .map_err(|e| e.to_string())
}

// ── Library ──────────────────────────────────────────────────────────────────

#[tauri::command]
pub fn library_remove_from(
    storage_state: tauri::State<'_, Mutex<StorageEngine>>,
    content_id: String,
) -> Result<(), String> {
    let storage = storage_state.inner().lock().map_err(|e| e.to_string())?;
    storage
        .delete_content(&content_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn library_clear(
    storage_state: tauri::State<'_, Mutex<StorageEngine>>,
    domain_id: String,
) -> Result<(), String> {
    use rymflux_core::types::DomainId;
    let storage = storage_state.inner().lock().map_err(|e| e.to_string())?;
    rymflux_core::commands::library::clear(&storage, &DomainId::from(domain_id.as_str()))
        .map_err(|e| e.to_string())
}

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
pub fn library_list_domains(
    storage_state: tauri::State<'_, Mutex<StorageEngine>>,
) -> Result<Vec<DomainRecord>, String> {
    let storage = storage_state.inner().lock().map_err(|e| e.to_string())?;
    commands::library::list_domains(&storage).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn library_count_content(
    storage_state: tauri::State<'_, Mutex<StorageEngine>>,
    domain_id: String,
) -> Result<i64, String> {
    let storage = storage_state.inner().lock().map_err(|e| e.to_string())?;
    let domain = DomainId::from(domain_id);
    commands::library::count_by_domain(&storage, &domain).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn library_count_all(
    storage_state: tauri::State<'_, Mutex<StorageEngine>>,
) -> Result<HashMap<String, i64>, String> {
    let storage = storage_state.inner().lock().map_err(|e| e.to_string())?;
    commands::library::count_all(&storage).map_err(|e| e.to_string())
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
    chapter_index: Option<u32>,
    chapter_offset_ms: Option<u64>,
    speed: Option<f32>,
) -> Result<(), String> {
    let storage = storage_state.inner().lock().map_err(|e| e.to_string())?;
    let domain = DomainId::from(domain_id);
    let ctx = match (chapter_index, chapter_offset_ms) {
        (Some(index), Some(offset)) => Some(ProgressWriteContext {
            chapter_index: index,
            chapter_offset_ms: offset,
        }),
        _ => None,
    };
    commands::progress::set(
        &storage,
        &domain,
        &content_id,
        position_ms,
        ctx.as_ref(),
        speed,
    )
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

// ── Diagnostics ─────────────────────────────────────────────────────────────

#[tauri::command]
pub fn set_diag_mode(enabled: bool) {
    rymflux_core::set_diag_mode(enabled);
}

use std::sync::LazyLock;
use std::time::{Duration, SystemTime};

// ── Generic HTTP proxy (domain-agnostic passthrough) ──────────────────────

static HTTP_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .user_agent("rymflux-audiobook-player/0.1")
        .build()
        .expect("failed to build HTTP client")
});

#[tauri::command]
pub async fn http_get(url: String) -> Result<String, String> {
    let resp = HTTP_CLIENT
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("HTTP request failed: {e}"))?;
    if !resp.status().is_success() {
        return Err(format!("HTTP {}: {}", resp.status(), url));
    }
    resp.text()
        .await
        .map_err(|e| format!("Failed to read response: {e}"))
}

// ── Generic library store command (domain-agnostic) ────────────────────────

#[tauri::command]
pub fn library_store_item(
    storage_state: tauri::State<'_, Mutex<StorageEngine>>,
    content_item: ContentItem,
    identity_source_id: String,
    identity_duration_ms: Option<i64>,
) -> Result<(), String> {
    let storage = storage_state.inner().lock().map_err(|e| e.to_string())?;
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    let identity = ContentIdentity {
        identity_id: rymflux_core::identity::derive_identity_id(&identity_source_id),
        structural_fingerprint: None,
        source_id: Some(identity_source_id),
        file_path: None,
        file_name: None,
        file_size: None,
        duration_ms: identity_duration_ms,
        domain_id: content_item.domain_id.clone(),
        first_seen_at: now.to_string(),
        last_seen_at: now.to_string(),
    };
    storage
        .store_identity(&identity)
        .map_err(|e| e.to_string())?;
    storage
        .upsert_content(&content_item)
        .map_err(|e| e.to_string())?;
    Ok(())
}

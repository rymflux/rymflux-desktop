use rymflux_core::commands;
use rymflux_core::audio::PlaybackEngine;
use rymflux_core::storage::StorageEngine;
use rymflux_core::types::{AudioSource, ContentItem, DomainId, PlaybackState, ProgressRecord};
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

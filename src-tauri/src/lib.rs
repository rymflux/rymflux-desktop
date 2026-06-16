use rymflux_core::types::PlaybackState;
use rymflux_core::storage::StorageEngine;
use rymflux_core::audio::PlaybackEngine;
use rymflux_core::error::CoreError;
use rymflux_core::EventEmitter;
use std::sync::Arc;
use std::sync::Mutex;

struct AppState {
    engine: Mutex<PlaybackEngine>,
    storage: Mutex<StorageEngine>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let event_emitter = Arc::new(DesktopEventEmitter);
    let db_path = dirs::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("rymflux")
        .join("rymflux.db");

    let storage = StorageEngine::new(
        &db_path.to_string_lossy(),
    ).expect("failed to open database");
    let engine = PlaybackEngine::new(event_emitter);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            engine: Mutex::new(engine),
            storage: Mutex::new(storage),
        })
        .invoke_handler(tauri::generate_handler![
            // Phase 2: register command wrappers here
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

struct DesktopEventEmitter;
impl EventEmitter for DesktopEventEmitter {
    fn emit_progress(&self, _state: &PlaybackState) {
        // Phase 2: emit Tauri event to frontend
    }
    fn emit_finished(&self) {
        // Phase 2: emit Tauri event to frontend
    }
    fn emit_error(&self, _error: &CoreError) {
        // Phase 2: emit Tauri event to frontend
    }
}

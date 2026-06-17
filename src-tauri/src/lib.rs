use rymflux_core::types::{PlaybackState};
use rymflux_core::storage::StorageEngine;
use rymflux_core::audio::PlaybackEngine;
use rymflux_core::error::CoreError;
use rymflux_core::EventEmitter;
use std::sync::Arc;
use std::sync::Mutex;
use tauri::Emitter;
use tauri::Manager;

mod commands;

#[expect(dead_code, reason = "wired in step 1.2 — command wrappers consume these fields")]
struct AppState {
    engine: Mutex<PlaybackEngine>,
    storage: Mutex<StorageEngine>,
}

struct DesktopEventEmitter {
    app_handle: Arc<tauri::AppHandle>,
}

impl EventEmitter for DesktopEventEmitter {
    fn emit_progress(&self, state: &PlaybackState) {
        let _ = self.app_handle.emit("audio:progress", state);
    }
    fn emit_finished(&self) {
        let _ = self.app_handle.emit("audio:finished", ());
    }
    fn emit_error(&self, error: &CoreError) {
        let _ = self.app_handle.emit("audio:error", error.to_string());
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_handle = Arc::new(app.handle().clone());
            let event_emitter: Arc<dyn EventEmitter> = Arc::new(DesktopEventEmitter { app_handle });

            let db_path = dirs::data_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("."))
                .join("rymflux")
                .join("rymflux.db");
            if let Some(parent) = db_path.parent() {
                std::fs::create_dir_all(parent).ok();
            }
            let storage = StorageEngine::new(&db_path.to_string_lossy())
                .expect("failed to open database");
            let engine = PlaybackEngine::new(event_emitter);
            app.manage(AppState {
                engine: Mutex::new(engine),
                storage: Mutex::new(storage),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Phase 2: register command wrappers here
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

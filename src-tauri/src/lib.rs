use rymflux_core::audio::PlaybackEngine;
use rymflux_core::error::CoreError;
use rymflux_core::storage::StorageEngine;
use rymflux_core::types::PlaybackState;
use rymflux_core::EventEmitter;
use std::sync::Arc;
use std::sync::Mutex;
use tauri::Emitter;
use tauri::Manager;

mod commands;

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
    rymflux_core::init_logging();
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::new().build())
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
            let storage =
                StorageEngine::new(&db_path.to_string_lossy()).expect("failed to open database");
            storage
                .run_migrations()
                .expect("failed to run database migrations");
            let engine = PlaybackEngine::new(event_emitter);
            app.manage(Mutex::new(engine));
            app.manage(Mutex::new(storage));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::play_audio,
            commands::pause_audio,
            commands::seek_audio,
            commands::set_audio_speed,
            commands::set_audio_volume,
            commands::get_audio_state,
            commands::stop_audio,
            commands::library_list,
            commands::library_list_domains,
            commands::library_count_content,
            commands::library_count_all,
            commands::library_search,
            commands::library_get_detail,
            commands::progress_get,
            commands::progress_set,
            commands::progress_sync,
            commands::http_get,
            commands::library_store_item,
            commands::library_remove_from,
            commands::library_clear,
            commands::set_diag_mode,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

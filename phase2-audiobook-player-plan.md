# Phase 2: Audiobook Player (Streaming-First) — Execution Plan

## Context

Build a daily-driveable streaming-first LibriVox audiobook player inside the RYMFLUX platform shell. Phase 1 delivered a domain-agnostic kernel (`rymflux-core`) with SQLite storage, Symphonia+Rodio audio engine, Blake3 content identity, typed IPC contracts, and command wrappers. The Tauri v2+Svelte 5 scaffold (`rymflux-desktop`) exists with AppState wired (PlaybackEngine + StorageEngine in Mutex) but empty command handler, stub DesktopEventEmitter, and a stock greet template frontend.

**End state:** A working audiobook player that streams MP3 chapters from LibriVox, persists chapter-accurate progress, allows catalog browsing and personal library management, and meets the 13 exit criteria from `rymflux-internal/docs/sdlc/audiobook-player.md`.

---

## Approach

The plan is ordered so each milestone is independently testable. Steps within a milestone are numbered but should be done in order. The `refs/` directory has Tauri v2 docs, pre-built rustdocs, and the lofty-rs/plugins-workspace repos for reference.

### Step 0: Developer environment setup

This MUST be done first — before any code changes.

#### 0.1 — Set up Tailwind CSS in the Svelte frontend

Install Tailwind CSS v4 in `rymflux-desktop/`:
- `npx @tailwindcss/vite init` or manually add the `@tailwindcss/vite` plugin to `vite.config.js`
- Add Tailwind imports: `@import "tailwindcss"` at the top of `src/app.css` (or create `src/app.css`)
- Ensure all global styles and the dark mode template styling are replaced/removed from `+page.svelte`

Files to touch:
- `rymflux-desktop/vite.config.js` — add `@tailwindcss/vite` plugin
- `rymflux-desktop/src/app.css` — create with `@import "tailwindcss"`
- `rymflux-desktop/src/app.html` — add `<link>` or ensure CSS is loaded (SvelteKit handles `src/app.css` automatically if imported in layout)

#### 0.2 — Set up GitHub Actions CI pipeline

Create `.github/workflows/` in the repo root (or in each workspace crate):

1. `ci-core.yml` — triggered on push/PR touching `rymflux-core/`:
   ```yaml
   cargo check
   cargo clippy -- -D warnings
   cargo test
   ```
2. `ci-desktop.yml` — triggered on push/PR touching `rymflux-desktop/`:
   ```yaml
   cargo check (in src-tauri)
   pnpm install && pnpm check  (in rymflux-desktop/)
   pnpm test (if tests exist)
   ```

File to create:
- `.github/workflows/ci-core.yml`
- `.github/workflows/ci-desktop.yml`

#### 0.3 — Clean up Tauri scaffold remnants

- Remove `greet` command references, unused logos from `+page.svelte`
- Verify `pnpm check` and `cargo check` both pass cleanly after Tailwind install
- Remove the `/public/` or `/static/` demo assets no longer needed

#### 0.4 — Add `reqwest` dependency to desktop Cargo.toml

For the LibriVox API client. Tauri already depends on tokio, so `reqwest` integrates cleanly:

```toml
# rymflux-desktop/src-tauri/Cargo.toml
reqwest = { version = "0.12", features = ["json"] }
```

Also add `tauri-plugin-http` if the frontend needs direct HTTP access (not needed for this phase — all API calls go through Rust backend commands).

---

### Milestone v0.1 — Catalog browsing + streaming playback

This is the core playback milestone. After this, a clickable "search → browse → play" flow works end-to-end.

#### 1.1 — Implement DesktopEventEmitter with Tauri AppHandle

**Problem:** `EventEmitter` trait needs `emit_progress(&self, state: &PlaybackState)` but the current `DesktopEventEmitter` is a unit struct with no access to Tauri's event system.

**Solution:** Restructure `rymflux-desktop/src-tauri/src/lib.rs`:

```rust
use tauri::Emitter;  // Tauri v2 — the trait providing .emit()

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
```

Move all initialization (DB path, StorageEngine, PlaybackEngine) into `tauri::Builder::setup()` where `app.handle()` is available:

```rust
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_handle = Arc::new(app.handle().clone());
            let event_emitter: Arc<dyn EventEmitter> = Arc::new(DesktopEventEmitter { app_handle });
            
            let db_path = dirs::data_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("rymflux")
                .join("rymflux.db");
            // Ensure parent directory exists
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
            // all commands registered here
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

Event names (frontend will `listen()` for these):
- `"audio:progress"` — payload: `PlaybackState`
- `"audio:finished"` — payload: unit
- `"audio:error"` — payload: `String`

#### 1.2 — Create Tauri command wrappers for all core commands

Create `rymflux-desktop/src-tauri/src/commands.rs` with thin `#[tauri::command]` wrappers. Each command:
1. Locks `Mutex<PlaybackEngine>` and/or `Mutex<StorageEngine>` from Tauri state
2. Calls the corresponding `rymflux_core::commands::*` function
3. Maps `CoreError` to `String` via `.map_err(|e| e.to_string())`

Commands to register (exact signatures):

```rust
// ── Playback ──

#[tauri::command]
fn play_audio(
    engine_state: tauri::State<'_, Mutex<PlaybackEngine>>,
    storage_state: tauri::State<'_, Mutex<StorageEngine>>,
    source: AudioSource,
    content_id: String,
    position_ms: u64,
) -> Result<PlaybackState, String> {
    let mut engine = engine_state.inner().lock().map_err(|e| e.to_string())?;
    let storage = storage_state.inner().lock().map_err(|e| e.to_string())?;
    rymflux_core::commands::playback::play(&storage, &mut engine, &source, &content_id, position_ms)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn pause_audio(
    engine_state: tauri::State<'_, Mutex<PlaybackEngine>>,
    storage_state: tauri::State<'_, Mutex<StorageEngine>>,
    domain_id: String,
    content_id: String,
) -> Result<PlaybackState, String> {
    let mut engine = engine_state.inner().lock().map_err(|e| e.to_string())?;
    let storage = storage_state.inner().lock().map_err(|e| e.to_string())?;
    let domain = DomainId::from(domain_id);
    rymflux_core::commands::playback::pause(&storage, &mut engine, &domain, &content_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn seek_audio(
    engine_state: tauri::State<'_, Mutex<PlaybackEngine>>,
    storage_state: tauri::State<'_, Mutex<StorageEngine>>,
    domain_id: String,
    content_id: String,
    position_ms: u64,
) -> Result<PlaybackState, String> {
    let mut engine = engine_state.inner().lock().map_err(|e| e.to_string())?;
    let storage = storage_state.inner().lock().map_err(|e| e.to_string())?;
    let domain = DomainId::from(domain_id);
    rymflux_core::commands::playback::seek(&storage, &mut engine, &domain, &content_id, position_ms)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn set_audio_speed(
    engine_state: tauri::State<'_, Mutex<PlaybackEngine>>,
    rate: f32,
) -> Result<PlaybackState, String> {
    let mut engine = engine_state.inner().lock().map_err(|e| e.to_string())?;
    rymflux_core::commands::playback::set_speed(&mut engine, rate)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn set_audio_volume(
    engine_state: tauri::State<'_, Mutex<PlaybackEngine>>,
    volume: f32,
) -> Result<PlaybackState, String> {
    let mut engine = engine_state.inner().lock().map_err(|e| e.to_string())?;
    Ok(rymflux_core::commands::playback::set_volume(&mut engine, volume))
}

#[tauri::command]
fn get_audio_state(
    engine_state: tauri::State<'_, Mutex<PlaybackEngine>>,
) -> Result<PlaybackState, String> {
    let engine = engine_state.inner().lock().map_err(|e| e.to_string())?;
    Ok(rymflux_core::commands::playback::get_state(&engine))
}

#[tauri::command]
fn stop_audio(
    engine_state: tauri::State<'_, Mutex<PlaybackEngine>>,
    storage_state: tauri::State<'_, Mutex<StorageEngine>>,
    domain_id: String,
    content_id: String,
) -> Result<PlaybackState, String> {
    let mut engine = engine_state.inner().lock().map_err(|e| e.to_string())?;
    let storage = storage_state.inner().lock().map_err(|e| e.to_string())?;
    let domain = DomainId::from(domain_id);
    rymflux_core::commands::playback::stop(&storage, &mut engine, &domain, &content_id)
        .map_err(|e| e.to_string())
}

// ── Library ──

#[tauri::command]
fn library_list(
    storage_state: tauri::State<'_, Mutex<StorageEngine>>,
    domain_id: String,
) -> Result<Vec<ContentItem>, String> {
    let storage = storage_state.inner().lock().map_err(|e| e.to_string())?;
    let domain = DomainId::from(domain_id);
    rymflux_core::commands::library::list(&storage, &domain, None, None)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn library_search(
    storage_state: tauri::State<'_, Mutex<StorageEngine>>,
    domain_id: String,
    query: String,
) -> Result<Vec<ContentItem>, String> {
    let storage = storage_state.inner().lock().map_err(|e| e.to_string())?;
    let domain = DomainId::from(domain_id);
    rymflux_core::commands::library::search(&storage, &domain, &query)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn library_get_detail(
    storage_state: tauri::State<'_, Mutex<StorageEngine>>,
    content_id: String,
) -> Result<ContentItem, String> {
    let storage = storage_state.inner().lock().map_err(|e| e.to_string())?;
    rymflux_core::commands::library::get_detail(&storage, &content_id)
        .map_err(|e| e.to_string())
}

// ── Progress ──

#[tauri::command]
fn progress_get(
    storage_state: tauri::State<'_, Mutex<StorageEngine>>,
    content_id: String,
) -> Result<ProgressRecord, String> {
    let storage = storage_state.inner().lock().map_err(|e| e.to_string())?;
    rymflux_core::commands::progress::get(&storage, &content_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn progress_set(
    storage_state: tauri::State<'_, Mutex<StorageEngine>>,
    domain_id: String,
    content_id: String,
    position_ms: i64,
) -> Result<(), String> {
    let storage = storage_state.inner().lock().map_err(|e| e.to_string())?;
    let domain = DomainId::from(domain_id);
    rymflux_core::commands::progress::set(&storage, &domain, &content_id, position_ms)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn progress_sync(
    storage_state: tauri::State<'_, Mutex<StorageEngine>>,
    domain_id: String,
) -> Result<Vec<ProgressRecord>, String> {
    let storage = storage_state.inner().lock().map_err(|e| e.to_string())?;
    let domain = DomainId::from(domain_id);
    rymflux_core::commands::progress::sync(&storage, &domain)
        .map_err(|e| e.to_string())
}
```

Add `mod commands;` to `lib.rs` and list every command in `tauri::generate_handler![]`.

**Note on already-imported types:** `PlaybackState` is already imported in lib.rs. Add imports for `DomainId`, `ContentItem`, `ProgressRecord`, `AudioSource`.

#### 1.3 — Create LibriVox API client (Rust module)

Create `rymflux-desktop/src-tauri/src/librivox.rs` — a stateless API client using `reqwest`.

**API response structs** (serde `Deserialize`):

```rust
#[derive(Deserialize)]
struct LibrivoxListResponse {
    books: Vec<LibrivoxBook>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
struct LibrivoxBook {
    id: String,
    title: String,
    description: String,
    url_text_source: Option<String>,
    language: Option<String>,
    copyright_year: Option<String>,
    num_sections: Option<String>,
    url_zip_file: Option<String>,
    url_librivox: Option<String>,
    totaltime: Option<String>,
    totaltimesecs: Option<i64>,
    authors: Vec<LibrivoxAuthor>,
    // Extended fields (when extended=1)
    url_iarchive: Option<String>,
    sections: Option<Vec<LibrivoxSection>>,
}

#[derive(Deserialize)]
struct LibrivoxAuthor {
    id: String,
    first_name: String,
    last_name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
struct LibrivoxSection {
    id: String,
    section_number: String,
    title: String,
    listen_url: String,
    playtime: Option<String>,  // seconds as string
    language: Option<String>,
    readers: Vec<LibrivoxReader>,
}

#[derive(Deserialize)]
struct LibrivoxReader {
    reader_id: String,
    display_name: String,
}
```

**Public functions:**

```rust
/// Search by title (uses `^` prefix for anchored search).
pub async fn search_by_title(query: &str, limit: u32, offset: u32) -> Result<Vec<LibrivoxBook>, String>

/// Search by author last name.
pub async fn search_by_author(author: &str, limit: u32, offset: u32) -> Result<Vec<LibrivoxBook>, String>

/// Get book details with all sections (chapters).
pub async fn get_book(id: &str) -> Result<LibrivoxBook, String>

/// Convert a LibrivoxBook to the domain's ContentItem format.
pub fn book_to_content_item(book: &LibrivoxBook) -> ContentItem

/// Get the base URL for chapter streaming.
pub fn get_stream_url(section: &LibrivoxSection) -> String  // returns section.listen_url
```

Base URL: `https://librivox.org/api/feed/audiobooks`

The `reqwest::Client` should be cheaply cloneable and shared. Store it in Tauri managed state or create a fresh one per request. For simplicity, create a module-level `lazy_static` or `OnceLock<Client>` — a single `Client` instance reused.

**Search URL patterns:**
- By title (anchored): `https://librivox.org/api/feed/audiobooks?title=%5E{query}&format=json&limit={n}&offset={o}`
- By author: `https://librivox.org/api/feed/audiobooks?author={last_name}&format=json&limit={n}&offset={o}`
- By ID (extended): `https://librivox.org/api/feed/audiobooks/id/{id}?extended=1&format=json`

#### 1.4 — Create Tauri commands for LibriVox catalog operations

Add to `rymflux-desktop/src-tauri/src/commands.rs`:

```rust
#[tauri::command]
async fn catalog_search(
    query: String,
    limit: Option<u32>,
    offset: Option<u32>,
) -> Result<Vec<CatalogItem>, String> {
    // Search LibriVox API, convert to CatalogItem for frontend
    let books = librivox::search_by_title(&query, limit.unwrap_or(20), offset.unwrap_or(0)).await?;
    Ok(books.into_iter().map(|b| b.into()).collect())
}

#[tauri::command]
async fn catalog_get_book(id: String) -> Result<CatalogDetail, String> {
    let book = librivox::get_book(&id).await?;
    Ok(book.into())
}

// Frontend-facing catalog types (not core types — these are domain types)
#[derive(Serialize, Deserialize)]
struct CatalogItem {
    id: String,
    title: String,
    author: String,
    description: String,
    total_time_secs: Option<i64>,
    num_sections: Option<u32>,
    cover_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct CatalogDetail {
    item: CatalogItem,
    sections: Vec<ChapterInfo>,
}

#[derive(Serialize, Deserialize)]
struct ChapterInfo {
    id: String,
    section_number: u32,
    title: String,
    listen_url: String,
    playtime_secs: Option<u64>,
}
```

Also add a command to add a catalog book to the local library:

```rust
#[tauri::command]
async fn library_add_from_catalog(
    storage_state: tauri::State<'_, Mutex<StorageEngine>>,
    catalog_id: String,
) -> Result<(), String> {
    // 1. Fetch book details from LibriVox API
    // 2. Create ContentItem with source_uri = catalog listen URL
    // 3. Store identity (source_id = librivox_{id})
    // 4. Insert content item into storage
    // All storage operations through existing StorageEngine
}
```

#### 1.5 — Frontend: Create IPC bridge layer

Create the frontend TypeScript files:

**`src/lib/types/ipc.ts`** — matching Core's IPC types:

```typescript
export interface AudioSource {
  uri: string;
  duration_ms: number;
  mime_type: string;
}

export interface PlaybackState {
  position_ms: number;
  duration_ms: number;
  speed: number;
  volume: number;
  is_playing: boolean;
  is_loaded: boolean;
}

export interface ContentIdentity {
  identity_id: string;
  source_id: string | null;
  file_name: string | null;
  domain_id: string;
  // etc.
}

export interface ProgressRecord {
  domain_id: string;
  content_id: string;
  position_ms: number;
  extra: Record<string, unknown>;
  updated_at: string;
}

export interface CatalogItem {
  id: string;
  title: string;
  author: string;
  description: string;
  total_time_secs: number | null;
  num_sections: number | null;
  cover_url: string | null;
}

export interface ChapterInfo {
  id: string;
  section_number: number;
  title: string;
  listen_url: string;
  playtime_secs: number | null;
}

export interface CatalogDetail {
  item: CatalogItem;
  sections: ChapterInfo[];
}
```

**`src/lib/ipc/audioEngine.ts`** — TauriAudioEngine class:

```typescript
import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import type { AudioSource, PlaybackState } from '$lib/types/ipc';

export class TauriAudioEngine {
  private _unlisten: UnlistenFn[] = [];

  async init(onProgress: (s: PlaybackState) => void, onFinished: () => void, onError: (e: string) => void) {
    this._unlisten.push(
      await listen<PlaybackState>('audio:progress', (e) => onProgress(e.payload)),
      await listen<void>('audio:finished', () => onFinished()),
      await listen<string>('audio:error', (e) => onError(e.payload)),
    );
  }

  destroy() { this._unlisten.forEach(u => u()); this._unlisten = []; }

  play(source: AudioSource, contentId: string, positionMs: number): Promise<PlaybackState> {
    return invoke('play_audio', { source, contentId, positionMs });
  }
  pause(domainId: string, contentId: string): Promise<PlaybackState> {
    return invoke('pause_audio', { domainId, contentId });
  }
  seek(domainId: string, contentId: string, positionMs: number): Promise<PlaybackState> {
    return invoke('seek_audio', { domainId, contentId, positionMs });
  }
  setSpeed(rate: number): Promise<PlaybackState> {
    return invoke('set_audio_speed', { rate });
  }
  setVolume(volume: number): Promise<PlaybackState> {
    return invoke('set_audio_volume', { volume });
  }
  getState(): Promise<PlaybackState> {
    return invoke('get_audio_state');
  }
  stop(domainId: string, contentId: string): Promise<PlaybackState> {
    return invoke('stop_audio', { domainId, contentId });
  }
}
```

**`src/lib/ipc/catalog.ts`**:

```typescript
import { invoke } from '@tauri-apps/api/core';
import type { CatalogItem, CatalogDetail } from '$lib/types/ipc';

export async function searchCatalog(query: string, limit?: number, offset?: number): Promise<CatalogItem[]> {
  return invoke('catalog_search', { query, limit, offset });
}
export async function getBook(id: string): Promise<CatalogDetail> {
  return invoke('catalog_get_book', { id });
}
export async function addToLibrary(catalogId: string): Promise<void> {
  return invoke('library_add_from_catalog', { catalogId });
}
```

**`src/lib/ipc/library.ts`**:

```typescript
import { invoke } from '@tauri-apps/api/core';
import type { ContentItem, ProgressRecord } from '$lib/types/ipc';

export async function listLibrary(domainId: string): Promise<ContentItem[]> {
  return invoke('library_list', { domainId });
}
export async function searchLibrary(domainId: string, query: string): Promise<ContentItem[]> {
  return invoke('library_search', { domainId, query });
}
export async function getProgress(contentId: string): Promise<ProgressRecord> {
  return invoke('progress_get', { contentId });
}
export async function setProgress(domainId: string, contentId: string, positionMs: number): Promise<void> {
  return invoke('progress_set', { domainId, contentId, positionMs });
}
export async function syncProgress(domainId: string): Promise<ProgressRecord[]> {
  return invoke('progress_sync', { domainId });
}
```

#### 1.6 — Frontend: Create stores

**`src/lib/stores/playerStore.ts`**:

```typescript
import type { PlaybackState, AudioSource } from '$lib/types/ipc';

let positionMs = $state(0);
let durationMs = $state(0);
let speed = $state(1.0);
let volume = $state(1.0);
let isPlaying = $state(false);
let isLoaded = $state(false);
let currentSource = $state<AudioSource | null>(null);
let currentContentId = $state<string | null>(null);
let currentDomainId = $state<string>('audiobook');

// Derived
let remainingMs = $derived(durationMs - positionMs);
let progressFraction = $derived(durationMs > 0 ? positionMs / durationMs : 0);

export function getPlayerState() {
  return {
    get positionMs() { return positionMs; },
    get durationMs() { return durationMs; },
    get speed() { return speed; },
    get volume() { return volume; },
    get isPlaying() { return isPlaying; },
    get isLoaded() { return isLoaded; },
    get currentSource() { return currentSource; },
    get currentContentId() { return currentContentId; },
    get currentDomainId() { return currentDomainId; },
    get remainingMs() { return remainingMs; },
    get progressFraction() { return progressFraction; },
  };
}

export function updatePlaybackState(s: PlaybackState) {
  positionMs = s.position_ms;
  durationMs = s.duration_ms;
  speed = s.speed;
  volume = s.volume;
  isPlaying = s.is_playing;
  isLoaded = s.is_loaded;
}

export function setCurrentTrack(source: AudioSource, contentId: string) {
  currentSource = source;
  currentContentId = contentId;
}
```

**`src/lib/stores/uiStore.ts`**:

```typescript
let sidebarOpen = $state(true);
let theme = $state<'light' | 'dark'>('dark');
let viewMode = $state<'grid' | 'list'>('grid');

export function getUiState() {
  return {
    get sidebarOpen() { return sidebarOpen; },
    set sidebarOpen(v: boolean) { sidebarOpen = v; },
    get theme() { return theme; },
    get viewMode() { return viewMode; },
    set viewMode(v: 'grid' | 'list') { viewMode = v; },
  };
}
```

**`src/lib/stores/progressStore.ts`** — manages progress heartbeat and resume coordination. On mount: loads last progress for each content item in the library and exposes a "continue listening" list.

```typescript
// Uses Tauri invoke('progress_sync') to sync on startup
// Stores last-played list
```

#### 1.7 — Frontend: Shell layout + routing

**ShellLayout** (`src/routes/+layout.svelte`):

```
+------------------+----------------------------+
|   Sidebar        |    Content area (slot)     |
|   ─────────      |                            |
|   Library        |    Route-driven content     |
|   Search         |                            |
|   Settings       |                            |
+------------------+----------------------------+
|   PlayerBar (MiniPlayer + controls)           |
+-----------------------------------------------+
```

Three-zone layout using CSS grid:
- **Sidebar** — fixed width (240px), collapsible via hamburger. Contains nav links.
- **Content** — flex-grow, scrollable. Renders `{@render children()}`.
- **PlayerBar** — fixed height (64px), visible when content is loaded. Contains MiniPlayer and PlaybackControls.

**Routes** (SvelteKit file-based):
- `/` — Dashboard (continue listening)
- `/search` — Search LibriVox catalog
- `/library` — Personal library
- `/library/[domain]` — Domain-specific library (currently only `audiobook`)
- `/player/[contentId]` — NowPlaying view
- `/settings` — Settings pane

Create route files:
- `src/routes/+layout.svelte` — ShellLayout (import Sidebar, PlayerBar)
- `src/routes/+page.svelte` — Home/Dashboard
- `src/routes/search/+page.svelte` — Catalog search
- `src/routes/library/+page.svelte` — Library overview
- `src/routes/library/[domain]/+page.svelte` — Domain library grid
- `src/routes/player/[contentId]/+page.svelte` — NowPlaying
- `src/routes/settings/+page.svelte` — Settings

Each route uses `+page.ts` load function to `invoke()` data from the backend.

#### 1.8 — Frontend: Shared UI components

Create `src/lib/components/`:

- **ShellLayout.svelte** — three-zone grid
- **Sidebar.svelte** — nav links (Library, Search, Settings), domain list
- **PlayerBar.svelte** — MiniPlayer + PlaybackControls + SeekBar
- **SeekBar.svelte** — clickable/draggable progress bar, input type="range"
- **TimeDisplay.svelte** — formatted elapsed / remaining time (HH:MM:SS)
- **PlaybackControls.svelte** — play/pause, skip -30s, skip +15s, speed selector dropdown
- **VolumeSlider.svelte** — slider 0–1 with mute toggle
- **CoverImage.svelte** — book cover art with fallback (SVG placeholder)
- **LoadingSpinner.svelte** — spinner component
- **ErrorBoundary.svelte** — Svelte 5 error boundary (uses `$effect` or `onMount` error catching)

#### 1.9 — Create LibraryView domain component

`src/domains/audiobook/LibraryView.svelte`:

- Props: `domainId: string`, `items: DomainItem[]`, `onSelect: (item: DomainItem) => void`
- Grid mode (cards with cover art, title, author, progress badge)
- List mode (table with thumbnail, title, author, duration, progress)
- View mode toggle button (grid ↔ list)
- Search input (debounced 300ms, filters items by title/author)
- Empty state: "Your library is empty. Browse the LibriVox catalog to add audiobooks." with Browse button
- Responsive grid breakpoints per spec

**Data flow:**
1. Route's `+page.ts` calls `invoke('library_list', { domainId })`
2. LibraryView receives items as `$props()`
3. User clicks a card → calls `onSelect`
4. App navigates to `/player/[contentId]`

#### 1.10 — Create DetailView domain component

`src/domains/audiobook/DetailView.svelte`:

- Props: `item: DomainItem`, `onPlay: () => void`
- Header: Cover art (large), title, author, narrator, total duration
- Progress: "Continue" button (seeks to saved position with 3s rewind) or "Start from beginning"
- Chapters: scrollable list showing title, duration, play icon — clicking plays from that chapter
- Metadata: publisher (LibriVox), language, description
- Actions: "Play All", "Add to Library" / "Remove from Library"
- The `onPlay` callback should be called with chapter info when a specific chapter is selected

#### 1.11 — Create NowPlayingView domain component + MiniPlayer

**`src/domains/audiobook/NowPlayingView.svelte`**:

- Props: `item: DomainItem`, `state: PlaybackState`
- Large cover art (centered)
- Title + author
- Seek bar (clickable/draggable)
- Playback controls: play/pause, skip -30s, skip +15s, speed selector
- Chapter list: current chapter highlighted, click to jump
- Sleep timer: 15/30/45/60 min, end of chapter, custom
- Volume slider

**`src/domains/audiobook/MiniPlayer.svelte`**:

- Props: `state: PlaybackState`
- Compact bar: cover thumbnail (32×32), chapter title (truncated), thin progress bar, play/pause button
- Click to expand → navigate to NowPlaying view

#### 1.12 — Wire up frontend to backend

In `src/routes/+layout.svelte`:

```typescript
import { TauriAudioEngine } from '$lib/ipc/audioEngine';
import { updatePlaybackState } from '$lib/stores/playerStore';

let engine = $state<TauriAudioEngine | null>(null);

onMount(() => {
  engine = new TauriAudioEngine();
  engine.init(
    (s) => updatePlaybackState(s),
    () => { /* handle finished */ },
    (e) => console.error(e),
  );
  return () => engine?.destroy();
});
```

The `playerStore` is shared across components via import — no context/prop drilling needed because Svelte 5 runes track dependencies across modules.

**Verification at this point:** `pnpm dev` + `cargo tauri dev` — search LibriVox for a book, view details, click play. Audio streams from the listen_url, progress bar updates every 250ms via events, pause/seek/speed work.

---

### Milestone v0.2 — Library management

Builds on v0.1 to persist a personal library with add/remove, grid/list views, search, and sort.

#### 2.1 — Backend: Library add/remove operations

Add Tauri commands:

```rust
#[tauri::command]
async fn library_remove_from(
    state: tauri::State<'_, Mutex<StorageEngine>>,
    content_id: String,
) -> Result<(), String> {
    let storage = state.inner().lock().map_err(|e| e.to_string())?;
    storage.delete_content(&content_id).map_err(|e| e.to_string())
}
```

The `library_add_from_catalog` (from 1.4) already handles adding. Extend it to:
1. Check if content already exists (by source_id)
2. If not, fetch from API, create ContentItem + ContentIdentity, store both
3. Return the new content_id

#### 2.2 — Frontend: Library grid and list views

The `LibraryView.svelte` (1.9) already handles both view modes. 

Route data loading:
- `src/routes/library/[domain]/+page.ts`:
  ```typescript
  import { invoke } from '@tauri-apps/api/core';
  export async function load({ params }) {
    const items = await invoke('library_list', { domainId: params.domain });
    return { items, domainId: params.domain };
  }
  ```

- `src/routes/library/+page.svelte`: Show domain tabs (currently only "Audiobooks"), render LibraryView for the active domain
- `src/routes/library/[domain]/+page.svelte`: Render LibraryView for that domain with items from load function

#### 2.3 — Frontend: Add/remove button wiring

- **DetailView**: "Add to Library" / "Remove from Library" button calls `invoke('library_add_from_catalog', ...)` or `invoke('library_remove_from', ...)`
- **LibraryView**: Context menu or delete button per item
- Both trigger a re-fetch of the library list after mutation

#### 2.4 — Frontend: Search within library

LibraryView search input filters via `invoke('library_search', { domainId, query })`. Debounce 300ms.

#### 2.5 — Frontend: Sortable list columns

List mode: clicking column headers toggles sort direction. Implement with a local `$state` for sort field/direction, sort the items array before rendering.

**Verification:** Add a book to library, see it appear in grid/list views. Remove it, see it disappear. Search narrows results. Sort works. Library persists across app restart.

---

### Milestone v0.3 — UI polish

#### 3.1 — Sleep timer

Implement in `NowPlayingView.svelte`:

```typescript
let sleepTimer = $state<{ endTime: number } | null>(null);
let sleepOption = $state<'none' | 15 | 30 | 45 | 60 | 'chapter'>('none');

$effect(() => {
  if (sleepTimer && Date.now() >= sleepTimer.endTime) {
    // Pause playback
    engine?.pause(domainId, contentId);
    sleepTimer = null;
  }
});

function startSleepTimer(minutes: number) {
  sleepTimer = { endTime: Date.now() + minutes * 60 * 1000 };
}
```

For "end of chapter": when `audio:finished` fires and sleep timer is set to 'chapter', pause instead of advancing.

#### 3.2 — Continue Listening dashboard

`src/routes/+page.svelte` (Home/Dashboard):
- On mount, call `invoke('progress_sync', { domainId: 'audiobook' })` to get in-progress books
- Filter for items where `position_ms > 0` and `position_ms < duration_ms`
- Display as horizontal scrollable cards: cover art, title, progress bar, "Continue" button
- Tap "Continue" → resolve content identity → seek to `max(0, position_ms - 3000)` → navigate to NowPlaying

#### 3.3 — Dark mode

Tailwind dark mode is default (the app targets a dark desktop player). Set `class="dark"` on `<html>` in `app.html` and use Tailwind's `dark:` variants throughout.

No toggle needed yet — the app always starts in dark mode per the current CSS in `+page.svelte`.

#### 3.4 — Cover art display

LibriVox API provides cover art via `coverart=1` parameter:
- `https://librivox.org/api/feed/audiobooks?id={id}&format=json&coverart=1`
- Returns `coverart_thumbnail` and `coverart_jpg` fields

Update the `LibrivoxBook` struct and the API client to request cover art. Display in LibraryView cards and DetailView header.

`CoverImage.svelte`: Shows image if available, otherwise a book-cover SVG placeholder with first letter of title.

---

### Milestone v0.4 — Daily-driver ready

#### 4.1 — Speed memory per content

When setting speed in NowPlayingView, save to `extra_json` in progress record:
```typescript
await invoke('progress_set', {
  domainId: 'audiobook',
  contentId,
  positionMs: currentPosition,
  // extra is not settable via progress_set currently — needs extension
});
```

**Backend change:** Add an `extra: Option<serde_json::Value>` parameter to `progress_set` command so frontend can pass chapter index and speed settings.

Actually, the core `commands::progress::set` already creates a `ProgressRecord` with `extra: Default::default()`. **Change needed:** Expose `extra` as an optional parameter so the frontend can pass chapter context and speed preference.

Update `progress_set` command:
```rust
#[tauri::command]
fn progress_set(
    storage_state: tauri::State<'_, Mutex<StorageEngine>>,
    domain_id: String,
    content_id: String,
    position_ms: i64,
    extra: Option<serde_json::Value>,
) -> Result<(), String> {
    let storage = storage_state.inner().lock().map_err(|e| e.to_string())?;
    let domain = DomainId::from(domain_id);
    let progress = ProgressRecord {
        domain_id: domain,
        content_id,
        position_ms,
        extra: extra.unwrap_or(serde_json::Value::Object(Default::default())),
        updated_at: chrono::Utc::now().to_rfc3339(),
    };
    storage.set_progress(&progress).map_err(|e| e.to_string())
}
```

This change should be made directly in the `progress_set` Tauri command, not in core's `commands::progress::set` (which is fine as-is for simple uses). The command-level function creates the `ProgressRecord` directly.

#### 4.2 — Window persistence

Tauri v2 saves window size/position natively. No extra work needed unless custom behavior is desired. If needed, use Tauri's `Window::set_size`/`Window::set_position` at startup from settings store.

#### 4.3 — Keyboard shortcuts

Register in `+layout.svelte`:
- `Space` — play/pause
- `←` / `→` — skip back/forward
- `+` / `-` — volume up/down
- `M` — mute toggle
- `[` / `]` — speed down/up (0.05 increments)

Implementation: `$effect` registers a `keydown` listener on the window, calling the appropriate engine method.

#### 4.4 — Developer dashboard

`src/routes/settings/+page.svelte`:
- View engine state (is_playing, position, source URL, volume, speed)
- View storage stats (number of domains, content items, progress entries)
- LibriVox API request log (last N requests with response times)
- Clear library button
- Export/import progress (calls `progress_sync` and writes JSON to disk via Tauri save dialog — deferred to a future iteration if complex)

#### 4.5 — Catalog search improvements

Current search uses LibriVox `title` anchored search. Add support for:
- Author search toggle (search by author last name)
- Search history (local, last 10 queries)
- Visual indicator while searching (loading spinner)
- Pagination in search results (load more / next page)

---

## Edge cases & failure handling

| Scenario | Handling |
|----------|----------|
| LibriVox API unreachable | Catalog search returns error string; frontend shows "Catalog unavailable. Check your internet connection." |
| Chapter URL fails to stream | PlaybackEngine emits `audio:error`; progress stops; UI shows "Playback error" with retry button |
| App crash during playback | Progress saved on pause/stop/seek; on resume, last saved position used with 3s rewind |
| Item removed from LibriVox catalog | Content still in local library but stream URLs return 404; playback fails with clear error |
| No network at launch | Library works offline (metadata cached); catalog search fails; user sees offline indicator |
| Speed set to extreme values | Backend clamps to 0.5x–4.0x range in PlaybackEngine; frontend slider caps at bounds |
| Double-click play | Already-playing detection in PlaybackEngine; second play call with same source is no-op or restarts |
| Sleep timer fires mid-seek | Timer checked on next progress event after seek settles — no race |
| Very long chapter (>10 hrs) | No issue — Symphonia streams and ring buffer never fills; position tracking via sample count has no overflow because AtomicU64 tracks samples, not ms |
| Chapter with no readers/unknown narrator | Show "Unknown" in detail view |

---

## Critical files & anchors

| File | Region | Why |
|------|--------|-----|
| `rymflux-desktop/src-tauri/src/lib.rs` | `run()` + `DesktopEventEmitter` | Core scaffold rewrite — move init to `.setup()`, wire AppHandle into emitter, register all commands |
| `rymflux-desktop/src-tauri/src/commands.rs` | _entire file, new_ | All Tauri command wrappers — thin bridge to core commands |
| `rymflux-desktop/src-tauri/src/librivox.rs` | _entire file, new_ | LibriVox API client — HTTP calls, response deserialization, ContentItem conversion |
| `rymflux-core/src/ipc/playback.rs` | `PlayParams`, `SeekParams` | IPC contract structs — match frontend's `invoke()` argument shapes |
| `rymflux-core/src/commands/playback.rs` | `play()`, `pause()`, `seek()`, `stop()` | Core orchestrators that Tauri commands delegate to |
| `rymflux-core/src/storage/engine.rs` | `upsert_content()`, `delete_content()`, `resolve_by_source()` | Storage operations used by library add/remove and identity resolution |
| `rymflux-desktop/src/routes/+layout.svelte` | _entire file, new_ | ShellLayout — 3-zone grid, playerStore init, keyboard shortcuts |
| `rymflux-desktop/src/domains/audiobook/LibraryView.svelte` | _entire file, new_ | Grid/list library browser — most complex UI component |
| `rymflux-desktop/src/domains/audiobook/NowPlayingView.svelte` | _entire file, new_ | Full player view with chapter nav, sleep timer, speed/volume controls |

---

## Verification

**Smoke test (after v0.1):**
```bash
# Terminal 1: Tauri dev
cd rymflux-desktop && cargo tauri dev

# Manual test flow:
# 1. App opens with dark sidebar + empty content area
# 2. Click "Search" → enter "pride" → see Pride and Prejudice results
# 3. Click a result → see DetailView with chapters, cover art, description
# 4. Click "Play" → audio streams from first chapter, progress bar moves
# 5. Click pause → audio stops, progress saved
# 6. Click seek bar → position jumps, audio resumes
# 7. Change speed to 1.5x → audio speeds up
# 8. Close app → reopen → click "Continue Listening" → resumes at saved position with 3s rewind
```

**CI verification (always):**
```bash
cd rymflux-core && cargo test && cargo clippy -- -D warnings
cd rymflux-desktop && cargo check && pnpm check
```

**Library test (after v0.2):**
1. Click "Add to Library" on a book → appears in Library grid
2. Switch to list view → see sortable columns
3. Search in library → narrows results
4. Remove a book → disappears
5. Close and reopen app → library still shows added books

**Chapter-accurate resume test:**
1. Play a book to chapter 3, position 5min → note chapter and position
2. Close app (force quit)
3. Reopen → Continue Listening shows the book
4. Tap Continue → seeks to chapter 3 at 5:00 - 3s → plays

---

## Assumptions & contingencies

| Assumption | If false → fallback |
|------------|---------------------|
| LibriVox API remains available with current response format | Add a caching layer that stores last-fetched catalog data; fall back to cache on API failure. Cache TTL: 1 hour. |
| Rodio seek requires re-creating sink (current core behavior) | If a future rodio version supports hot-swap, simplify seek in PlaybackEngine — no change to Tauri commands needed. |
| Tauri v2 `app.emit()` works from a background progress thread (spawned by PlaybackEngine) | It does — `Emitter` trait methods are thread-safe. If thread-safety becomes an issue, use `app.emit_any()` or pass events through a channel. |
| SvelteKit adapter-static with prerender=true is correct for Tauri | Confirmed by Tauri docs and current config — SSG, no SSR, all routing via SPA client-side navigation once loaded. |
| All LibriVox audio chapters are MP3 files (supported by Symphonia) | Confirmed by API response: `listen_url` is `.mp3` (64kb/s). Symphonia `"mp3"` feature is already enabled in core. |
| User's data dir is writable | `dirs::data_dir()` fallback to `.` already in code. `create_dir_all` added in setup. |

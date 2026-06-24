# RYMFLUX Desktop

A **content-type-agnostic audio platform** — the "Stremio of Audio." Plugin-like domains define content models, UI views, and progress semantics. The kernel (`rymflux-core`) is blind to content types; the desktop shell is a neutral host.

Currently in **Phase 2** — building a daily-driveable streaming audiobook player (LibriVox) as the reference domain.

## Architecture

```
rymflux-core (Rust kernel via Tauri invoke)
├── Audio engine      — Play/Pause/Seek/Speed/Volume
├── Storage (SQLite)  — Domain-agnostic tables (content, identities, progress)
├── Identity system   — Source-ID-based resolution (Blake3)
└── EventEmitter      — Progress/finished/error events

rymflux-desktop (Tauri/Svelte shell)
├── ShellLayout       — Sidebar | Content | PlayerBar
├── Sidebar           — Nav: Home, Search, Library, Settings
├── PlayerBar         — Persistent bottom bar with controls
├── Route dispatcher  — Direct imports from domain modules
├── Stores            — playerStore (playback state), uiStore (preferences)
└── IPC modules       — Tauri command wrappers: audioEngine, catalog, library

domains/audiobook/    — Reference domain for Phase 2
├── LibraryView       — Grid/list library browser
├── DetailView        — Cover, metadata, chapters, play/add/remove
├── NowPlayingView    — Full player with chapter nav, sleep timer, speed
└── MiniPlayer        — Compact bar format
```

## Prerequisites

- **Rust toolchain** (stable) — [rustup.rs](https://rustup.rs)
- **Node.js** 20+
- **pnpm** — `npm install -g pnpm`
- **System deps** (Linux) — `libwebkit2gtk-4.1-dev`, `libgtk-3-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`
- **System deps** (macOS) — Xcode Command Line Tools
- **System deps** (Windows) — Visual Studio Build Tools + WebView2

## Development

```bash
# Install frontend dependencies
pnpm install

# Type-check the frontend
pnpm check

# Build the frontend
pnpm build

# Check Rust backend
cd src-tauri && cargo check

# Run Rust tests
cargo test

# Lint Rust
cargo clippy -- -D warnings

# Run full Tauri app (dev mode)
pnpm run tauri dev
```

## Project Structure

```
src/
├── lib/
│   ├── components/     — Reusable shell components (PlayerBar, SeekBar, etc.)
│   ├── ipc/            — Tauri command wrappers (audioEngine, catalog, library)
│   ├── stores/         — Svelte rune stores (playerStore, uiStore)
│   └── types/          — Shared TypeScript types matching Rust IPC
├── routes/             — SvelteKit file-based routing
├── domains/
│   └── audiobook/      — Audiobook domain views
src-tauri/
├── src/
│   ├── commands.rs     — Tauri command handlers
│   ├── librivox.rs     — LibriVox API client
│   └── lib.rs          — App setup + command registration
└── Cargo.toml          — Rust dependencies (depends on rymflux-core)
```

## Domain Architecture

Domains are compiled-in during Phase 2 (dynamic plugin loading is Phase 5+). Each domain provides its own views (LibraryView, DetailView, NowPlayingView) which the shell imports directly. The typed slot abstraction (SlotRenderer, DomainManifest, domainStore) is deferred until a second domain exists — per "Extract, don't design."

## SDLC Milestones

| Phase | Scope |
|-------|-------|
| **v0.1** | Catalog browsing + streaming playback. Search LibriVox, view details, chapter navigation, play/pause/seek/speed. Position resume with 3s rewind. |
| **v0.2** | Library management. Add/remove books, grid/list views, search, sort, persistence. |
| **v0.3** | UI polish. NowPlaying chapter nav, sleep timer, speed selector, continue listening dashboard. |
| **v0.4** | Daily-driver. Speed memory, window persistence, keyboard shortcuts, dev dashboard. |

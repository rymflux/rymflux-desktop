# Agent Context

## Quick Start

Before implementing anything:

1. Read `PLAN.md` for the step's expected shape
2. Verify APIs against the source of truth (see below) — the plan may be aspirational
3. Implement
4. Append entry to `IMPLEMENTATION_JOURNAL.md` with any divergences, discoveries, or lessons
5. Run verification commands (see Build & Verify)

---

## Planning & Documentation

- **`PLAN.md`** — Master  plan. Contains architecture, DDL, API surfaces, and test requirements. **Consult this first** before any implementation step. When the plan says X but the real API says Y, fix the plan with a `⚠️` callout so it stays accurate.
- **`IMPLEMENTATION_JOURNAL.md`** — Chronological log of plan divergences, API corrections, architecture decisions, and testing discoveries. Append to this after each completed step. Use it to avoid repeating mistakes and to know which parts of the plan are stale.

⏰ **After finishing each step, append to `IMPLEMENTATION_JOURNAL.md`** even if no divergences occurred — a "no issues found" entry is still useful for future agents.

---

## Frontend Architecture

### Tech stack
- **Svelte 5** with runes (`$state`, `$derived`, `$effect`)
- **SvelteKit** routing (file-based, `+page.svelte`, `+layout.svelte`, `+page.ts`)
- **Tailwind CSS v4** via `@tailwindcss/vite` plugin
- **TypeScript** in all `.svelte` files (use `<script lang="ts">`)

### Path alias
- `$src` maps to `./src` (configured in `svelte.config.js` via `kit.alias`)
- Example: `import DetailView from '$src/domains/audiobook/DetailView.svelte'`

### .svelte.ts convention
Files using Svelte 5 runes (`$state`, `$derived`, etc.) outside `.svelte` files must use the `.svelte.ts` extension (e.g., `playerStore.svelte.ts`, `uiStore.svelte.ts`). This tells SvelteKit's compiler to process them.

### Layout hierarchy
```
+layout.svelte (root)
  └── ShellLayout.svelte (3-zone grid: Sidebar | <main> | PlayerBar)
       ├── Sidebar.svelte (nav links)
       ├── <main> — route content ({@render children()})
       │    └── ErrorBoundary.svelte (wraps route content)
       └── PlayerBar.svelte (persistent bottom bar when audio is loaded)
            ├── Mini-info section (cover thumbnail, title, progress bar, clickable nav)
            ├── SeekBar + TimeDisplay (position / remaining)
            ├── PlaybackControls (play/pause, skip, speed)
            └── VolumeSlider
```

### Control flow
- `+layout.svelte` defines all handler functions (`handlePlayPause`, `handleSeek`, etc.)
- These are passed as props through `ShellLayout` → `PlayerBar`
- The `TauriAudioEngine` instance is set via `setAudioEngine()` (Svelte context, keyed by `Symbol('audioEngine')`)
- Components that need the engine call `getAudioEngine()` from `$lib/ipc/engineContext`
- `playerStore.svelte.ts` holds reactive playback state; `updatePlaybackState(s)` is called on every `audio:progress` event

### Stores
| Store | Purpose | Key fields |
|-------|---------|------------|
| `playerStore.svelte.ts` | Playback state | `positionMs`, `durationMs`, `speed`, `volume`, `isPlaying`, `isLoaded`, `currentSource`, `currentContentId`, `currentTitle`, `currentCoverUrl`, `currentChapterIndex`, `currentSections` |
| `uiStore.svelte.ts` | UI preferences | `sidebarOpen`, `viewMode` |

### IPC layers
File naming reflects domain:
| File | Backend commands |
|------|-----------------|
| `$lib/ipc/audioEngine.ts` | `TauriAudioEngine` class — wraps all playback `invoke` calls + event listeners |
| `$lib/ipc/catalog.ts` | `catalog_search`, `catalog_get_book`, `addToLibrary`, `resolveSource` |
| `$lib/ipc/library.ts` | `listLibrary`, `searchLibrary`, `getLibraryDetail`, `removeFromLibrary`, progress CRUD |
| `$lib/types/ipc.ts` | Shared TypeScript types mirroring Rust IPC types |

### Domains
Domain-specific components live in `src/domains/<domain>/`:
- `audiobook/DetailView.svelte` — Book detail page (cover, chapters, play/add/remove buttons)
- `audiobook/NowPlayingView.svelte` — Full player view with chapter nav, sleep timer
- `audiobook/LibraryView.svelte` — Grid/list library browser with search, sort, progress indicators

### Error boundaries
Svelte 5 has a **built-in `<svelte:boundary>`** element (verified at `opensrc/.../svelte/src/internal/client/dom/blocks/boundary.js`). It supports:
- `onerror={(error, reset) => void}` — callback for logging/side effects
- `failed={snippet}` — fallback UI snippet (rendered on error)
- `pending={snippet}` — loading state for async content

Use `<svelte:boundary>` instead of implementing custom error catching via window events.

---

## Build & Verify

### Rust backend
```bash
# Compile check (fast, skips linking)
cargo check

# Full test
cargo test

# Clippy (must pass before PR)
cargo clippy -- -D warnings

# Regenerate rustdoc
cargo doc
```

**When to run each:**
- `cargo check` after any Rust or Tauri command change
- `cargo test` when modifying Rust business logic
- `cargo clippy -- -D warnings` before submitting changes
- `cd src-tauri && cargo check` if running from project root fails (Cargo.toml is in src-tauri/)
- For Tauri command/permission/plugin/config changes that affect the full app, run `pnpm run tauri build`

### Frontend (Svelte/TypeScript)
```bash
# Type check (uses pnpm, NOT npm)
pnpm run check

# Build
pnpm run build

# Lint
pnpm dlx eslint .
```

**When to run each:**
- `pnpm run check` after any Svelte, TypeScript, routing, store, or component change
- `pnpm run build` after any frontend change
- `pnpm dlx eslint .` after any frontend change

### Rules
- Never assume a small edit compiles
- Run the appropriate verification commands for the changed layers
- Do not mark work complete if any verification command fails

---

## Dependency Source Truth (`src-tauri/vendor/`)

- **Location:** `src-tauri/vendor/<crate>/src/` (e.g., `src-tauri/vendor/reqwest/src/`)
- **Use case:** When you need the *exact* API — struct fields, trait method signatures, function return types. `crates.io` docs or memory may be wrong; the vendor source is ground truth.
- **Refresh:** `cargo vendor` — overwrites `src-tauri/vendor/` with latest from `Cargo.lock`. Only needed when dependency versions change.

## Generated Docs (`src-tauri/target/doc/`)

- **Location:** `src-tauri/target/doc/<crate>/index.html` (open in browser)
- **Root crate:** `src-tauri/target/doc/rymflux_desktop_lib/index.html`
- **Use case:** Navigate trait associated types, enum variants, module re-exports. Faster for exploration than grepping vendor source.
- **Caveat:** Docs reflect what was true at `cargo doc` time. If you change a dependency version, re-run `cargo doc`.

## Effective API Research (Rust)

When a step requires a Rust library API you haven't used before (e.g., Symphonia `FormatReader`):

1. **Grep the vendor source** for the type or trait name:
   ```
   rg "pub trait Source" vendor/rodio/src/
   rg "pub struct AudiobookSource" vendor/rodio/src/
   ```
2. **Read the trait definition** — all required methods, their signatures, and provided methods with defaults.
3. **If the plan exists, verify every API detail against vendor.** The plan may be aspirational.
4. **Cross-reference** with `cargo doc` for navigation-friendly exploration.

### CodeGraph MCP Integration (Knowledge Graph for this Project)

**CodeGraph** provides a **pre-indexed, auto-syncing knowledge graph** of the entire codebase using Tree-sitter. It dramatically reduces tool calls, file reads, and token usage by giving agents direct access to symbols, call graphs, dependencies, routes, and blast radius/impact analysis.

#### Status
- ✅ MCP server is installed and configured (`codegraph install` has been run).
- ✅ Project index exists (`.codegraph/` directory present).
- The graph **auto-syncs** on file changes via native watchers — no manual `sync` needed in normal use.

#### How to Use CodeGraph (Primary Instructions for Agents)

**Preferred workflow:**
1. **Use CodeGraph tools first** for any structural, architectural, or exploration question.
2. **Never start with mass grep / find / read loops** — this duplicates work the graph already performed.
3. Treat content returned by CodeGraph as **already read and authoritative** (it includes line-numbered source).

**Main Tool (recommended for almost everything):**
- `codegraph_explore` — The go-to tool.
  - Query with symbols, functions, classes, files, or natural language questions.
  - Returns: relevant source code (verbatim, grouped by file), call paths between symbols (including dynamic dispatch), and blast-radius / impact summary.
  - Perfect for: "How does X work?", "What calls Y?", "Trace the flow from A to B", "What happens if I change Z?"

**Other useful tools:**
- `codegraph_node` — Get full details on a specific symbol/file + its callers/dependents.
- `codegraph_search` — Full-text + semantic search across the graph.
- `codegraph_callers` — Quick caller analysis.

**CLI fallbacks** (for sub-agents or non-MCP contexts):
```bash
codegraph explore "<query or symbol>"
codegraph node <symbol-or-file>
```

#### Best Practices

- **Start every coding/architecture task** by calling `codegraph_explore` with a precise query.
- For changes: Always explore the target + its callers/blast radius **before** editing.
- After edits: The graph auto-syncs. If you see a staleness banner on a file, read it directly for the absolute latest version.
- Cross-language / framework routes are supported (e.g., API routes to handlers).
- iOS / React Native bridging is handled where applicable.

#### When to Fall Back
- Use regular `read_file`, `grep`, etc., only for:
  - Very recent unsynced edits (rare).
  - Content outside the indexed languages.
  - Verification when explicitly needed.

#### Commands You Can Run (via shell tools)

- `codegraph status` → Check index health and pending syncs.
- `codegraph init` → (Re)build index if needed.
- `codegraph explore ...` → Direct CLI access.

#### Troubleshooting
- No tools visible? Ensure `.codegraph/` exists in the project root and restart the agent.
- Stale index? Run `codegraph status` or `codegraph init`.
- Remove: `codegraph uninit` (project) or `codegraph uninstall` (global agent config).

**CodeGraph is the fastest and most accurate way to understand and modify this codebase.** Prioritize its tools to stay efficient and token-light.

---


## Framework Source of Truth (Svelte/SvelteKit)

Use the local OpenSrc copies of Svelte and SvelteKit as the primary source of truth for framework behavior, APIs, implementation details, and best practices.

Run these in a terminal to find the source paths:
```bash
opensrc path svelte
opensrc path @sveltejs/kit
```

When answering questions or generating code related to Svelte or SvelteKit:

1. **Inspect the relevant source files before responding.** Don't assume — verify.
2. **Prefer repository source over memory or assumptions.**
3. **If documentation, examples, or prior knowledge conflict with the source, follow the source.**
4. **Cite the specific files, functions, types, or modules** that support your answer when practical.
5. **For framework internals, lifecycle behavior, reactivity, routing, load functions, adapters, and configuration — verify implementation details directly from the repositories.**
6. **Do not assume behavior based on older Svelte or SvelteKit versions.**

### What to look for in the Svelte source
| Topic | Location in `opensrc` Svelte source |
|-------|--------------------------------------|
| Error boundaries (`<svelte:boundary>`) | `src/internal/client/dom/blocks/boundary.js` |
| Compiler analysis | `src/compiler/phases/2-analyze/visitors/` |
| Client codegen | `src/compiler/phases/3-transform/client/visitors/` |
| Public API exports | `src/index-client.js` |
| TypeScript types | `src/index.d.ts` (partial — many internals untyped) |

Treat these repositories as the authoritative reference for the installed versions in this project.

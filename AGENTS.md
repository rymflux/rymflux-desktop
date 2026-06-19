# Agent Context

## Planning & Documentation

- **`PLAN.md`** — Master implementation plan. Contains architecture, DDL, API surfaces, and test requirements. **Consult this first** before any implementation step. When the plan says X but the real API says Y, fix the plan with a `⚠️` callout so it stays accurate.
- **`IMPLEMENTATION_JOURNAL.md`** — Chronological log of plan divergences, API corrections, architecture decisions, and testing discoveries. Append to this after each completed step. Use it to avoid repeating mistakes and to know which parts of the plan are stale.
- **Workflow:** Read `PLAN.md` for the step → verify APIs against vendor/docs (see below) → implement → append entry to `IMPLEMENTATION_JOURNAL.md`.

## Dependency Source Truth (`src-tauri/vendor/`)

- **Location:** `src-tauri/vendor/<crate>/src/` (e.g., `src-tauri/vendor/reqwest/src/`)
- **Use case:** When you need the *exact* API — struct fields, trait method signatures, function return types. `crates.io` docs or memory may be wrong; the vendor source is ground truth.
- **Refresh:** `cargo vendor` — overwrites `src-tauri/vendor/` with latest from `Cargo.lock`. Only needed when dependency versions change.

## Generated Docs (`src-tauri/target/doc/`)

- **Location:** `src-tauri/target/doc/<crate>/index.html` (open in browser)
- **Root crate:** `src-tauri/target/doc/rymflux_desktop_lib/index.html`
- **Use case:** Navigate trait associated types, enum variants, module re-exports. Faster for exploration than grepping vendor source.
- **Regenerate:** `cargo doc` (or `cargo doc --open` to launch browser)
- **Caveat:** Docs reflect what was true at `cargo doc` time. If you change a dependency version, re-run `cargo doc`.

## Effective API Research

When a step requires a library API you haven't used before (e.g., Symphonia `FormatReader`):

1. **Grep the vendor source** for the type or trait name:
   ```
   rg "pub trait Source" vendor/rodio/src/
   rg "pub struct AudiobookSource" vendor/rodio/src/  (if it exists)
   ```
2. **Read the trait definition** — all required methods, their signatures, and provided methods with defaults.
4. **If the plan exists, verify every API detail against vendor.** The plan may be aspirational.
5. **Cross-reference** with `cargo doc` for navigation-friendly exploration.


## Framework Source of Truth

Use the local OpenSrc copies of Svelte and SvelteKit as the primary source of truth for framework behavior, APIs, implementation details, and best practices.

Svelte source:
$(opensrc path svelte)

SvelteKit source:
$(opensrc path @sveltejs/kit)

When answering questions or generating code related to Svelte or SvelteKit:

1. Inspect the relevant source files before responding.
2. Prefer repository source over memory or assumptions.
3. If documentation, examples, or prior knowledge conflict with the source, follow the source.
4. Cite the specific files, functions, types, or modules that support your answer when practical.
5. For framework internals, lifecycle behavior, reactivity, routing, load functions, adapters, and configuration, verify implementation details directly from the repositories.
6. Do not assume behavior based on older Svelte or SvelteKit versions.

Treat these repositories as the authoritative reference for the installed versions in this project.

## Build & Verify

- **Compile check:** `cargo check` (fast, skips linking)
- **Full test:** `cargo test`
- **Clippy:** `cargo clippy -- -D warnings` (must pass before PR)
- **Lint after any change** — never assume a small edit compiles.

## Build & Verify

* **Frontend type check:** `pnpm run check`
* **Frontend build:** `pnpm run build`
* **Frontend lint:** `pnpm dlx eslint [options] [file|dir|glob]*` 

* **Compile check:** `cargo check`
* **Full test:** `cargo test`
* **Clippy:** `cargo clippy -- -D warnings`
* **Production app build (for Tauri-related changes):** `pnpm run tauri build`

### Verification Rules

* Run `npm run check` after any Svelte, TypeScript, routing, store, or component change.
* Run `npm run build` after any frontend change.
* Run `pnpm dlx eslint .` after any frontend change.
* Run `cargo check` after any Rust or Tauri backend change.
* Run `cargo test` when modifying Rust business logic.
* Run `cargo clippy -- -D warnings` before submitting changes.
* Run `npm run tauri build` when modifying Tauri commands, permissions, configuration, plugins, window management, IPC, or application startup.
* Never assume a small edit compiles; always verify with the appropriate commands.
* Do not mark work complete if any verification command fails.


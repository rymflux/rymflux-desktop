<script lang="ts">
	import { diag, getUiState, getPlayerState, setTheme, Icon, toggleDiagMode as toggleFrontendDiagMode } from '@rymflux/shell';
	import { listDomains, countContent, syncProgress, clearLibrary } from '$lib/ipc/library';
	import { getAudioEngine } from '$lib/ipc/engineContext';
	import { setBackendDiagMode } from '$lib/ipc/diag';
	import { onMount } from 'svelte';

	let ui = getUiState();
	let playerState = getPlayerState();

	let domains = $state<string[]>([]);
	let contentCount = $state(0);
	let progressCount = $state(0);
	let loading = $state(true);
	let clearing = $state(false);
	let clearConfirm = $state(false);

	onMount(async () => {
		try {
			const d = await listDomains();
			const [counts, progresses] = await Promise.all([
				Promise.all(d.map((r) => countContent(r.id))),
				Promise.all(d.map((r) => syncProgress(r.id).catch(() => [] as never[]))),
			]);
			domains = d.map((r) => r.display_name);
			contentCount = counts.reduce((a, b) => a + b, 0);
			progressCount = progresses.flat().length;
		} catch (e) {
			diag.error('failed to load storage stats', e);
		} finally {
			loading = false;
		}
	});

	async function handleClear() {
		if (clearing) return;
		clearing = true;
		try {
			const d = await listDomains();
			await Promise.all(d.map((r) => clearLibrary(r.id)));
			contentCount = 0;
			progressCount = 0;
			clearConfirm = false;
		} catch (e) {
			diag.error('failed to clear library', e);
		} finally {
			clearing = false;
		}
	}

	// ── Developer dashboard ────────────────────────────────────────────────

	let engine = getAudioEngine();
	let ipcLatency = $state<number | null>(null);
	let ipcTesting = $state(false);
	let devExpanded = $state(false);
	let diagEnabled = $state(false);

	async function handleToggleDiag() {
		diagEnabled = !diagEnabled;
		toggleFrontendDiagMode();
		await setBackendDiagMode(diagEnabled);
	}

	async function testIpcLatency() {
		if (!engine || ipcTesting) return;
		ipcTesting = true;
		const start = performance.now();
		try {
			await engine.getState();
			ipcLatency = performance.now() - start;
		} catch {
			ipcLatency = -1;
		} finally {
			ipcTesting = false;
		}
	}

	type EventLogEntry = { ts: number; type: 'progress' | 'finished' | 'error'; msg: string };
	let eventLog = $state<EventLogEntry[]>([]);
	let logExpanded = $state(false);

	onMount(() => {
		if (!engine) return;
		const unlisteners: (() => void)[] = [];
		import('@tauri-apps/api/event').then(({ listen }) => {
			listen<import('@rymflux/shell').PlaybackState>('audio:progress', (e) => {
				eventLog = [...eventLog.slice(-99), { ts: performance.now(), type: 'progress' as const, msg: `pos=${Math.round(e.payload.position_ms / 1000)}s` }];
			}).then((u) => unlisteners.push(u));
			listen<void>('audio:finished', () => {
				eventLog = [...eventLog.slice(-99), { ts: performance.now(), type: 'finished' as const, msg: 'finished' }];
			}).then((u) => unlisteners.push(u));
			listen<string>('audio:error', (e) => {
				eventLog = [...eventLog.slice(-99), { ts: performance.now(), type: 'error' as const, msg: e.payload }];
			}).then((u) => unlisteners.push(u));
		});
		return () => unlisteners.forEach((u) => u());
	});
</script>

<div class="max-w-2xl mx-auto space-y-8" style="color: var(--text-primary);">
	<h1 class="text-2xl font-bold">Settings</h1>

	<!-- Display -->
	<section>
		<h2 class="text-lg font-semibold mb-3">Display</h2>
		<div class="space-y-3">
			<div class="flex items-center justify-between">
				<span class="text-sm">Theme</span>
				<div class="flex rounded-lg overflow-hidden border" style="border-color: var(--border);">
					<button
						onclick={() => setTheme('dark')}
						class="px-4 py-1.5 text-sm font-medium transition-colors"
						style="
							background-color: {ui.theme === 'dark' ? 'var(--bg-hover)' : 'transparent'};
							color: {ui.theme === 'dark' ? 'var(--text-primary)' : 'var(--text-secondary)'};
						"
					>
						<Icon name="moon" size={16} class="w-4 h-4 inline mr-1.5 -mt-0.5" />
						Dark
					</button>
					<button
						onclick={() => setTheme('light')}
						class="px-4 py-1.5 text-sm font-medium transition-colors"
						style="
							background-color: {ui.theme === 'light' ? 'var(--bg-hover)' : 'transparent'};
							color: {ui.theme === 'light' ? 'var(--text-primary)' : 'var(--text-secondary)'};
						"
					>
						<Icon name="sun" size={16} class="w-4 h-4 inline mr-1.5 -mt-0.5" />
						Light
					</button>
				</div>
			</div>
			<div class="flex items-center justify-between">
				<span class="text-sm">View mode</span>
				<select
					value={ui.viewMode}
					onchange={(e) => (ui.viewMode = (e.target as HTMLSelectElement).value as 'grid' | 'list')}
					class="px-3 py-1.5 text-sm rounded-lg border"
					style="background-color: var(--bg-muted); border-color: var(--border); color: var(--text-primary);"
				>
					<option value="grid">Grid</option>
					<option value="list">List</option>
				</select>
			</div>
		</div>
	</section>

	<!-- Engine State -->
	<section>
		<h2 class="text-lg font-semibold mb-3">Audio Engine</h2>
		{#if playerState.isLoaded}
			<table class="w-full text-sm">
				<tbody class="divide-y" style="border-color: var(--border-subtle);">
					<tr>
						<td class="py-1.5" style="color: var(--text-secondary);">Status</td>
						<td class="py-1.5 text-right">
							<span
								class="inline-block w-2 h-2 rounded-full mr-1.5"
								style="background-color: {playerState.isPlaying ? '#22c55e' : 'var(--text-muted)'};"
							></span>
							{playerState.isPlaying ? 'Playing' : 'Paused'}
						</td>
					</tr>
					<tr>
						<td class="py-1.5" style="color: var(--text-secondary);">Title</td>
						<td class="py-1.5 text-right truncate max-w-56">{playerState.currentTitle || '—'}</td>
					</tr>
					<tr>
						<td class="py-1.5" style="color: var(--text-secondary);">Content ID</td>
						<td class="py-1.5 text-right font-mono text-xs truncate max-w-56">{playerState.currentContentId ?? '—'}</td>
					</tr>
					<tr>
						<td class="py-1.5" style="color: var(--text-secondary);">Position</td>
						<td class="py-1.5 text-right font-mono">{(playerState.positionMs / 1000).toFixed(1)}s / {(playerState.durationMs / 1000).toFixed(1)}s</td>
					</tr>
					<tr>
						<td class="py-1.5" style="color: var(--text-secondary);">Speed</td>
						<td class="py-1.5 text-right">{playerState.speed.toFixed(2)}x</td>
					</tr>
					<tr>
						<td class="py-1.5" style="color: var(--text-secondary);">Volume</td>
						<td class="py-1.5 text-right">{Math.round(playerState.volume * 100)}%</td>
					</tr>
				</tbody>
			</table>
		{:else}
			<p class="label-secondary">No audio loaded.</p>
		{/if}
	</section>

	<!-- Storage Stats -->
	<section>
		<h2 class="text-lg font-semibold mb-3">Storage</h2>
		{#if loading}
			<p class="label-secondary">Loading stats…</p>
		{:else}
			<table class="w-full text-sm">
				<tbody class="divide-y" style="border-color: var(--border-subtle);">
					<tr>
						<td class="py-1.5" style="color: var(--text-secondary);">Domains</td>
						<td class="py-1.5 text-right">{domains.length}</td>
					</tr>
					<tr>
						<td class="py-1.5" style="color: var(--text-secondary);">Content items</td>
						<td class="py-1.5 text-right">{contentCount}</td>
					</tr>
					<tr>
						<td class="py-1.5" style="color: var(--text-secondary);">Progress entries</td>
						<td class="py-1.5 text-right">{progressCount}</td>
					</tr>
				</tbody>
			</table>
		{/if}
	</section>

	<!-- Developer Dashboard -->
	<section>
		<h2 class="text-lg font-semibold mb-3" style="color: var(--text-secondary);">
			<button
				onclick={() => (devExpanded = !devExpanded)}
				class="flex items-center gap-2 transition-colors hover:text-white"
				style="color: var(--text-secondary);"
			>
				Developer
				<Icon name="chevron-right" size={16} class="w-4 h-4 transition-transform {devExpanded ? 'rotate-90' : ''}" />
			</button>
		</h2>
		{#if devExpanded}
			<div class="space-y-4">
				<!-- IPC Latency Test -->
				<div class="card p-4">
					<h3 class="text-sm font-medium mb-2">IPC Latency</h3>
					<button
						onclick={testIpcLatency}
						disabled={ipcTesting}
						class="px-3 py-1.5 text-xs font-medium rounded-lg transition-colors disabled:opacity-50"
						style="background-color: var(--bg-hover); color: var(--text-primary);"
					>
						{ipcTesting ? 'Testing…' : 'Test Round-Trip'}
					</button>
					{#if ipcLatency !== null}
						<p class="text-xs mt-2" style="color: {ipcLatency >= 0 ? 'var(--text-secondary)' : 'var(--text-danger)'}">
							{ipcLatency >= 0 ? `${ipcLatency.toFixed(1)} ms` : 'Failed'}
						</p>
					{/if}
				</div>

				<!-- Diagnostic Mode -->
				<div class="card p-4">
					<h3 class="text-sm font-medium mb-2">Diagnostic Mode</h3>
					<p class="label-secondary mb-3">
						Enables verbose console.log and tracing::debug output for debugging.
					</p>
					<div class="flex items-center gap-3">
						<button
							onclick={handleToggleDiag}
							class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors"
							style="background-color: {diagEnabled ? '#22c55e' : 'var(--bg-hover)'};"
							aria-label="Toggle diagnostic mode"
						>
							<span
								class="inline-block h-4 w-4 rounded-full bg-white transition-transform"
								style="transform: translateX({diagEnabled ? '22px' : '3px'});"
							></span>
						</button>
						<span class="text-xs font-mono" style="color: var(--text-secondary);">
							{diagEnabled ? 'ON' : 'OFF'}
						</span>
					</div>
				</div>

				<!-- Event Log -->
				<div class="card p-4">
					<button
						onclick={() => (logExpanded = !logExpanded)}
						class="flex items-center gap-2 text-sm font-medium transition-colors"
						style="color: var(--text-primary);"
					>
						Event Log ({eventLog.length})
						<Icon name="chevron-right" size={12} class="w-3 h-3 transition-transform {logExpanded ? 'rotate-90' : ''}" />
					</button>
					{#if logExpanded}
						<div class="mt-2 max-h-48 overflow-y-auto font-mono text-xs space-y-0.5">
							{#each [...eventLog].reverse() as entry (entry.ts)}
								<div
									class="flex gap-2"
									style="color: {entry.type === 'error' ? 'var(--text-danger)' : entry.type === 'finished' ? '#22c55e' : 'var(--text-muted)'};"
								>
									<span class="shrink-0">{(entry.ts / 1000).toFixed(1)}s</span>
									<span class="truncate">{entry.msg}</span>
								</div>
							{/each}
							{#if eventLog.length === 0}
								<p style="color: var(--text-muted);">No events yet. Play some audio.</p>
							{/if}
						</div>
					{/if}
				</div>

				<!-- Keyboard Shortcuts Reference -->
				<div class="card p-4">
					<h3 class="text-sm font-medium mb-2">Keyboard Shortcuts</h3>
					<table class="w-full text-xs">
						<tbody class="divide-y" style="border-color: var(--border-subtle);">
							<tr><td class="py-1" style="color: var(--text-secondary);"><kbd class="px-1 rounded" style="background-color: var(--bg-hover);">Space</kbd></td><td class="py-1 text-right" style="color: var(--text-secondary);">Play / Pause</td></tr>
							<tr><td class="py-1" style="color: var(--text-secondary);"><kbd class="px-1 rounded" style="background-color: var(--bg-hover);">←</kbd></td><td class="py-1 text-right" style="color: var(--text-secondary);">Skip -30s</td></tr>
							<tr><td class="py-1" style="color: var(--text-secondary);"><kbd class="px-1 rounded" style="background-color: var(--bg-hover);">→</kbd></td><td class="py-1 text-right" style="color: var(--text-secondary);">Skip +15s</td></tr>
							<tr><td class="py-1" style="color: var(--text-secondary);"><kbd class="px-1 rounded" style="background-color: var(--bg-hover);">+</kbd> <kbd class="px-1 rounded" style="background-color: var(--bg-hover);">-</kbd></td><td class="py-1 text-right" style="color: var(--text-secondary);">Volume</td></tr>
							<tr><td class="py-1" style="color: var(--text-secondary);"><kbd class="px-1 rounded" style="background-color: var(--bg-hover);">M</kbd></td><td class="py-1 text-right" style="color: var(--text-secondary);">Mute</td></tr>
							<tr><td class="py-1" style="color: var(--text-secondary);"><kbd class="px-1 rounded" style="background-color: var(--bg-hover);">[</kbd> <kbd class="px-1 rounded" style="background-color: var(--bg-hover);">]</kbd></td><td class="py-1 text-right" style="color: var(--text-secondary);">Speed fine</td></tr>
							<tr><td class="py-1" style="color: var(--text-secondary);"><kbd class="px-1 rounded" style="background-color: var(--bg-hover);">1</kbd>-<kbd class="px-1 rounded" style="background-color: var(--bg-hover);">6</kbd></td><td class="py-1 text-right" style="color: var(--text-secondary);">Speed presets</td></tr>
							<tr><td class="py-1" style="color: var(--text-secondary);"><kbd class="px-1 rounded" style="background-color: var(--bg-hover);">H</kbd> <kbd class="px-1 rounded" style="background-color: var(--bg-hover);">S</kbd> <kbd class="px-1 rounded" style="background-color: var(--bg-hover);">L</kbd></td><td class="py-1 text-right" style="color: var(--text-secondary);">Navigate</td></tr>
							<tr><td class="py-1" style="color: var(--text-secondary);"><kbd class="px-1 rounded" style="background-color: var(--bg-hover);">?</kbd></td><td class="py-1 text-right" style="color: var(--text-secondary);">Help</td></tr>
						</tbody>
					</table>
				</div>
			</div>
		{/if}
	</section>

	<!-- Danger Zone -->
	<section>
		<h2 class="text-lg font-semibold mb-3" style="color: var(--text-danger);">Danger Zone</h2>
		<div class="card p-4 space-y-3" style="background-color: var(--bg-danger-subtle); border-color: var(--border-danger);">
			<p class="label-secondary">
				Clear all library content and progress. This cannot be undone.
			</p>
			{#if clearConfirm}
				<div class="flex items-center gap-3">
					<p class="text-sm" style="color: var(--text-danger);">Are you sure?</p>
					<button
						onclick={handleClear}
						disabled={clearing}
						class="px-4 py-1.5 rounded-lg text-sm font-medium transition-colors disabled:opacity-50"
						style="background-color: var(--bg-danger); color: white;"
					>
						{clearing ? 'Clearing…' : 'Yes, clear everything'}
					</button>
					<button
						onclick={() => (clearConfirm = false)}
						class="px-4 py-1.5 rounded-lg text-sm transition-colors"
						style="background-color: var(--bg-hover); color: var(--text-primary);"
					>
						Cancel
					</button>
				</div>
			{:else}
				<button
					onclick={() => (clearConfirm = true)}
					class="px-4 py-1.5 text-sm font-medium rounded-lg transition-colors"
					style="background-color: var(--bg-danger-subtle); color: var(--text-danger); border: 1px solid var(--border-danger);"
				>
					Clear Library
				</button>
			{/if}
		</div>
	</section>
</div>

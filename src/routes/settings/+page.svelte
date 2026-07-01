<script lang="ts">
	import { getUiState, getPlayerState } from '@rymflux/shell';
	import { listDomains, countContent, syncProgress, clearLibrary } from '$lib/ipc/library';
	import { getAudioEngine } from '$lib/ipc/engineContext';
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
			console.error('failed to load storage stats', e);
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
			console.error('failed to clear library', e);
		} finally {
			clearing = false;
		}
	}

	// ── Developer dashboard ────────────────────────────────────────────────

	let engine = getAudioEngine();
	let ipcLatency = $state<number | null>(null);
	let ipcTesting = $state(false);
	let devExpanded = $state(false);

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

<div class="max-w-2xl mx-auto space-y-8">
	<h1 class="text-2xl font-bold">Settings</h1>

	<!-- Display -->
	<section>
		<h2 class="text-lg font-semibold mb-3">Display</h2>
		<div class="space-y-3">
			<div class="flex items-center justify-between">
				<span class="text-sm">View mode</span>
				<select
					value={ui.viewMode}
					onchange={(e) => (ui.viewMode = (e.target as HTMLSelectElement).value as 'grid' | 'list')}
					class="bg-white/10 border border-white/20 rounded-lg px-3 py-1.5 text-sm"
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
				<tbody class="divide-y divide-white/5">
					<tr>
						<td class="py-1.5 text-gray-500">Status</td>
						<td class="py-1.5 text-right">
							<span class="inline-block w-2 h-2 rounded-full {playerState.isPlaying ? 'bg-green-500' : 'bg-gray-500'} mr-1.5"></span>
							{playerState.isPlaying ? 'Playing' : 'Paused'}
						</td>
					</tr>
					<tr>
						<td class="py-1.5 text-gray-500">Title</td>
						<td class="py-1.5 text-right truncate max-w-56">{playerState.currentTitle || '—'}</td>
					</tr>
					<tr>
						<td class="py-1.5 text-gray-500">Content ID</td>
						<td class="py-1.5 text-right font-mono text-xs truncate max-w-56">{playerState.currentContentId ?? '—'}</td>
					</tr>
					<tr>
						<td class="py-1.5 text-gray-500">Position</td>
						<td class="py-1.5 text-right font-mono">{(playerState.positionMs / 1000).toFixed(1)}s / {(playerState.durationMs / 1000).toFixed(1)}s</td>
					</tr>
					<tr>
						<td class="py-1.5 text-gray-500">Speed</td>
						<td class="py-1.5 text-right">{playerState.speed.toFixed(2)}x</td>
					</tr>
					<tr>
						<td class="py-1.5 text-gray-500">Volume</td>
						<td class="py-1.5 text-right">{Math.round(playerState.volume * 100)}%</td>
					</tr>
				</tbody>
			</table>
		{:else}
			<p class="text-sm text-gray-500">No audio loaded.</p>
		{/if}
	</section>

	<!-- Storage Stats -->
	<section>
		<h2 class="text-lg font-semibold mb-3">Storage</h2>
		{#if loading}
			<p class="text-sm text-gray-500">Loading stats…</p>
		{:else}
			<table class="w-full text-sm">
				<tbody class="divide-y divide-white/5">
					<tr>
						<td class="py-1.5 text-gray-500">Domains</td>
						<td class="py-1.5 text-right">{domains.length}</td>
					</tr>
					<tr>
						<td class="py-1.5 text-gray-500">Content items</td>
						<td class="py-1.5 text-right">{contentCount}</td>
					</tr>
					<tr>
						<td class="py-1.5 text-gray-500">Progress entries</td>
						<td class="py-1.5 text-right">{progressCount}</td>
					</tr>
				</tbody>
			</table>
		{/if}
	</section>

	<!-- Developer Dashboard -->
	<section>
		<h2 class="text-lg font-semibold mb-3 text-gray-400">
			<button
				onclick={() => (devExpanded = !devExpanded)}
				class="flex items-center gap-2 hover:text-white transition-colors"
			>
				Developer
				<svg
					xmlns="http://www.w3.org/2000/svg"
					viewBox="0 0 24 24"
					fill="currentColor"
					class="w-4 h-4 transition-transform {devExpanded ? 'rotate-90' : ''}"
				>
					<path d="M9 6l6 6-6 6" />
				</svg>
			</button>
		</h2>
		{#if devExpanded}
			<div class="space-y-4">
				<!-- IPC Latency Test -->
				<div class="bg-white/5 rounded-lg p-4">
					<h3 class="text-sm font-medium mb-2">IPC Latency</h3>
					<button
						onclick={testIpcLatency}
						disabled={ipcTesting}
						class="px-3 py-1.5 bg-white/10 rounded-lg text-xs font-medium hover:bg-white/20 transition-colors disabled:opacity-50"
					>
						{ipcTesting ? 'Testing…' : 'Test Round-Trip'}
					</button>
					{#if ipcLatency !== null}
						<p class="text-xs mt-2 {ipcLatency >= 0 ? 'text-gray-400' : 'text-red-400'}">
							{ipcLatency >= 0 ? `${ipcLatency.toFixed(1)} ms` : 'Failed'}
						</p>
					{/if}
				</div>

				<!-- Event Log -->
				<div class="bg-white/5 rounded-lg p-4">
					<button
						onclick={() => (logExpanded = !logExpanded)}
						class="flex items-center gap-2 text-sm font-medium hover:text-white transition-colors"
					>
						Event Log ({eventLog.length})
						<svg
							xmlns="http://www.w3.org/2000/svg"
							viewBox="0 0 24 24"
							fill="currentColor"
							class="w-3 h-3 transition-transform {logExpanded ? 'rotate-90' : ''}"
						>
							<path d="M9 6l6 6-6 6" />
						</svg>
					</button>
					{#if logExpanded}
						<div class="mt-2 max-h-48 overflow-y-auto font-mono text-[10px] space-y-0.5">
							{#each [...eventLog].reverse() as entry (entry.ts)}
								<div class="flex gap-2 {entry.type === 'error' ? 'text-red-400' : entry.type === 'finished' ? 'text-green-400' : 'text-gray-500'}">
									<span class="shrink-0">{(entry.ts / 1000).toFixed(1)}s</span>
									<span class="truncate">{entry.msg}</span>
								</div>
							{/each}
							{#if eventLog.length === 0}
								<p class="text-gray-600">No events yet. Play some audio.</p>
							{/if}
						</div>
					{/if}
				</div>

				<!-- Keyboard Shortcuts Reference -->
				<div class="bg-white/5 rounded-lg p-4">
					<h3 class="text-sm font-medium mb-2">Keyboard Shortcuts</h3>
					<table class="w-full text-xs">
						<tbody class="divide-y divide-white/5">
							<tr><td class="py-1 text-gray-500"><kbd class="px-1 bg-white/10 rounded text-[10px]">Space</kbd></td><td class="py-1 text-right text-gray-400">Play / Pause</td></tr>
							<tr><td class="py-1 text-gray-500"><kbd class="px-1 bg-white/10 rounded text-[10px]">←</kbd></td><td class="py-1 text-right text-gray-400">Skip -30s</td></tr>
							<tr><td class="py-1 text-gray-500"><kbd class="px-1 bg-white/10 rounded text-[10px]">→</kbd></td><td class="py-1 text-right text-gray-400">Skip +15s</td></tr>
							<tr><td class="py-1 text-gray-500"><kbd class="px-1 bg-white/10 rounded text-[10px]">+</kbd> <kbd class="px-1 bg-white/10 rounded text-[10px]">-</kbd></td><td class="py-1 text-right text-gray-400">Volume</td></tr>
							<tr><td class="py-1 text-gray-500"><kbd class="px-1 bg-white/10 rounded text-[10px]">M</kbd></td><td class="py-1 text-right text-gray-400">Mute</td></tr>
							<tr><td class="py-1 text-gray-500"><kbd class="px-1 bg-white/10 rounded text-[10px]">[</kbd> <kbd class="px-1 bg-white/10 rounded text-[10px]">]</kbd></td><td class="py-1 text-right text-gray-400">Speed fine</td></tr>
							<tr><td class="py-1 text-gray-500"><kbd class="px-1 bg-white/10 rounded text-[10px]">1</kbd>-<kbd class="px-1 bg-white/10 rounded text-[10px]">6</kbd></td><td class="py-1 text-right text-gray-400">Speed presets</td></tr>
							<tr><td class="py-1 text-gray-500"><kbd class="px-1 bg-white/10 rounded text-[10px]">H</kbd> <kbd class="px-1 bg-white/10 rounded text-[10px]">S</kbd> <kbd class="px-1 bg-white/10 rounded text-[10px]">L</kbd></td><td class="py-1 text-right text-gray-400">Navigate</td></tr>
							<tr><td class="py-1 text-gray-500"><kbd class="px-1 bg-white/10 rounded text-[10px]">?</kbd></td><td class="py-1 text-right text-gray-400">Help</td></tr>
						</tbody>
					</table>
				</div>
			</div>
		{/if}
	</section>

	<!-- Danger Zone -->
	<section>
		<h2 class="text-lg font-semibold mb-3 text-red-400">Danger Zone</h2>
		<div class="bg-red-600/10 border border-red-600/20 rounded-lg p-4 space-y-3">
			<p class="text-sm text-gray-400">
				Clear all library content and progress. This cannot be undone.
			</p>
			{#if clearConfirm}
				<div class="flex items-center gap-3">
					<p class="text-sm text-red-400">Are you sure?</p>
					<button
						onclick={handleClear}
						disabled={clearing}
						class="px-4 py-1.5 bg-red-600 text-white rounded-lg text-sm font-medium hover:bg-red-700 transition-colors disabled:opacity-50"
					>
						{clearing ? 'Clearing…' : 'Yes, clear everything'}
					</button>
					<button
						onclick={() => (clearConfirm = false)}
						class="px-4 py-1.5 bg-white/10 rounded-lg text-sm hover:bg-white/20 transition-colors"
					>
						Cancel
					</button>
				</div>
			{:else}
				<button
					onclick={() => (clearConfirm = true)}
					class="px-4 py-1.5 bg-red-600/20 text-red-400 border border-red-600/30 rounded-lg text-sm font-medium hover:bg-red-600/30 transition-colors"
				>
					Clear Library
				</button>
			{/if}
		</div>
	</section>
</div>

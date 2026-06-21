<script lang="ts">
	import { getUiState } from '$lib/stores/uiStore.svelte';
	import { getPlayerState } from '$lib/stores/playerStore.svelte';
	import { listDomains, countContent, syncProgress, clearLibrary } from '$lib/ipc/library';
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
			const [d, c, p] = await Promise.all([
				listDomains(),
				countContent('audiobook'),
				syncProgress('audiobook'),
			]);
			domains = d.map((r) => r.display_name);
			contentCount = c;
			progressCount = p.length;
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
			await clearLibrary('audiobook');
			contentCount = 0;
			progressCount = 0;
			clearConfirm = false;
		} catch (e) {
			console.error('failed to clear library', e);
		} finally {
			clearing = false;
		}
	}
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

	<!-- Danger Zone -->
	<section>
		<h2 class="text-lg font-semibold mb-3 text-red-400">Danger Zone</h2>
		<div class="bg-red-600/10 border border-red-600/20 rounded-lg p-4 space-y-3">
			<p class="text-sm text-gray-400">
				Clear all audiobook content and progress from the library. This cannot be undone.
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

	<p class="text-xs text-gray-600">rymflux v0.1</p>
</div>

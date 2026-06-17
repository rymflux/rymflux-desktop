<script lang="ts">
	import { getPlayerState } from '$lib/stores/playerStore.svelte';
	import { onMount } from 'svelte';
	import { syncProgress } from '$lib/ipc/library';
	import type { ProgressRecord } from '$lib/types/ipc';

	let playerState = getPlayerState();
	let recent = $state<ProgressRecord[]>([]);

	onMount(async () => {
		try {
			recent = await syncProgress('audiobook');
		} catch {
			// noop
		}
	});
</script>

<div class="max-w-4xl mx-auto">
	<h1 class="text-2xl font-bold mb-6">Home</h1>

	{#if playerState.isLoaded}
		<div class="mb-8 p-4 bg-white/5 rounded-xl">
			<p class="text-sm text-gray-400">Now Playing</p>
			<p class="text-lg font-semibold">{playerState.currentTitle || 'Untitled'}</p>
		</div>
	{/if}

	<section>
		<h2 class="text-lg font-semibold mb-3">Continue Listening</h2>
		{#if recent.length === 0}
			<p class="text-gray-500 text-sm">No listening history yet. Search the catalog to find an audiobook.</p>
		{:else}
			<div class="space-y-2">
				{#each recent as r}
					<div class="p-3 bg-white/5 rounded-lg text-sm">
						{r.content_id}
					</div>
				{/each}
			</div>
		{/if}
	</section>
</div>

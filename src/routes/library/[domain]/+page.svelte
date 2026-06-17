<script lang="ts">
	import { listLibrary } from '$lib/ipc/library';
	import type { ContentItem } from '$lib/types/ipc';
	import { onMount } from 'svelte';

	let { params } = $props();
	let items = $state<ContentItem[]>([]);
	let loading = $state(true);

	onMount(async () => {
		try {
			items = await listLibrary(params.domain);
		} catch {
			// noop
		} finally {
			loading = false;
		}
	});
</script>

<div class="max-w-4xl mx-auto">
	<h1 class="text-2xl font-bold mb-6 capitalize">{params.domain}</h1>

	{#if loading}
		<p class="text-gray-500 text-sm">Loading...</p>
	{:else if items.length === 0}
		<p class="text-gray-500 text-sm">Nothing here yet.</p>
	{:else}
		<div class="space-y-2">
			{#each items as item}
				<a
					href="/player/{item.content_id}"
					class="block p-4 bg-white/5 rounded-xl hover:bg-white/10 transition-colors"
				>
					<h3 class="font-semibold">{item.title || 'Untitled'}</h3>
					<p class="text-sm text-gray-400">{item.author || 'Unknown author'}</p>
				</a>
			{/each}
		</div>
	{/if}
</div>

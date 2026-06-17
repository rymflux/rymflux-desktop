<script lang="ts">
	import { listLibrary } from '$lib/ipc/library';
	import type { ContentItem } from '$lib/types/ipc';
	import { onMount } from 'svelte';

	let items = $state<ContentItem[]>([]);
	let loading = $state(true);

	onMount(async () => {
		try {
			items = await listLibrary('audiobook');
		} catch {
			// noop
		} finally {
			loading = false;
		}
	});
</script>

<div class="max-w-4xl mx-auto">
	<h1 class="text-2xl font-bold mb-6">Library</h1>

	<div class="flex gap-4 mb-6">
		<a
			href="/library/audiobook"
			class="px-4 py-2 bg-white/10 rounded-lg text-sm hover:bg-white/20 transition-colors"
		>
			Audiobooks
		</a>
	</div>

	{#if loading}
		<p class="text-gray-500 text-sm">Loading...</p>
	{:else if items.length === 0}
		<div class="text-center py-12">
			<p class="text-gray-400">Your library is empty.</p>
			<p class="text-gray-500 text-sm mt-1">Browse the LibriVox catalog to add audiobooks.</p>
			<a
				href="/search"
				class="inline-block mt-4 px-5 py-2 bg-blue-600 rounded-lg text-sm font-medium hover:bg-blue-700 transition-colors"
			>
				Browse Catalog
			</a>
		</div>
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

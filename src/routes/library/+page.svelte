<script lang="ts">
	import { listLibrary } from '$lib/ipc/library';
	import type { ContentItem } from '$lib/types/ipc';
	import LibraryView from '$src/domains/audiobook/LibraryView.svelte';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';

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

	function handleSelect(item: ContentItem) {
		goto(`/player/${item.content_id}`);
	}
</script>

<div class="max-w-4xl mx-auto">
	<h1 class="text-2xl font-bold mb-6">Library</h1>

	<div class="flex gap-4 mb-6">
		<a
			href="/library/audiobook"
			class="px-4 py-2 bg-blue-600 rounded-lg text-sm font-medium hover:bg-blue-700 transition-colors"
		>
			Audiobooks
		</a>
	</div>

	<LibraryView {items} {loading} onSelect={handleSelect} />
</div>

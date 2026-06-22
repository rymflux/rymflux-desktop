<script lang="ts">
	import { listLibrary } from '$lib/ipc/library';
import type { DomainItem } from '$lib/types/ipc';
	import LibraryView from '$src/domains/audiobook/LibraryView.svelte';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';

	let { params } = $props();
	let items = $state<DomainItem[]>([]);
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

	function handleSelect(item: DomainItem) {
		goto(resolve(`/player/${item.content_id}`));
	}
</script>

<div class="max-w-4xl mx-auto">
	<h1 class="text-2xl font-bold capitalize mb-6">{params.domain}</h1>

	<LibraryView {items} {loading} onSelect={handleSelect} />
</div>

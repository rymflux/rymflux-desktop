<script lang="ts">
	import { listLibrary, syncProgress } from '$lib/ipc/library';
	import { getDomainRegistry } from '@rymflux/shell';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';

	let { params } = $props();
	let items = $state<import('@rymflux/shell').DomainItem[]>([]);
	let progressMap = $state<Map<string, number>>(new Map());
	let loading = $state(true);

	onMount(async () => {
		try {
			const [libItems, progress] = await Promise.all([
				listLibrary(params.domain),
				syncProgress(params.domain).catch(() => [] as never[]),
			]);
			items = libItems;
			progressMap = new Map(
				(progress as Array<{ content_id: string; position_ms: number }>).map((p) => [p.content_id, p.position_ms]),
			);
		} catch {
			// noop
		} finally {
			loading = false;
		}
	});

	function handleSelect(item: import('@rymflux/shell').DomainItem) {
		goto(resolve(`/player/${item.content_id}`));
	}

	let registry = getDomainRegistry();
	let domainView = registry.get(params.domain)?.views.library;
</script>

<div class="max-w-4xl mx-auto">
	<h1 class="text-2xl font-bold capitalize mb-6">{params.domain}</h1>

	{#each [domainView] as Component}
		{#if Component}
			<Component {items} {loading} {progressMap} onSelect={handleSelect}></Component>
		{:else}
			<p class="text-gray-500">Unknown domain: {params.domain}</p>
		{/if}
	{/each}
</div>

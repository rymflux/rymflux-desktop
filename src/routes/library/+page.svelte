<script lang="ts">
	import { listDomains, countAllContent } from '$lib/ipc/library';
	import type { DomainRecord } from '@rymflux/shell';
	import { onMount } from 'svelte';
	import { resolve } from '$app/paths';

	let domains = $state<{ id: string; label: string; count: number }[]>([]);
	let loading = $state(true);

	onMount(async () => {
		try {
			const records: DomainRecord[] = await listDomains();
			const counts = await countAllContent();
			domains = records.map((d) => ({
				id: d.id,
				label: d.display_name,
				count: counts[d.id] ?? 0,
			}));
		} catch {
			domains = [];
		} finally {
			loading = false;
		}
	});
</script>

<div class="max-w-4xl mx-auto">
	<h1 class="text-2xl font-bold mb-6">Library</h1>

	{#if loading}
		<div class="flex justify-center py-12">
			<div class="w-8 h-8 border-2 border-blue-500 border-t-transparent rounded-full animate-spin"></div>
		</div>
	{:else if domains.length === 0}
		<div class="text-center py-12">
			<p class="text-gray-400">No library domains found.</p>
			<a
				href={resolve('/search')}
				class="inline-block mt-4 px-5 py-2 bg-blue-600 rounded-lg text-sm font-medium hover:bg-blue-700 transition-colors"
			>
				Browse Catalog
			</a>
		</div>
	{:else}
		<div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
			{#each domains as domain (domain.id)}
				<a
					href={resolve('/library/[domain]', { domain: domain.id })}
					class="block p-5 rounded-xl bg-white/5 hover:bg-white/10 border border-white/10 transition-colors group"
				>
					<h2 class="text-lg font-semibold group-hover:text-blue-400 transition-colors">{domain.label}</h2>
					<p class="text-sm text-gray-500 mt-1">
						{domain.count} {domain.count === 1 ? 'item' : 'items'}
					</p>
				</a>
			{/each}
		</div>
	{/if}
</div>

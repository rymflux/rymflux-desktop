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
	<h1 class="text-2xl font-bold mb-6" style="color: var(--text-primary);">Library</h1>

	{#if loading}
		<div class="flex justify-center py-12">
			<div class="w-8 h-8 border-2 border-t-transparent rounded-full animate-spin" style="border-color: var(--accent, #3b82f6); border-top-color: transparent;"></div>
		</div>
	{:else if domains.length === 0}
		<div class="text-center py-12">
			<p class="label-secondary">No library domains found.</p>
			<a
				href={resolve('/search')}
				class="inline-block mt-4 px-5 py-2 rounded-lg text-sm font-medium transition-colors"
				style="background-color: var(--accent, #3b82f6); color: white;"
			>
				Browse Catalog
			</a>
		</div>
	{:else}
		<div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
			{#each domains as domain (domain.id)}
				<a
					href={resolve('/library/[domain]', { domain: domain.id })}
					class="card-hover block p-5"
				>
					<h2 class="text-lg font-semibold transition-colors" style="color: var(--text-primary);">{domain.label}</h2>
					<p class="label-secondary mt-1">
						{domain.count} {domain.count === 1 ? 'item' : 'items'}
					</p>
				</a>
			{/each}
		</div>
	{/if}
</div>

<script lang="ts">
	import { searchCatalog } from '$lib/ipc/catalog';
	import type { CatalogItem } from '$lib/types/ipc';

	let query = $state('');
	let results = $state<CatalogItem[]>([]);
	let searching = $state(false);
	let debounceTimer: ReturnType<typeof setTimeout> | undefined;

	async function doSearch(q: string) {
		if (!q.trim()) {
			results = [];
			return;
		}
		searching = true;
		try {
			results = await searchCatalog(q.trim(), 20, 0);
		} catch (e) {
			console.error('search failed', e);
		} finally {
			searching = false;
		}
	}

	function handleInput(e: Event) {
		const el = e.currentTarget as HTMLInputElement;
		query = el.value;
		clearTimeout(debounceTimer);
		debounceTimer = setTimeout(() => doSearch(query), 300);
	}
</script>

<div class="max-w-4xl mx-auto">
	<h1 class="text-2xl font-bold mb-4">Search LibriVox</h1>

	<input
		type="search"
		placeholder="Search by title..."
		value={query}
		oninput={handleInput}
		class="w-full px-4 py-2.5 rounded-xl bg-white/10 border border-white/20 text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500"
	/>

	{#if searching}
		<p class="text-gray-400 text-sm mt-4">Searching...</p>
	{/if}

	{#if results.length > 0}
		<div class="mt-6 space-y-2">
			{#each results as book}
				<a
					href="/player/{book.id}"
					class="block p-4 bg-white/5 rounded-xl hover:bg-white/10 transition-colors"
				>
					<h3 class="font-semibold">{book.title}</h3>
					<p class="text-sm text-gray-400">{book.author}</p>
					{#if book.description}
						<p class="text-xs text-gray-500 mt-1 line-clamp-2">{book.description}</p>
					{/if}
				</a>
			{/each}
		</div>
	{:else if query && !searching}
		<p class="text-gray-500 text-sm mt-4">No results found.</p>
	{/if}
</div>

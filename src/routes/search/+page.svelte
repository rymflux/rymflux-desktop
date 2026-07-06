<script lang="ts">
	import { resolve } from '$app/paths';
	import { diag } from '$lib/utils/diag.svelte';
	import CoverImage from '$lib/components/CoverImage.svelte';
	import LoadingSpinner from '$lib/components/LoadingSpinner.svelte';
	import { getDomainRegistry } from '$lib/registry/index';
	import type { DomainItem } from '$lib/types/ipc';

	let query = $state('');
	let searchType = $state<'title' | 'author'>('title');
	let results = $state<DomainItem[]>([]);
	let searching = $state(false);
	let offset = $state(0);
	let hasMore = $state(false);
	let debounceTimer: ReturnType<typeof setTimeout> | undefined;

	const limit = 20;

	async function doSearch(q: string, startOffset: number, append: boolean) {
		if (!q.trim()) {
			results = [];
			hasMore = false;
			return;
		}
		searching = true;
		try {
			const registry = getDomainRegistry();
			const allItems: DomainItem[] = [];
			for (const [, domain] of registry) {
				if (domain.search) {
					const items = await domain.search(q.trim(), searchType, limit, startOffset);
					allItems.push(...items);
				}
			}
			if (append) {
				results = [...results, ...allItems];
			} else {
				results = allItems;
			}
			hasMore = allItems.length >= limit;
			offset = startOffset;
		} catch (e) {
			diag.error('search failed', e);
		} finally {
			searching = false;
		}
	}

	function handleInput(e: Event) {
		const el = e.currentTarget as HTMLInputElement;
		query = el.value;
		clearTimeout(debounceTimer);
		debounceTimer = setTimeout(() => doSearch(query, 0, false), 300);
	}

	function handleToggle(type: 'title' | 'author') {
		if (type === searchType) return;
		searchType = type;
		clearTimeout(debounceTimer);
		doSearch(query, 0, false);
	}

	function loadMore() {
		doSearch(query, offset + limit, true);
	}
</script>

<div class="max-w-4xl mx-auto">
	<h1 class="text-2xl font-bold mb-4">Search</h1>

	<div class="flex gap-2 mb-3">
		<button
			onclick={() => handleToggle('title')}
			class="px-3 py-1.5 rounded-lg text-sm font-medium transition-colors {searchType === 'title' ? 'bg-blue-600 text-white' : 'bg-white/10 text-gray-400 hover:text-white'}"
		>
			By Title
		</button>
		<button
			onclick={() => handleToggle('author')}
			class="px-3 py-1.5 rounded-lg text-sm font-medium transition-colors {searchType === 'author' ? 'bg-blue-600 text-white' : 'bg-white/10 text-gray-400 hover:text-white'}"
		>
			By Author
		</button>
	</div>

	<input
		type="search"
		placeholder="Search by {searchType === 'title' ? 'title' : 'author last name'}..."
		value={query}
		oninput={handleInput}
		class="w-full px-4 py-2.5 rounded-xl bg-white/10 border border-white/20 text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500"
	/>

	{#if searching}
		<div class="mt-6">
			<LoadingSpinner size="md" />
		</div>
	{/if}

	{#if results.length > 0}
		<div class="mt-6 space-y-2">
			{#each results as book (book.content_id)}
				<a
					href={resolve('/player/[contentId]', { contentId: book.content_id })}
					class="flex gap-3 p-3 bg-white/5 rounded-xl hover:bg-white/10 transition-colors"
				>
					<div class="w-16 h-16 shrink-0 rounded-lg overflow-hidden bg-white/10">
						<CoverImage url={book.cover_url} title={book.title} class="w-full h-full object-cover" />
					</div>
					<div class="flex-1 min-w-0">
						<h3 class="font-semibold text-sm truncate">{book.title}</h3>
						<p class="text-xs text-gray-400 truncate">{book.author}</p>
						<div class="flex flex-wrap gap-1.5 mt-1">
							{#if book.language}
								<span class="px-1.5 py-0.5 rounded bg-white/10 text-[10px] text-gray-400 uppercase tracking-wider">{book.language}</span>
							{/if}
							{#if book.num_sections != null}
								<span class="px-1.5 py-0.5 rounded bg-white/10 text-[10px] text-gray-400">{book.num_sections} {book.num_sections === 1 ? 'section' : 'sections'}</span>
							{/if}
							{#if book.duration_ms != null}
								<span class="px-1.5 py-0.5 rounded bg-white/10 text-[10px] text-gray-400">{Math.floor(book.duration_ms / 60000)} min</span>
							{/if}
						</div>
						{#if book.description}
							<p class="text-[11px] text-gray-500 mt-1 line-clamp-1">{book.description}</p>
						{/if}
					</div>
				</a>
			{/each}
		</div>

		{#if hasMore && !searching}
			<div class="mt-4 flex justify-center">
				<button
					onclick={loadMore}
					class="px-5 py-2 bg-white/10 rounded-lg text-sm font-medium hover:bg-white/20 transition-colors"
				>
					Load More
				</button>
			</div>
		{/if}
	{:else if query && !searching}
		<p class="text-gray-500 text-sm mt-4">No results found.</p>
	{/if}
</div>

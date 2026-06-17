<script lang="ts">
	import { getBook, addToLibrary } from '$lib/ipc/catalog';
	import { getProgress } from '$lib/ipc/library';
	import { getPlayerState, setCurrentTrack } from '$lib/stores/playerStore';
	import CoverImage from '$lib/components/CoverImage.svelte';
	import TimeDisplay from '$lib/components/TimeDisplay.svelte';
	import { onMount } from 'svelte';
	import type { CatalogDetail } from '$lib/types/ipc';

	let { params } = $props();
	let playerState = getPlayerState();

	let book = $state<CatalogDetail | null>(null);
	let savedProgress = $state(0);
	let loading = $state(true);
	let adding = $state(false);

	onMount(async () => {
		try {
			const [b, p] = await Promise.all([
				getBook(params.contentId),
				getProgress(params.contentId).catch(() => null),
			]);
			book = b;
			if (p) savedProgress = p.position_ms;
		} catch {
			// noop
		} finally {
			loading = false;
		}
	});

	function handlePlay() {
		if (!book) return;
		const firstSection = book.sections[0];
		if (!firstSection) return;
		const source = {
			uri: firstSection.listen_url,
			duration_ms: (firstSection.playtime_secs ?? 0) * 1000,
			mime_type: 'audio/mpeg',
		};
		setCurrentTrack(source, params.contentId, book.item.title);
	}

	async function handleAddToLibrary() {
		if (adding) return;
		adding = true;
		try {
			await addToLibrary(params.contentId);
		} catch (e) {
			console.error('add to library failed', e);
		} finally {
			adding = false;
		}
	}
</script>

<div class="max-w-3xl mx-auto">
	{#if loading}
		<p class="text-gray-500">Loading...</p>
	{:else if book}
		<div class="flex gap-6 mb-8">
			<CoverImage url={book.item.cover_url} title={book.item.title} class="w-40 h-40 rounded-xl shrink-0" />
			<div class="flex-1 min-w-0">
				<h1 class="text-2xl font-bold">{book.item.title}</h1>
				<p class="text-gray-400 mt-1">{book.item.author}</p>
				<p class="text-sm text-gray-500 mt-2 line-clamp-3">{book.item.description}</p>
				<div class="flex gap-3 mt-4">
					<button
						onclick={handlePlay}
						class="px-5 py-2 bg-blue-600 rounded-lg text-sm font-medium hover:bg-blue-700 transition-colors"
					>
						Play
					</button>
					<button
						onclick={handleAddToLibrary}
						disabled={adding}
						class="px-5 py-2 bg-white/10 rounded-lg text-sm font-medium hover:bg-white/20 transition-colors disabled:opacity-50"
					>
						{adding ? 'Adding...' : 'Add to Library'}
					</button>
				</div>
				{#if savedProgress > 0}
					<p class="text-xs text-gray-500 mt-2">
						Saved progress: <TimeDisplay seconds={Math.floor(savedProgress / 1000)} />
					</p>
				{/if}
			</div>
		</div>

		<section>
			<h2 class="text-lg font-semibold mb-3">Chapters</h2>
			<div class="space-y-1">
				{#each book.sections as section}
					<button
						onclick={handlePlay}
						class="w-full flex items-center gap-3 p-3 rounded-lg hover:bg-white/5 transition-colors text-left"
					>
						<span class="text-sm text-gray-400 w-8 shrink-0">{section.section_number}</span>
						<span class="flex-1 truncate text-sm">{section.title}</span>
						{#if section.playtime_secs}
							<TimeDisplay seconds={section.playtime_secs} />
						{/if}
					</button>
				{/each}
			</div>
		</section>
	{:else}
		<p class="text-gray-500">Book not found.</p>
	{/if}
</div>

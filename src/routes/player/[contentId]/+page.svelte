<script lang="ts">
	import { getBook, addToLibrary, resolveSource } from '$lib/ipc/catalog';
	import { getProgress } from '$lib/ipc/library';
	import { setCurrentTrack, getPlayerState } from '$lib/stores/playerStore.svelte';
	import DetailView from '$src/domains/audiobook/DetailView.svelte';
	import { getAudioEngine } from '$lib/ipc/engineContext';
	import { onMount } from 'svelte';
import type { CatalogDetail } from '$lib/types/ipc';

	let { params } = $props();
	let playerState = getPlayerState();
	let engine = getAudioEngine();

	// Catalog uses numeric IDs; library stores prefixed (librivox_123)
	let catalogId = $derived(
		params.contentId.startsWith('librivox_')
			? params.contentId.slice('librivox_'.length)
			: params.contentId,
	);

	let book = $state<CatalogDetail | null>(null);
	let savedProgress = $state(0);
	let loading = $state(true);
	let adding = $state(false);

	onMount(async () => {
		const bPromise = getBook(catalogId).catch((e) => {
			console.error('getBook failed:', e);
			return null;
		});
		const pPromise = getProgress(params.contentId).catch(() => null);
		const [b, p] = await Promise.all([bPromise, pPromise]);
		book = b;
		if (p) savedProgress = p.position_ms;
		loading = false;
	});



	async function handlePlay(chapterIndex?: number) {
		if (!book) return;
		const sections = book.sections;
		const idx = chapterIndex ?? 0;
		const section = sections[idx];
		if (!section) return;
		const source = await resolveSource(section, book.archive_identifier ?? null);
		// Seek to saved progress with 3s rewind when continuing
		const startMs = savedProgress > 0 && chapterIndex === undefined
			? Math.max(0, savedProgress - 3000)
			: 0;
		setCurrentTrack(source, params.contentId, book.item.title);
		engine?.play(source, params.contentId, startMs);
	}

	async function handleAddToLibrary() {
		if (adding) return;
		adding = true;
		try {
			await addToLibrary(catalogId);
		} catch (e) {
			console.error('add to library failed', e);
		} finally {
			adding = false;
		}
	}
</script>

<div class="max-w-3xl mx-auto">
	{#if loading}
		<div class="flex justify-center py-12">
			<div class="w-8 h-8 border-2 border-blue-500 border-t-transparent rounded-full animate-spin"></div>
		</div>
	{:else if book}
		<DetailView
			{book}
			contentId={params.contentId}
			{savedProgress}
			onPlay={handlePlay}
			onAddToLibrary={handleAddToLibrary}
			{adding}
		/>
	{:else}
		<div class="text-center py-12">
			<p class="text-gray-500">Book not found.</p>
			<a href="/search" class="text-blue-400 text-sm mt-2 inline-block hover:underline">Search catalog</a>
		</div>
	{/if}
</div>

<script lang="ts">
	import { getProgress, getLibraryDetail, removeFromLibrary, storeItem } from '$lib/ipc/library';
	import { setCurrentTrack, getPlayerState, getDomainRegistry } from '@rymflux/shell';
	import { getAudioEngine } from '$lib/ipc/engineContext';
	import { buildProgressContext } from '$lib/ipc/progressContext';
	import { onMount } from 'svelte';
	import type { ChapterInfo } from '@rymflux/shell';
	import { resolve } from '$app/paths';

	let { params } = $props();
	let playerState = getPlayerState();
	let engine = getAudioEngine();
	let domain = getDomainRegistry().get('audiobook');

	let book = $state<import('@rymflux/shell').CatalogDetail | null>(null);
	let savedProgress = $state(0);
	let loading = $state(true);
	let adding = $state(false);
	let removing = $state(false);
	let isInLibrary = $state(false);
	let showDetails = $state(false);
	let showNowPlaying = $derived(playerState.isLoaded && playerState.currentContentId === params.contentId);

	let detailView = domain?.views.detail;
	let playerView = domain?.views.player;

	onMount(async () => {
		const bPromise = domain?.getDetail?.(params.contentId).catch((e) => {
			console.error('getDetail failed:', e);
			return null;
		}) ?? Promise.resolve(null);
		const pPromise = getProgress(params.contentId).catch(() => null);
		const lPromise = getLibraryDetail(params.contentId).then((item) => item !== null);
		const [b, p, inLib] = await Promise.all([bPromise, pPromise, lPromise]);
		book = b;
		if (p) savedProgress = p.position_ms;
		isInLibrary = inLib;
		loading = false;
	});

	// Compute cumulative chapter offsets (ms from start of book to start of each chapter)
	function getChapterOffsets(sections: ChapterInfo[]): number[] {
		const offsets: number[] = [];
		let cum = 0;
		for (const s of sections) {
			offsets.push(cum);
			cum += (s.playtime_secs ?? 0) * 1000;
		}
		return offsets;
	}

	// Find which chapter contains the given cumulative position
	function findChapterIndex(offsets: number[], positionMs: number): number {
		for (let i = offsets.length - 1; i >= 0; i--) {
			if (positionMs >= offsets[i]) return i;
		}
		return 0;
	}

	async function handlePlay(chapterIndex?: number) {
		if (!book || !domain) return;
		const sections = book.sections;
		const offsets = getChapterOffsets(sections);

		// Find which chapter to play — when resuming, find the chapter containing saved progress
		const idx = chapterIndex ?? (savedProgress > 0 ? findChapterIndex(offsets, savedProgress) : 0);
		const section = sections[idx];
		if (!section) return;

		const source = domain.resolveSource?.(section.listen_url, (section.playtime_secs ?? 0) * 1000);
		if (!source) return;

		// Seek to saved progress with 3s rewind, then adjust to intra-chapter position
		const chapterOffset = offsets[idx];
		const rawStartMs = savedProgress > 0 && chapterIndex === undefined
			? Math.max(0, savedProgress - 3000)
			: 0;
		const startMs = Math.max(0, rawStartMs - chapterOffset);

		setCurrentTrack(source, params.contentId, book.item.title, 'audiobook', book.item.cover_url, idx, book.sections);
		engine?.play(source, params.contentId, startMs);
		const savedSpeed = localStorage.getItem('speed_' + params.contentId);
		if (savedSpeed) {
			engine?.setSpeed(parseFloat(savedSpeed));
		}
	}

	async function handleAddToLibrary() {
		if (adding || !domain) return;
		adding = true;
		try {
			const item = await domain.buildLibraryItem?.(params.contentId);
			if (item) {
				await storeItem(item, item.id, (item.metadata_json?.total_time_secs as number | undefined) ?? null);
				isInLibrary = true;
			}
		} catch (e) {
			console.error('add to library failed', e);
		} finally {
			adding = false;
		}
	}

	function handlePlayPause() {
		if (!engine) return;
		const p = playerState;
		if (!p.currentContentId) return;
		if (p.isPlaying) {
			engine.pause(p.currentDomainId, p.currentContentId, buildProgressContext(p.currentSections, p.currentChapterIndex));
		} else if (p.currentSource) {
			engine.play(p.currentSource, p.currentContentId, p.positionMs);
			const savedSpeed = localStorage.getItem('speed_' + p.currentContentId);
			if (savedSpeed) engine.setSpeed(parseFloat(savedSpeed));
		}
	}

	function handleSeek(ms: number) {
		if (!engine || !playerState.currentContentId) return;
		engine.seek(playerState.currentDomainId, playerState.currentContentId, ms, buildProgressContext(playerState.currentSections, playerState.currentChapterIndex));
	}

	function handleSkipBack() {
		const p = playerState;
		handleSeek(Math.max(0, p.positionMs - 30_000));
	}

	function handleSkipForward() {
		const p = playerState;
		handleSeek(Math.min(p.durationMs, p.positionMs + 15_000));
	}

	function handleSpeedChange(rate: number) {
		engine?.setSpeed(rate);
		if (playerState.currentContentId) {
			localStorage.setItem('speed_' + playerState.currentContentId, rate.toString());
		}
	}

	function handleVolumeChange(v: number) {
		engine?.setVolume(v);
	}

	async function handleRemoveFromLibrary() {
		if (removing) return;
		removing = true;
		try {
			await removeFromLibrary(params.contentId);
			isInLibrary = false;
		} catch (e) {
			console.error('remove from library failed', e);
		} finally {
			removing = false;
		}
	}
</script>

<div class="max-w-3xl mx-auto">
	{#if loading}
		<div class="flex justify-center py-12">
			<div class="w-8 h-8 border-2 border-blue-500 border-t-transparent rounded-full animate-spin"></div>
		</div>
	{:else if book}
		{#if showNowPlaying && !showDetails}
			{#each [playerView] as Component}
				{#if Component}
					<Component
						onPlayPause={handlePlayPause}
						onSeek={handleSeek}
						onSkipBack={handleSkipBack}
						onSkipForward={handleSkipForward}
						onSpeedChange={handleSpeedChange}
						onVolumeChange={handleVolumeChange}
						onChapterClick={handlePlay}
					></Component>
				{/if}
			{/each}
			<div class="text-center mt-4">
				<button
					onclick={() => showDetails = true}
					class="text-sm text-blue-400 hover:underline"
				>
					← Show Book Details & Chapters
				</button>
			</div>
		{:else}
			{#each [detailView] as Component}
				{#if Component}
					<Component
						{book}
						{savedProgress}
						onPlay={handlePlay}
						onAddToLibrary={handleAddToLibrary}
						onRemoveFromLibrary={handleRemoveFromLibrary}
						{adding}
						{removing}
						{isInLibrary}
					></Component>
				{/if}
			{/each}
			{#if showNowPlaying}
				<div class="text-center mt-4">
					<button
						onclick={() => showDetails = false}
						class="text-sm text-blue-400 hover:underline"
					>
						Show Player
					</button>
				</div>
			{/if}
		{/if}
	{:else}
		<div class="text-center py-12">
			<p class="text-gray-500">Book not found.</p>
			<a href={resolve('/search')} class="text-blue-400 text-sm mt-2 inline-block hover:underline">Search catalog</a>
		</div>
	{/if}
</div>

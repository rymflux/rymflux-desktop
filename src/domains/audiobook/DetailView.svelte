<script lang="ts">
	import CoverImage from '$lib/components/CoverImage.svelte';
	import TimeDisplay from '$lib/components/TimeDisplay.svelte';
	import type { CatalogDetail } from '$lib/types/ipc';

	let {
		book,
		contentId,
		savedProgress = 0,
		onPlay,
		onAddToLibrary,
		adding = false,
		hideAddButton = false,
		hideChapters = false,
	}: {
		book: CatalogDetail;
		contentId: string;
		savedProgress?: number;
		onPlay?: (chapterIndex?: number) => void;
		onAddToLibrary?: () => void;
		adding?: boolean;
		hideAddButton?: boolean;
		hideChapters?: boolean;
	} = $props();
</script>

<div class="flex flex-col md:flex-row gap-6 mb-8">
	<div class="w-full md:w-48 shrink-0">
		<div class="aspect-[3/4] rounded-xl overflow-hidden bg-white/5">
			<CoverImage url={book.item.cover_url} title={book.item.title} class="w-full h-full object-cover" />
		</div>
	</div>
	<div class="flex-1 min-w-0">
		<h1 class="text-2xl font-bold">{book.item.title}</h1>
		<p class="text-gray-400 mt-1">{book.item.author}</p>
		<p class="text-sm text-gray-500 mt-2 line-clamp-3">{book.item.description}</p>

		<div class="flex gap-3 mt-4">
			<button
				onclick={() => onPlay?.(savedProgress > 0 ? undefined : 0)}
				class="px-5 py-2 bg-blue-600 rounded-lg text-sm font-medium hover:bg-blue-700 transition-colors"
			>
				{savedProgress > 0 ? 'Continue' : 'Play'}
			</button>
			{#if !hideAddButton}
				<button
					onclick={() => onAddToLibrary?.()}
					disabled={adding}
					class="px-5 py-2 bg-white/10 rounded-lg text-sm font-medium hover:bg-white/20 transition-colors disabled:opacity-50"
				>
					{adding ? 'Adding…' : 'Add to Library'}
				</button>
			{/if}
		</div>

		{#if savedProgress > 0}
			<p class="text-xs text-gray-500 mt-2">
				Saved progress: <TimeDisplay seconds={Math.floor(savedProgress / 1000)} />
			</p>
		{/if}

		<!-- Metadata -->
		<div class="mt-4 space-y-1 text-xs text-gray-500">
			<p>Publisher: LibriVox</p>
			<p>Total time: {book.item.total_time_secs != null ? `${Math.floor(book.item.total_time_secs / 60)} min` : '—'}</p>
			<p>Sections: {book.item.num_sections ?? '—'}</p>
		</div>
	</div>
</div>

<!-- Chapters -->
{#if !hideChapters && book.sections.length > 0}
	<section>
		<h2 class="text-lg font-semibold mb-3">Chapters</h2>
		<div class="space-y-1">
			{#each book.sections as section, i}
				<button
					onclick={() => onPlay?.(i)}
					class="w-full flex items-center gap-3 p-3 rounded-lg hover:bg-white/5 transition-colors text-left"
				>
					<span class="text-sm text-gray-400 w-8 shrink-0">{section.section_number}</span>
					<span class="flex-1 truncate text-sm">{section.title}</span>
					{#if section.playtime_secs}
						<TimeDisplay seconds={section.playtime_secs} />
					{/if}
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-4 h-4 text-gray-500 shrink-0">
						<polygon points="8,5 19,12 8,19" />
					</svg>
				</button>
			{/each}
		</div>
	</section>
{/if}

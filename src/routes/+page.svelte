<script lang="ts">
	import { getPlayerState } from '$lib/stores/playerStore.svelte';
	import { syncProgress, getLibraryDetail } from '$lib/ipc/library';
	import CoverImage from '$lib/components/CoverImage.svelte';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';

	let playerState = getPlayerState();
	let cards = $state<{
		contentId: string;
		title: string;
		author: string;
		coverUrl: string | null;
		positionMs: number;
		durationMs: number | null;
	}[]>([]);
	let loading = $state(true);

	onMount(async () => {
		try {
			const progress = await syncProgress('audiobook');
			const inProgress = progress.filter((p) => p.position_ms > 0);

			const results = await Promise.all(
				inProgress.map(async (p) => {
					try {
						const detail = await getLibraryDetail(p.content_id);
						if (!detail) return null;
						const meta = (detail.metadata_json ?? {}) as Record<string, unknown>;
						const durationMs = meta.total_time_secs != null ? (meta.total_time_secs as number) * 1000 : null;
						if (durationMs != null && p.position_ms >= durationMs) return null;
						return {
							contentId: p.content_id,
							title: (meta.title as string) || 'Untitled',
							author: (meta.author as string) || 'Unknown',
							coverUrl: (meta.cover_url as string) || null,
							positionMs: p.position_ms,
							durationMs,
						};
					} catch {
						return null;
					}
				}),
			);

			cards = results.filter((r): r is NonNullable<typeof r> => r != null);
		} catch {
			// noop
		} finally {
			loading = false;
		}
	});
</script>

<div class="max-w-4xl mx-auto">
	<h1 class="text-2xl font-bold mb-6">Home</h1>

	{#if playerState.isLoaded}
		<div class="mb-8 p-4 bg-white/5 rounded-xl flex items-center gap-4">
			<div class="min-w-0 flex-1">
				<p class="text-sm text-gray-400">Now Playing</p>
				<p class="text-lg font-semibold truncate">{playerState.currentTitle || 'Untitled'}</p>
			</div>
			<a
				href="/player/{playerState.currentContentId}"
				class="shrink-0 px-4 py-2 bg-blue-600 rounded-lg text-sm font-medium hover:bg-blue-700 transition-colors"
			>
				Open Player
			</a>
		</div>
	{/if}

	<section>
		<h2 class="text-lg font-semibold mb-3">Continue Listening</h2>

		{#if loading}
			<div class="flex justify-center py-12">
				<div class="w-8 h-8 border-2 border-blue-500 border-t-transparent rounded-full animate-spin"></div>
			</div>
		{:else if cards.length === 0}
			<p class="text-gray-500 text-sm">
				No items in progress. Search the catalog to find an audiobook.
			</p>
		{:else}
			<div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
				{#each cards as card}
					<div class="bg-white/5 rounded-xl overflow-hidden border border-white/10 flex flex-col">
						<div class="aspect-[3/4] bg-white/5">
							<CoverImage url={card.coverUrl} title={card.title} class="w-full h-full object-cover" />
						</div>
						<div class="p-3 flex flex-col gap-2 flex-1">
							<h3 class="font-medium text-sm truncate">{card.title}</h3>
							<p class="text-xs text-gray-500 truncate">{card.author}</p>

							{#if card.durationMs != null && card.durationMs > 0}
								<div class="mt-auto">
									<div class="w-full h-1.5 bg-white/10 rounded-full overflow-hidden">
										<div
											class="h-full bg-blue-500 rounded-full transition-all"
											style="width: {Math.min((card.positionMs / card.durationMs) * 100, 100)}%"
										></div>
									</div>
									<p class="text-xs text-gray-500 mt-1">
										{Math.floor(card.positionMs / 60000)} / {Math.floor(card.durationMs / 60000)} min
									</p>
								</div>
							{/if}

							<button
								onclick={() => goto(`/player/${card.contentId}`)}
								class="w-full mt-1 px-3 py-1.5 bg-blue-600 rounded-lg text-sm font-medium hover:bg-blue-700 transition-colors"
							>
								Continue
							</button>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</section>
</div>

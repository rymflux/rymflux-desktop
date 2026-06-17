<script lang="ts">
	import CoverImage from '$lib/components/CoverImage.svelte';
	import { getPlayerState } from '$lib/stores/playerStore';
	import { getAudioEngine } from '$lib/ipc/engineContext';

	let playerState = getPlayerState();
	let engine = getAudioEngine();

	function handlePlayPause() {
		if (!playerState.currentContentId) return;
		if (playerState.isPlaying) {
			engine.pause(playerState.currentDomainId, playerState.currentContentId);
		} else {
			engine.play(playerState.currentSource!, playerState.currentContentId, playerState.positionMs);
		}
	}
</script>

<a
	href="/player/{playerState.currentContentId}"
	class="flex items-center gap-3 px-3 py-2 rounded-lg hover:bg-white/5 transition-colors group"
>
	<div class="w-8 h-8 rounded overflow-hidden bg-white/10 shrink-0">
		<CoverImage
			url={playerState.currentSource?.uri}
			title={playerState.currentTitle}
			class="w-full h-full object-cover"
		/>
	</div>
	<div class="flex-1 min-w-0">
		<div class="flex items-center gap-2">
			<div class="flex-1 h-1 rounded-full bg-white/10 overflow-hidden">
				<div
					class="h-full rounded-full bg-blue-500 transition-all duration-250"
					style="width: {playerState.progressFraction * 100}%"
				></div>
			</div>
		</div>
		<p class="text-xs text-gray-400 truncate mt-0.5">{playerState.currentTitle || 'No title'}</p>
	</div>
	<button
		onclick={(e) => { e.preventDefault(); handlePlayPause(); }}
		class="p-1.5 rounded-full hover:bg-white/10 transition-colors shrink-0"
		aria-label={playerState.isPlaying ? 'Pause' : 'Play'}
	>
		{#if playerState.isPlaying}
			<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-5 h-5">
				<rect x="5" y="4" width="5" height="16" rx="1" />
				<rect x="14" y="4" width="5" height="16" rx="1" />
			</svg>
		{:else}
			<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-5 h-5">
				<polygon points="8,5 19,12 8,19" />
			</svg>
		{/if}
	</button>
</a>

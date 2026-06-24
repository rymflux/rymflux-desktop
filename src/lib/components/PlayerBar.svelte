<script lang="ts">
	import CoverImage from './CoverImage.svelte';
	import SeekBar from './SeekBar.svelte';
	import PlaybackControls from './PlaybackControls.svelte';
	import TimeDisplay from './TimeDisplay.svelte';
	import VolumeSlider from './VolumeSlider.svelte';
	import { getPlayerState } from '$lib/stores/playerStore.svelte';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';

	let {
		onPlayPause,
		onSeek,
		onSkipBack,
		onSkipForward,
		onSpeedChange,
		onVolumeChange,
	}: {
		onPlayPause?: () => void;
		onSeek?: (ms: number) => void;
		onSkipBack?: () => void;
		onSkipForward?: () => void;
		onSpeedChange?: (rate: number) => void;
		onVolumeChange?: (v: number) => void;
	} = $props();

	let playerState = getPlayerState();

	function handleSeekFraction(f: number) {
		const target = Math.round(f * playerState.durationMs);
		onSeek?.(target);
	}

	function handleNavToPlayer() {
		if (playerState.currentContentId) {
			goto(resolve(`/player/${playerState.currentContentId}`));
		}
	}
</script>

{#if playerState.isLoaded}
	<div class="h-16 border-t border-white/10 bg-gray-900 flex items-center px-4 gap-3 shrink-0">
		<!-- Mini-info: cover + title + progress (clickable to full player) -->
		<button
			onclick={handleNavToPlayer}
			class="flex items-center gap-2 w-40 shrink-0 text-left hover:bg-white/5 rounded-lg px-1.5 py-1 transition-colors group"
		>
			<div class="w-8 h-8 rounded overflow-hidden bg-white/10 shrink-0">
				<CoverImage
					url={playerState.currentCoverUrl}
					title={playerState.currentTitle}
					class="w-full h-full object-cover"
				/>
			</div>
			<div class="flex-1 min-w-0">
				<p class="text-sm text-white/80 truncate">{playerState.currentTitle || 'No title'}</p>
				<div class="h-1 rounded-full bg-white/10 overflow-hidden mt-1">
					<div
						class="h-full rounded-full bg-blue-500 transition-all duration-250"
						style="width: {playerState.progressFraction * 100}%"
					></div>
				</div>
			</div>
		</button>

		<!-- Seek bar + time -->
		<div class="flex items-center gap-2 flex-1 min-w-0">
			<TimeDisplay seconds={Math.floor(playerState.positionMs / 1000)} />
			<div class="flex-1">
				<SeekBar
					progress={playerState.progressFraction}
					onSeek={handleSeekFraction}
				/>
			</div>
			<TimeDisplay seconds={Math.floor(playerState.remainingMs / 1000)} />
		</div>

		<!-- Controls -->
		<PlaybackControls
			{onPlayPause}
			{onSkipBack}
			{onSkipForward}
			{onSpeedChange}
		/>

		<!-- Volume -->
		<VolumeSlider volume={playerState.volume} onVolumeChange={onVolumeChange} />
	</div>
{/if}

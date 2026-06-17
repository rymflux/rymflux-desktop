<script lang="ts">
	import SeekBar from './SeekBar.svelte';
	import PlaybackControls from './PlaybackControls.svelte';
	import TimeDisplay from './TimeDisplay.svelte';
	import VolumeSlider from './VolumeSlider.svelte';
	import { getPlayerState } from '$lib/stores/playerStore';

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
</script>

{#if playerState.isLoaded}
	<div class="h-16 border-t border-white/10 bg-gray-900 flex items-center px-4 gap-3 shrink-0">
		<!-- Title (truncated) -->
		<div class="w-40 truncate text-sm text-white/80 shrink-0">
			{playerState.currentTitle || 'No title'}
		</div>

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
			onSeek={handleSeekFraction}
			{onSkipBack}
			{onSkipForward}
			{onSpeedChange}
		/>

		<!-- Volume -->
		<VolumeSlider volume={playerState.volume} onVolumeChange={onVolumeChange} />
	</div>
{/if}

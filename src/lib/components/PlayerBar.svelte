<script lang="ts">
	import CoverImage from './CoverImage.svelte';
	import SeekBar from './SeekBar.svelte';
	import PlaybackControls from './PlaybackControls.svelte';
	import TimeDisplay from './TimeDisplay.svelte';
	import VolumeSlider from './VolumeSlider.svelte';
	import { getPlayerState } from '../stores/playerStore.svelte';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';

	let {
		onPlayPause,
		onSeek,
		onSkipBack,
		onSkipForward,
		onSpeedChange,
		onVolumeChange
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
	<div
		class="h-16 border-t flex items-center px-4 gap-3 shrink-0"
		style="background-color: var(--bg-secondary); border-top-color: var(--border);"
	>
		<!-- Mini-info: cover + title + progress (clickable to full player) -->
		<button
			onclick={handleNavToPlayer}
			class="flex items-center gap-2 w-40 shrink-0 text-left rounded-lg px-1.5 py-1 transition-colors group"
			style="color: var(--text-primary);"
		>
			<div
				class="w-8 h-8 rounded overflow-hidden shrink-0"
				style="background-color: var(--bg-hover);"
			>
				<CoverImage
					url={playerState.currentCoverUrl}
					title={playerState.currentTitle}
					class="w-full h-full object-cover"
				/>
			</div>
			<div class="flex-1 min-w-0">
				<p class="text-sm truncate" style="color: var(--text-primary);">
					{playerState.currentTitle || 'No title'}
				</p>
				<div
					class="h-1 rounded-full mt-1 overflow-hidden"
					style="background-color: var(--bg-hover);"
				>
					<div
						class="h-full rounded-full transition-all duration-250"
						style="width: {playerState.progressFraction *
							100}%; background-color: var(--accent, #3b82f6);"
					></div>
				</div>
			</div>
		</button>

		<!-- Seek bar + time -->
		<div class="flex items-center gap-2 flex-1 min-w-0">
			<TimeDisplay seconds={Math.floor(playerState.positionMs / 1000)} />
			<div class="flex-1">
				<SeekBar progress={playerState.progressFraction} onSeek={handleSeekFraction} />
			</div>
			<TimeDisplay seconds={Math.floor(playerState.remainingMs / 1000)} />
		</div>

		<!-- Controls -->
		<PlaybackControls {onPlayPause} {onSkipBack} {onSkipForward} {onSpeedChange} />

		<!-- Volume -->
		<VolumeSlider volume={playerState.volume} {onVolumeChange} />
	</div>
{/if}

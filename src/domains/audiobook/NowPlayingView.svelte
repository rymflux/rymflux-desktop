<script lang="ts">
	import SeekBar from '$lib/components/SeekBar.svelte';
	import PlaybackControls from '$lib/components/PlaybackControls.svelte';
	import TimeDisplay from '$lib/components/TimeDisplay.svelte';
	import VolumeSlider from '$lib/components/VolumeSlider.svelte';
	import CoverImage from '$lib/components/CoverImage.svelte';
	import { getPlayerState } from '$lib/stores/playerStore.svelte';
	import { getAudioEngine } from '$lib/ipc/engineContext';
	import { setProgress } from '$lib/ipc/library';

	let playerState = getPlayerState();
	let engine = getAudioEngine();

	let sleepTimer = $state<{ endTime: number } | null>(null);
	let sleepOption = $state<'none' | 15 | 30 | 45 | 60>('none');
	let timerHandle = $state<ReturnType<typeof setInterval> | undefined>();

	$effect(() => {
		if (sleepTimer && Date.now() >= sleepTimer.endTime) {
			engine.pause(playerState.currentDomainId, playerState.currentContentId!);
			sleepTimer = null;
			sleepOption = 'none';
		}
	});

	function startSleepTimer(minutes: number) {
		sleepTimer = { endTime: Date.now() + minutes * 60 * 1000 };
		sleepOption = minutes as typeof sleepOption;
	}

	function cancelSleepTimer() {
		sleepTimer = null;
		sleepOption = 'none';
	}

	function handleSeekFraction(f: number) {
		if (!playerState.currentContentId) return;
		const target = Math.round(f * playerState.durationMs);
		engine.seek(playerState.currentDomainId, playerState.currentContentId, target);
	}

	function handleSkipBack() {
		if (!playerState.currentContentId) return;
		engine.seek(playerState.currentDomainId, playerState.currentContentId, Math.max(0, playerState.positionMs - 30_000));
	}

	function handleSkipForward() {
		if (!playerState.currentContentId) return;
		engine.seek(playerState.currentDomainId, playerState.currentContentId, Math.min(playerState.durationMs, playerState.positionMs + 15_000));
	}

	function handlePlayPause() {
		if (!playerState.currentContentId) return;
		if (playerState.isPlaying) {
			engine.pause(playerState.currentDomainId, playerState.currentContentId);
		} else {
			engine.play(playerState.currentSource!, playerState.currentContentId, playerState.positionMs);
		}
	}

	function handleSpeedChange(rate: number) {
		engine.setSpeed(rate);
	}

	function handleVolumeChange(v: number) {
		engine.setVolume(v);
	}
</script>

<div class="max-w-2xl mx-auto space-y-6">
	<!-- Large cover art + title -->
	<div class="text-center">
		<div class="w-48 h-48 mx-auto rounded-xl overflow-hidden bg-white/5 mb-4">
			<CoverImage
				url={playerState.currentSource?.uri}
				title={playerState.currentTitle}
				class="w-full h-full object-cover"
			/>
		</div>
		<h1 class="text-xl font-bold">{playerState.currentTitle || 'Now Playing'}</h1>
	</div>

	<!-- Seek bar -->
	<div class="space-y-1">
		<SeekBar
			progress={playerState.progressFraction}
			onSeek={handleSeekFraction}
		/>
		<div class="flex justify-between text-xs text-gray-500">
			<TimeDisplay seconds={Math.floor(playerState.positionMs / 1000)} />
			<TimeDisplay seconds={Math.floor(playerState.remainingMs / 1000)} />
		</div>
	</div>

	<!-- Controls -->
	<div class="flex justify-center">
		<PlaybackControls
			onPlayPause={handlePlayPause}
			onSeek={handleSeekFraction}
			onSkipBack={handleSkipBack}
			onSkipForward={handleSkipForward}
			onSpeedChange={handleSpeedChange}
		/>
	</div>

	<!-- Volume -->
	<div class="flex justify-center">
		<div class="w-48">
			<VolumeSlider volume={playerState.volume} onVolumeChange={handleVolumeChange} />
		</div>
	</div>

	<!-- Sleep timer -->
	<div class="flex justify-center">
		<div class="flex items-center gap-2 text-sm">
			<span class="text-gray-500">Sleep:</span>
			{#if sleepOption === 'none'}
				{#each [15, 30, 45, 60] as mins}
					<button
						onclick={() => startSleepTimer(mins)}
						class="px-2 py-1 rounded bg-white/10 hover:bg-white/20 transition-colors text-xs"
					>
						{mins}m
					</button>
				{/each}
			{:else}
				<span class="text-blue-400 text-xs">Sleeping in {sleepOption} min</span>
				<button
					onclick={cancelSleepTimer}
					class="px-2 py-1 rounded bg-white/10 hover:bg-white/20 transition-colors text-xs"
				>
					Cancel
				</button>
			{/if}
		</div>
	</div>
</div>

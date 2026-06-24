<script lang="ts">
	import SeekBar from '$lib/components/SeekBar.svelte';
	import PlaybackControls from '$lib/components/PlaybackControls.svelte';
	import TimeDisplay from '$lib/components/TimeDisplay.svelte';
	import VolumeSlider from '$lib/components/VolumeSlider.svelte';
	import CoverImage from '$lib/components/CoverImage.svelte';
	import { getPlayerState } from '$lib/stores/playerStore.svelte';
	import { getAudioEngine } from '$lib/ipc/engineContext';
import { resolveSource } from '$lib/ipc/catalog';
import { setCurrentTrack } from '$lib/stores/playerStore.svelte';
import { onMount } from 'svelte';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
	let engine = getAudioEngine()!;

	let playerState = getPlayerState();

let sleepOption = $state<'none' | 15 | 30 | 45 | 60 | 'chapter'>('none');
let timerHandle = $state<ReturnType<typeof setTimeout> | undefined>();

function startSleepTimer(minutes: number) {
	sleepOption = minutes as typeof sleepOption;
	clearTimeout(timerHandle);
	timerHandle = setTimeout(() => {
		engine.pause(playerState.currentDomainId, playerState.currentContentId!);
		sleepOption = 'none';
		timerHandle = undefined;
	}, minutes * 60 * 1000);
}

function startChapterSleep() {
	clearTimeout(timerHandle);
	sleepOption = 'chapter';
}

function cancelSleepTimer() {
	clearTimeout(timerHandle);
	timerHandle = undefined;
}

let cleanupFinished: UnlistenFn | undefined;

onMount(() => {
	const setup = async () => {
		cleanupFinished = await listen<void>('audio:finished', () => {
			if (sleepOption === 'chapter') {
				engine.pause(playerState.currentDomainId, playerState.currentContentId!);
				sleepOption = 'none';
			}
		});
	};
	setup();

	return () => {
		cleanupFinished?.();
	};
});

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

	async function handleChapterClick(i: number) {
		const sections = playerState.currentSections;
		const section = sections[i];
		if (!section || !playerState.currentContentId) return;
		const source = await resolveSource(section.listen_url, (section.playtime_secs ?? 0) * 1000);
		setCurrentTrack(source, playerState.currentContentId, playerState.currentTitle, playerState.currentDomainId, playerState.currentCoverUrl, i, playerState.currentSections);
		engine.play(source, playerState.currentContentId, 0);
	}
</script>

<div class="max-w-2xl mx-auto space-y-6">
	<!-- Large cover art + title -->
	<div class="text-center">
		<div class="w-48 h-48 mx-auto rounded-2xl overflow-hidden bg-white/5 mb-4">
			<CoverImage url={playerState.currentCoverUrl} title={playerState.currentTitle} class="w-full h-full object-cover" />
		</div>
		<h2 class="text-xl font-semibold truncate">{playerState.currentTitle || 'No title'}</h2>
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
				{#each [15, 30, 45, 60] as mins (mins)}
					<button
						onclick={() => startSleepTimer(mins)}
						class="px-2 py-1 rounded bg-white/10 hover:bg-white/20 transition-colors text-xs"
					>
						{mins}m
					</button>
				{/each}
				<button
					onclick={startChapterSleep}
					class="px-2 py-1 rounded bg-white/10 hover:bg-white/20 transition-colors text-xs"
				>
					End of chapter
				</button>
			{:else if sleepOption === 'chapter'}
				<span class="text-blue-400 text-xs">Sleeping at end of chapter</span>
				<button
					onclick={cancelSleepTimer}
					class="px-2 py-1 rounded bg-white/10 hover:bg-white/20 transition-colors text-xs"
				>
					Cancel
				</button>
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

	<!-- Chapter navigation -->
	{#if playerState.currentSections.length > 0}
		<section>
			<h3 class="text-sm font-semibold text-gray-400 uppercase tracking-wider mb-2">Chapters</h3>
			<div class="space-y-1 max-h-60 overflow-y-auto">
				{#each playerState.currentSections as section, i (section.section_number)}
					<button
						onclick={() => handleChapterClick(i)}
						class="w-full flex items-center gap-3 p-2.5 rounded-lg transition-colors text-left {i === playerState.currentChapterIndex ? 'bg-blue-600/20 text-blue-300 border border-blue-600/30' : 'hover:bg-white/5 text-gray-300'}"
					>
						<span class="text-xs text-gray-500 w-6 shrink-0 text-right">{section.section_number}</span>
						<span class="flex-1 truncate text-sm">{section.title || 'Chapter ' + section.section_number}</span>
						{#if section.playtime_secs}
							<TimeDisplay seconds={section.playtime_secs} />
						{/if}
						{#if i === playerState.currentChapterIndex}
							<span class="w-2 h-2 rounded-full bg-blue-400 shrink-0" title="Now playing"></span>
						{/if}
					</button>
				{/each}
			</div>
		</section>
	{/if}
	</div>
</div>

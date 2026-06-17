<script lang="ts">
	import ShellLayout from '$lib/components/ShellLayout.svelte';
	import { TauriAudioEngine } from '$lib/ipc/audioEngine';
	import { updatePlaybackState, getPlayerState } from '$lib/stores/playerStore';
	import { onMount } from 'svelte';

	let { children } = $props();

	let engine = $state<TauriAudioEngine | null>(null);
	let playerState = getPlayerState();

	onMount(() => {
		const e = new TauriAudioEngine();
		engine = e;
		e.init(
			(s) => updatePlaybackState(s),
			() => {
				/* handle finished */
			},
			(err) => console.error('audio:error', err),
		);
		return () => e.destroy();
	});

	function handlePlayPause() {
		if (!engine || !playerState.currentContentId) return;
		if (playerState.isPlaying) {
			engine.pause(playerState.currentDomainId, playerState.currentContentId);
		} else {
			engine.play(
				playerState.currentSource!,
				playerState.currentContentId,
				playerState.positionMs,
			);
		}
	}

	function handleSeek(ms: number) {
		if (!engine || !playerState.currentContentId) return;
		engine.seek(playerState.currentDomainId, playerState.currentContentId, ms);
	}

	function handleSkipBack() {
		handleSeek(Math.max(0, playerState.positionMs - 30_000));
	}

	function handleSkipForward() {
		handleSeek(Math.min(playerState.durationMs, playerState.positionMs + 15_000));
	}

	function handleSpeedChange(rate: number) {
		engine?.setSpeed(rate);
	}

	function handleVolumeChange(v: number) {
		engine?.setVolume(v);
	}
</script>

<ShellLayout
	onPlayPause={handlePlayPause}
	onSeek={handleSeek}
	onSkipBack={handleSkipBack}
	onSkipForward={handleSkipForward}
	onSpeedChange={handleSpeedChange}
	onVolumeChange={handleVolumeChange}
>
	{@render children()}
</ShellLayout>

<script lang="ts">
	import ShellLayout from '$lib/components/ShellLayout.svelte';
	import { TauriAudioEngine } from '$lib/ipc/audioEngine';
	import { setAudioEngine } from '$lib/ipc/engineContext';
	import { updatePlaybackState, getPlayerState } from '$lib/stores/playerStore.svelte';
	import { setProgress } from '$lib/ipc/library';
	import '../app.css';
	import { onMount } from 'svelte';

	let { children } = $props();

	let engine = $state<TauriAudioEngine | null>(null);
	let playerState = getPlayerState();
	let heartbeatHandle = $state<ReturnType<typeof setInterval> | undefined>();

	$effect(() => {
		if (engine) {
			setAudioEngine(engine);
		}
	});

	// Save progress every 10s during playback
	$effect(() => {
		if (playerState.isPlaying && playerState.currentContentId) {
			clearInterval(heartbeatHandle);
			heartbeatHandle = setInterval(() => {
				if (
					playerState.currentContentId &&
					playerState.positionMs > 0 &&
					playerState.currentDomainId
				) {
					setProgress(
						playerState.currentDomainId,
						playerState.currentContentId,
						playerState.positionMs,
					).catch(() => {});
				}
			}, 10_000);
		} else {
			clearInterval(heartbeatHandle);
			heartbeatHandle = undefined;
		}
		return () => clearInterval(heartbeatHandle);
	});

	onMount(() => {
		// Guard: only init Tauri engine when running inside Tauri
		const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
		if (!isTauri) return;

		const e = new TauriAudioEngine();
		engine = e;
		e.init(
			(s) => updatePlaybackState(s),
			() => {
				if (playerState.currentContentId && playerState.currentDomainId) {
					setProgress(
						playerState.currentDomainId,
						playerState.currentContentId,
						playerState.positionMs,
					).catch(() => {});
				}
			},
			(err) => console.error('audio:error', err),
		).catch((err) => console.error('audio:init failed', err));
		return () => {
			clearInterval(heartbeatHandle);
			e.destroy();
		};
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

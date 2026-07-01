<script lang="ts">
	import { ShellLayout, ErrorBoundary, updatePlaybackState, getPlayerState, createDomainRegistry } from '@rymflux/shell';
	import { audiobookDomain } from '@rymflux/domain-audiobook';
	import { TauriAudioEngine } from '$lib/ipc/audioEngine';
	import { setAudioEngine } from '$lib/ipc/engineContext';
	import { buildProgressContext } from '$lib/ipc/progressContext';
	import { setProgress } from '$lib/ipc/library';
	import '../app.css';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';

	createDomainRegistry([audiobookDomain]);

	let { children } = $props();

	let engine = $state<TauriAudioEngine | null>(null);
	let playerState = getPlayerState();
	let heartbeatHandle: ReturnType<typeof setInterval> | undefined;
	let showShortcuts = $state(false);
	let prevVolume = $state<number>(1.0); // for mute toggle

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
					const ctx =
						playerState.currentSections.length > 0
							? buildProgressContext(
									playerState.currentSections,
									playerState.currentChapterIndex,
								)
							: undefined;
					setProgress(
						playerState.currentDomainId,
						playerState.currentContentId,
						playerState.positionMs,
						ctx,
						playerState.speed,
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
		// Keyboard shortcuts for playback control
		function handleKeyDown(e: KeyboardEvent) {
			// Don't intercept when user is typing in an input or textarea
			const tag = (e.target as HTMLElement)?.tagName;
			if (tag === 'INPUT' || tag === 'TEXTAREA' || tag === 'SELECT') return;

			// Navigation and help shortcuts (no engine needed)
			switch (e.key) {
				case '?':
				case '/':
					e.preventDefault();
					showShortcuts = !showShortcuts;
					return;
				case 'h':
				case 'H':
					e.preventDefault();
					goto(resolve('/'));
					return;
				case 's':
				case 'S':
					e.preventDefault();
					goto(resolve('/search'));
					return;
				case 'l':
				case 'L':
					e.preventDefault();
					goto(resolve('/library'));
					return;
			}

			// Playback shortcuts need audio engine
			if (!engine) return;

			switch (e.key) {
				case ' ':
					e.preventDefault();
					handlePlayPause();
					break;
				case 'ArrowLeft':
					e.preventDefault();
					handleSkipBack();
					break;
				case 'ArrowRight':
					e.preventDefault();
					handleSkipForward();
					break;
				case '=':
				case '+':
					handleVolumeChange(Math.min(1.0, playerState.volume + 0.1));
					break;
				case '-':
				case '_':
					handleVolumeChange(Math.max(0.0, playerState.volume - 0.1));
					break;
				case 'm':
				case 'M':
					if (playerState.volume > 0) {
						prevVolume = playerState.volume;
						handleVolumeChange(0);
					} else {
						handleVolumeChange(prevVolume);
					}
					break;
				case '[':
					handleSpeedChange(Math.max(0.5, playerState.speed - 0.05));
					break;
				case ']':
					handleSpeedChange(Math.min(3.0, playerState.speed + 0.05));
					break;
				case '1':
					speedPreset(1.0);
					break;
				case '2':
					speedPreset(1.25);
					break;
				case '3':
					speedPreset(1.5);
					break;
				case '4':
					speedPreset(2.0);
					break;
				case '5':
					speedPreset(2.5);
					break;
				case '6':
					speedPreset(3.0);
					break;
			}
		}

		window.addEventListener('keydown', handleKeyDown);

		// Guard: only init Tauri engine when running inside Tauri
		const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
		if (isTauri) {
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
		}

		return () => {
			window.removeEventListener('keydown', handleKeyDown);
			clearInterval(heartbeatHandle);
			engine?.destroy();
		};
	});

	function progressCtx() {
		if (!playerState.currentSections.length) return undefined;
		return buildProgressContext(
			playerState.currentSections,
			playerState.currentChapterIndex,
		);
	}

	function handlePlayPause() {
		if (!engine || !playerState.currentContentId) return;
		if (playerState.isPlaying) {
			engine.pause(
				playerState.currentDomainId,
				playerState.currentContentId,
				progressCtx(),
			);
		} else {
			engine.play(
				playerState.currentSource!,
				playerState.currentContentId,
				playerState.positionMs,
			);
			const savedSpeed = localStorage.getItem('speed_' + playerState.currentContentId);
			if (savedSpeed) {
				engine.setSpeed(parseFloat(savedSpeed));
			}
		}
	}

	function handleSeek(ms: number) {
		if (!engine || !playerState.currentContentId) return;
		engine.seek(
			playerState.currentDomainId,
			playerState.currentContentId,
			ms,
			progressCtx(),
		);
	}

	function handleSkipBack() {
		handleSeek(Math.max(0, playerState.positionMs - 30_000));
	}

	function speedPreset(rate: number) {
		engine?.setSpeed(rate);
		if (playerState.currentContentId) {
			localStorage.setItem('speed_' + playerState.currentContentId, rate.toString());
		}
	}

	function handleSkipForward() {
		handleSeek(Math.min(playerState.durationMs, playerState.positionMs + 15_000));
	}

	function handleSpeedChange(rate: number) {
		engine?.setSpeed(rate);
		if (playerState.currentContentId) {
			localStorage.setItem('speed_' + playerState.currentContentId, rate.toString());
		}
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
	<ErrorBoundary>
		{@render children()}
	</ErrorBoundary>
</ShellLayout>

{#if showShortcuts}
	<!-- Keyboard shortcuts overlay -->
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/60"
		onclick={() => (showShortcuts = false)}
	>
		<div
			class="bg-gray-900 border border-white/10 rounded-xl p-6 max-w-sm w-full mx-4 shadow-2xl"
			role="dialog"
			aria-label="Keyboard shortcuts"
			tabindex="-1"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.key === 'Escape' && (showShortcuts = false)}
		>
			<h2 class="text-lg font-bold mb-4">Keyboard Shortcuts</h2>
			<table class="w-full text-sm">
				<tbody class="divide-y divide-white/5">
					<tr><td class="py-1.5 text-gray-400"><kbd class="px-1.5 py-0.5 bg-white/10 rounded text-xs">Space</kbd></td><td class="py-1.5 text-right">Play / Pause</td></tr>
					<tr><td class="py-1.5 text-gray-400"><kbd class="px-1.5 py-0.5 bg-white/10 rounded text-xs">←</kbd></td><td class="py-1.5 text-right">Skip back 30s</td></tr>
					<tr><td class="py-1.5 text-gray-400"><kbd class="px-1.5 py-0.5 bg-white/10 rounded text-xs">→</kbd></td><td class="py-1.5 text-right">Skip forward 15s</td></tr>
					<tr><td class="py-1.5 text-gray-400"><kbd class="px-1.5 py-0.5 bg-white/10 rounded text-xs">=</kbd> <kbd class="px-1.5 py-0.5 bg-white/10 rounded text-xs">-</kbd></td><td class="py-1.5 text-right">Volume up / down</td></tr>
					<tr><td class="py-1.5 text-gray-400"><kbd class="px-1.5 py-0.5 bg-white/10 rounded text-xs">M</kbd></td><td class="py-1.5 text-right">Mute / Unmute</td></tr>
					<tr><td class="py-1.5 text-gray-400"><kbd class="px-1.5 py-0.5 bg-white/10 rounded text-xs">[</kbd> <kbd class="px-1.5 py-0.5 bg-white/10 rounded text-xs">]</kbd></td><td class="py-1.5 text-right">Speed - / +</td></tr>
					<tr><td class="py-1.5 text-gray-400"><kbd class="px-1.5 py-0.5 bg-white/10 rounded text-xs">1</kbd>-<kbd class="px-1.5 py-0.5 bg-white/10 rounded text-xs">6</kbd></td><td class="py-1.5 text-right">Speed presets</td></tr>
					<tr><td class="py-1.5 text-gray-400"><kbd class="px-1.5 py-0.5 bg-white/10 rounded text-xs">H</kbd></td><td class="py-1.5 text-right">Home</td></tr>
					<tr><td class="py-1.5 text-gray-400"><kbd class="px-1.5 py-0.5 bg-white/10 rounded text-xs">S</kbd></td><td class="py-1.5 text-right">Search</td></tr>
					<tr><td class="py-1.5 text-gray-400"><kbd class="px-1.5 py-0.5 bg-white/10 rounded text-xs">L</kbd></td><td class="py-1.5 text-right">Library</td></tr>
					<tr><td class="py-1.5 text-gray-400"><kbd class="px-1.5 py-0.5 bg-white/10 rounded text-xs">?</kbd></td><td class="py-1.5 text-right">Toggle this help</td></tr>
				</tbody>
			</table>
			<button
				onclick={() => (showShortcuts = false)}
				class="mt-4 w-full px-4 py-2 bg-white/10 rounded-lg text-sm font-medium hover:bg-white/20 transition-colors"
			>
				Close
			</button>
		</div>
	</div>
{/if}

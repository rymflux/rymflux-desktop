<script lang="ts">
	import { getPlayerState } from '../stores/playerStore.svelte';
	import Icon from './Icon.svelte';
	let { onPlayPause, onSkipBack, onSkipForward, onSpeedChange } = $props<{
		onPlayPause?: () => void;
		onSkipBack?: () => void;
		onSkipForward?: () => void;
		onSpeedChange?: (rate: number) => void;
	}>();

	let player = getPlayerState();
	let showSpeedMenu = $state(false);
	let speeds = [0.75, 1.0, 1.25, 1.5, 2.0, 2.5, 3.0];

	function togglePlayPause() {
		onPlayPause?.();
	}

	function speedLabel(r: number): string {
		return r === 1.0 ? '1×' : `${r}×`;
	}

	function selectSpeed(r: number) {
		onSpeedChange?.(r);
		showSpeedMenu = false;
	}
</script>

<div class="flex items-center gap-1">
	<!-- Skip back 30s -->
	<button
		onclick={() => onSkipBack?.()}
		class="icon-btn-sm text-sm font-medium"
		style="color: var(--text-primary);"
		aria-label="Skip back 30 seconds"
	>
		-30s
	</button>

	<!-- Play/Pause -->
	<button
		onclick={togglePlayPause}
		class="icon-btn"
		style="color: var(--text-primary);"
		aria-label={player.isPlaying ? 'Pause' : 'Play'}
	>
		{#if player.isPlaying}
			<Icon name="pause" size={24} class="w-6 h-6" />
		{:else}
			<Icon name="play" size={24} class="w-6 h-6" />
		{/if}
	</button>

	<!-- Skip forward 15s -->
	<button
		onclick={() => onSkipForward?.()}
		class="icon-btn-sm text-sm font-medium"
		style="color: var(--text-primary);"
		aria-label="Skip forward 15 seconds"
	>
		+15s
	</button>

	<!-- Speed selector -->
	<div class="relative">
		<button
			onclick={() => (showSpeedMenu = !showSpeedMenu)}
			class="icon-btn-sm text-xs font-semibold min-w-[2.5rem] {showSpeedMenu
				? 'bg-blue-600/20 text-blue-300'
				: ''}"
			style="color: {showSpeedMenu ? '' : 'var(--text-primary)'};"
			aria-label="Playback speed"
		>
			{speedLabel(player.speed)}
		</button>
		{#if showSpeedMenu}
			<div
				class="absolute bottom-full left-1/2 -translate-x-1/2 mb-2 shadow-xl py-1 rounded-lg border"
				style="background-color: var(--bg-secondary); border-color: var(--border);"
			>
				{#each speeds as r (r)}
					<button
						onclick={() => selectSpeed(r)}
						class="block w-full px-4 py-1 text-xs text-left whitespace-nowrap hover:bg-white/10"
						style="color: {r === player.speed
							? 'var(--accent, #60a5fa)'
							: 'var(--text-secondary)'}; font-weight: {r === player.speed ? 'bold' : 'normal'};"
					>
						{speedLabel(r)}
					</button>
				{/each}
			</div>
		{/if}
	</div>
</div>

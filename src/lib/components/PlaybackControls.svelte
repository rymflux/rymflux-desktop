<script lang="ts">
	import { getPlayerState } from '$lib/stores/playerStore.svelte';
	let { onPlayPause, onSkipBack, onSkipForward, onSpeedChange } = $props<{
		onPlayPause?: () => void;
		onSkipBack?: () => void;
		onSkipForward?: () => void;
		onSpeedChange?: (rate: number) => void;
	}>();

	let player = getPlayerState();
	let showSpeedMenu = $state(false);
	let speeds = [0.75, 1.0, 1.25, 1.5, 2.0];

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

<div class="flex items-center gap-0.5">
	<!-- Skip back 30s -->
	<button
		onclick={() => onSkipBack?.()}
		class="p-1.5 rounded-full hover:bg-white/10 transition-colors text-sm font-medium"
		aria-label="Skip back 30 seconds"
	>
		-30s
	</button>

	<!-- Play/Pause -->
	<button
		onclick={togglePlayPause}
		class="p-2 rounded-full hover:bg-white/10 transition-colors"
		aria-label={player.isPlaying ? 'Pause' : 'Play'}
	>
		{#if player.isPlaying}
			<!-- pause icon -->
			<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-6 h-6">
				<rect x="5" y="4" width="5" height="16" rx="1" />
				<rect x="14" y="4" width="5" height="16" rx="1" />
			</svg>
		{:else}
			<!-- play icon -->
			<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-6 h-6">
				<polygon points="6,4 20,12 6,20" />
			</svg>
		{/if}
	</button>

	<!-- Skip forward 15s -->
	<button
		onclick={() => onSkipForward?.()}
		class="p-1.5 rounded-full hover:bg-white/10 transition-colors text-sm font-medium"
		aria-label="Skip forward 15 seconds"
	>
		+15s
	</button>

	<!-- Speed selector -->
	<div class="relative">
		<button
			onclick={() => (showSpeedMenu = !showSpeedMenu)}
			class="p-1.5 rounded-full hover:bg-white/10 transition-colors text-xs font-semibold min-w-[2.5rem]"
			aria-label="Playback speed"
		>
			{speedLabel(player.speed)}
		</button>
		{#if showSpeedMenu}
			<div
				class="absolute bottom-full left-1/2 -translate-x-1/2 mb-2 bg-gray-800 border border-white/10 rounded-lg shadow-xl py-1"
			>
				{#each speeds as r (r)}
					<button
						onclick={() => selectSpeed(r)}
						class="block w-full px-4 py-1 text-xs hover:bg-white/10 text-left whitespace-nowrap {r === player.speed ? 'text-blue-400 font-bold' : 'text-white/80'}"
					>
						{speedLabel(r)}
					</button>
				{/each}
			</div>
		{/if}
	</div>
</div>

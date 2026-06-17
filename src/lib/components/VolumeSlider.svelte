<script lang="ts">
	let { volume = 1, onVolumeChange }: { volume: number; onVolumeChange?: (v: number) => void } = $props();
	let muted = $state(false);
	let prevVolume = $state(1);

	function handleInput(e: Event) {
		const el = e.currentTarget as HTMLInputElement;
		const v = parseFloat(el.value);
		volume = v;
		muted = v === 0;
		onVolumeChange?.(v);
	}

	function toggleMute() {
		if (muted) {
			muted = false;
			onVolumeChange?.(prevVolume);
		} else {
			prevVolume = volume;
			muted = true;
			onVolumeChange?.(0);
		}
	}
</script>

<div class="flex items-center gap-1.5">
	<button
		onclick={toggleMute}
		class="p-1 rounded hover:bg-white/10 transition-colors"
		aria-label={muted ? 'Unmute' : 'Mute'}
	>
		{#if muted || volume === 0}
			<!-- muted icon -->
			<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-4 h-4">
				<path d="M3 9v6h4l5 5V4L7 9H3zm13.5 3A4.5 4.5 0 0 0 14 8.5v7a4.49 4.49 0 0 0 2.5-3.5z" />
				<line x1="22" y1="2" x2="2" y2="22" stroke="currentColor" stroke-width="2" />
			</svg>
		{:else if volume < 0.5}
			<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-4 h-4">
				<path d="M3 9v6h4l5 5V4L7 9H3zm13.5 3A4.5 4.5 0 0 0 14 8.5v7a4.49 4.49 0 0 0 2.5-3.5z" />
			</svg>
		{:else}
			<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-4 h-4">
				<path d="M3 9v6h4l5 5V4L7 9H3zm13.5 3c0-1.77-1.02-3.29-2.5-4.03v8.05c1.48-.73 2.5-2.25 2.5-4.02z" />
			</svg>
		{/if}
	</button>
	<input
		type="range"
		min={0}
		max={1}
		step={0.01}
		value={muted ? 0 : volume}
		oninput={handleInput}
		class="w-20 h-1 accent-blue-500"
		aria-label="Volume"
	/>
</div>

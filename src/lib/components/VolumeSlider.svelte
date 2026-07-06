<script lang="ts">
	import Icon from './Icon.svelte';
	let { volume = 1, onVolumeChange }: { volume: number; onVolumeChange?: (v: number) => void } =
		$props();
	let muted = $state(false);
	let prevVolume = $state(1);

	function handleInput(e: Event) {
		const el = e.currentTarget as HTMLInputElement;
		const v = parseFloat(el.value);
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
		class="icon-btn-sm"
		style="color: var(--text-primary);"
		aria-label={muted ? 'Unmute' : 'Mute'}
	>
		{#if muted || volume === 0}
			<Icon name="volume-x" size={16} class="w-4 h-4" />
		{:else if volume < 0.5}
			<Icon name="volume-1" size={16} class="w-4 h-4" />
		{:else}
			<Icon name="volume-2" size={16} class="w-4 h-4" />
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

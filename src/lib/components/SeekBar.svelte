<script lang="ts">
	let { progress = 0, onSeek }: { progress: number; onSeek?: (fraction: number) => void } = $props();
	let dragging = $state(false);
	let localFraction = $state(0);

	function handleInput(e: Event) {
		const el = e.currentTarget as HTMLInputElement;
		localFraction = parseFloat(el.value);
		onSeek?.(localFraction);
	}

	function handleStart() {
		dragging = true;
	}

	function handleEnd() {
		dragging = false;
	}
</script>

<div class="relative h-2 w-full group cursor-pointer">
	<!-- background track -->
	<div class="absolute inset-0 top-1/2 -translate-y-1/2 h-1 rounded-full bg-white/20"></div>
	<!-- filled track -->
	<div
		class="absolute left-0 top-1/2 -translate-y-1/2 h-1 rounded-full bg-blue-500 transition-all"
		style="width: {dragging ? localFraction * 100 : progress * 100}%"
	></div>
	<input
		type="range"
		min={0}
		max={1}
		step={0.001}
		value={dragging ? localFraction : progress}
		oninput={handleInput}
		onmousedown={handleStart}
		ontouchend={handleEnd}
		onmouseup={handleEnd}
		onchange={handleEnd}
		class="absolute inset-0 w-full h-full opacity-0 cursor-pointer"
		aria-label="Seek position"
	/>
</div>

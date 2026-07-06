<script lang="ts">
	let { progress = 0, onSeek }: { progress: number; onSeek?: (fraction: number) => void } =
		$props();
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

	$effect(() => {
		if (dragging) {
			const onUp = () => {
				dragging = false;
			};
			const onTouchEnd = () => {
				dragging = false;
			};
			window.addEventListener('mouseup', onUp);
			window.addEventListener('touchend', onTouchEnd);
			return () => {
				window.removeEventListener('mouseup', onUp);
				window.removeEventListener('touchend', onTouchEnd);
			};
		}
	});
</script>

<div class="relative h-2 w-full group cursor-pointer">
	<!-- background track -->
	<div
		class="absolute inset-0 top-1/2 -translate-y-1/2 h-1 rounded-full"
		style="background-color: var(--bg-elevated);"
	></div>
	<!-- filled track -->
	<div
		class="absolute left-0 top-1/2 -translate-y-1/2 h-1 rounded-full transition-all"
		style="width: {dragging
			? localFraction * 100
			: progress * 100}%; background-color: var(--accent, #3b82f6);"
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
		onmouseleave={handleEnd}
		onchange={handleEnd}
		ontouchstart={handleStart}
		class="absolute inset-0 w-full h-full opacity-0 cursor-pointer"
		aria-label="Seek position"
	/>
</div>

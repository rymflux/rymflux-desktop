<script lang="ts">
	let { seconds = 0 }: { seconds: number } = $props();

	function fmt(s: number): string {
		const h = Math.floor(s / 3600);
		const m = Math.floor((s % 3600) / 60);
		const sec = Math.floor(s % 60);
		if (h > 0) return `${h}:${String(m).padStart(2, '0')}:${String(sec).padStart(2, '0')}`;
		return `${m}:${String(sec).padStart(2, '0')}`;
	}
</script>

<span class="font-mono text-xs tabular-nums">{fmt(seconds)}</span>

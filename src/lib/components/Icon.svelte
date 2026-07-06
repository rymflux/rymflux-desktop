<script lang="ts">
	import {
		Play,
		Pause,
		Menu,
		Moon,
		Sun,
		Volume,
		Volume1,
		Volume2,
		VolumeX,
		ChevronRight,
		X,
		Info,
		TriangleAlert,
		CircleX
	} from '@lucide/svelte';

	let {
		name,
		size = 16,
		class: className = '',
		color
	}: {
		name: string;
		size?: number;
		class?: string;
		color?: string;
	} = $props();

	const icons: Record<string, import('svelte').Component> = {
		play: Play,
		pause: Pause,
		menu: Menu,
		moon: Moon,
		sun: Sun,
		volume: Volume,
		'volume-1': Volume1,
		'volume-2': Volume2,
		'volume-x': VolumeX,
		'chevron-right': ChevronRight,
		x: X,
		info: Info,
		warning: TriangleAlert,
		error: CircleX
	};

	let Cmp = $derived(icons[name]);
</script>

{#if Cmp}
	<Cmp {size} class={className} {color} />
{/if}

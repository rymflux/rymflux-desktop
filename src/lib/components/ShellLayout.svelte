<script lang="ts">
	import Sidebar from './Sidebar.svelte';
	import PlayerBar from './PlayerBar.svelte';
	import { getUiState } from '$lib/stores/uiStore';

	let {
		children,
		onPlayPause,
		onSeek,
		onSkipBack,
		onSkipForward,
		onSpeedChange,
		onVolumeChange,
	}: {
		children: import('svelte').Snippet;
		onPlayPause?: () => void;
		onSeek?: (ms: number) => void;
		onSkipBack?: () => void;
		onSkipForward?: () => void;
		onSpeedChange?: (rate: number) => void;
		onVolumeChange?: (v: number) => void;
	} = $props();

	let ui = getUiState();
</script>

<div class="h-screen flex flex-col bg-gray-900 text-white overflow-hidden">
	<div class="flex flex-1 min-h-0">
		<Sidebar />

		<!-- Hamburger toggle for small screens -->
		<button
			onclick={() => (ui.sidebarOpen = !ui.sidebarOpen)}
			class="absolute top-2 left-2 z-50 p-1.5 rounded-md hover:bg-white/10 md:hidden"
			aria-label="Toggle sidebar"
		>
			<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-5 h-5">
				<path d="M3 6h18v2H3V6zm0 5h18v2H3v-2zm0 5h18v2H3v-2z" />
			</svg>
		</button>

		<main class="flex-1 overflow-y-auto p-6">
			{@render children()}
		</main>
	</div>

	<PlayerBar
		{onPlayPause}
		{onSeek}
		{onSkipBack}
		{onSkipForward}
		{onSpeedChange}
		{onVolumeChange}
	/>
</div>

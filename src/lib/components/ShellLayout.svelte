<script lang="ts">
	import Sidebar from './Sidebar.svelte';
	import PlayerBar from './PlayerBar.svelte';
	import ToastContainer from './ToastContainer.svelte';
	import Icon from './Icon.svelte';
	import { getUiState } from '../stores/uiStore.svelte';

	let {
		children,
		onPlayPause,
		onSeek,
		onSkipBack,
		onSkipForward,
		onSpeedChange,
		onVolumeChange
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

<div
	class="h-screen flex flex-col overflow-hidden"
	data-theme={ui.theme}
	style="background-color: var(--bg-primary); color: var(--text-primary);"
>
	<ToastContainer />
	<div class="flex flex-1 min-h-0">
		<Sidebar />

		<!-- Hamburger toggle for small screens -->
		<button
			onclick={() => (ui.sidebarOpen = !ui.sidebarOpen)}
			class="absolute top-2 left-2 z-50 p-1.5 rounded-md md:hidden icon-btn-sm"
			aria-label="Toggle sidebar"
		>
			<Icon name="menu" size={20} class="w-5 h-5" />
		</button>

		<main class="flex-1 overflow-y-auto p-6">
			{@render children()}
		</main>
	</div>

	<PlayerBar {onPlayPause} {onSeek} {onSkipBack} {onSkipForward} {onSpeedChange} {onVolumeChange} />
</div>

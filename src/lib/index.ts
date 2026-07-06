export { default as ShellLayout } from './components/ShellLayout.svelte';
export { default as PlayerBar } from './components/PlayerBar.svelte';
export { default as Sidebar } from './components/Sidebar.svelte';
export { default as ErrorBoundary } from './components/ErrorBoundary.svelte';
export { default as LoadingSpinner } from './components/LoadingSpinner.svelte';
export { default as CoverImage } from './components/CoverImage.svelte';
export { default as TimeDisplay } from './components/TimeDisplay.svelte';
export { default as SeekBar } from './components/SeekBar.svelte';
export { default as PlaybackControls } from './components/PlaybackControls.svelte';
export { default as VolumeSlider } from './components/VolumeSlider.svelte';
export { default as ToastContainer } from './components/ToastContainer.svelte';
export { default as Icon } from './components/Icon.svelte';

export {
	getPlayerState,
	updatePlaybackState,
	setCurrentTrack,
	setChapterIndex
} from './stores/playerStore.svelte';
export { getUiState, setTheme } from './stores/uiStore.svelte';
export { diag, isDiagMode, setDiagMode, toggleDiagMode } from './utils/diag.svelte';
export { getToasts, addToast, removeToast } from './stores/toastStore.svelte';
export type { Toast, ToastType } from './stores/toastStore.svelte';

export type * from './types/ipc.js';

export { createDomainRegistry, getDomainRegistry } from './registry/index.js';
export type {
	DomainManifest,
	LibraryViewProps,
	DetailViewProps,
	PlayerViewProps
} from './registry/index.js';

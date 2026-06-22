let sidebarOpen = $state(true);
const theme = $state<'light' | 'dark'>('dark');
let viewMode = $state<'grid' | 'list'>('grid');

export function getUiState() {
	return {
		get sidebarOpen() {
			return sidebarOpen;
		},
		set sidebarOpen(v: boolean) {
			sidebarOpen = v;
		},
		get theme() {
			return theme;
		},
		get viewMode() {
			return viewMode;
		},
		set viewMode(v: 'grid' | 'list') {
			viewMode = v;
		},
	};
}

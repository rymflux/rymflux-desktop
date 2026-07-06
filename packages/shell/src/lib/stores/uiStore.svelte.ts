const THEME_KEY = 'rymflux:theme';

function loadTheme(): 'light' | 'dark' {
	if (typeof localStorage === 'undefined') return 'dark';
	const stored = localStorage.getItem(THEME_KEY);
	if (stored === 'light' || stored === 'dark') return stored;
	return 'dark';
}

let sidebarOpen = $state(true);
let theme = $state<'light' | 'dark'>(loadTheme());
let viewMode = $state<'grid' | 'list'>('grid');

export function setTheme(v: 'light' | 'dark') {
	theme = v;
	if (typeof localStorage !== 'undefined') {
		localStorage.setItem(THEME_KEY, v);
	}
}

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
		}
	};
}

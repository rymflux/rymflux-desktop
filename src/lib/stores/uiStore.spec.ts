import { describe, it, expect } from 'vitest';
import { getUiState, setTheme } from './uiStore.svelte';

describe('uiStore', () => {
	it('returns initial defaults', () => {
		const state = getUiState();
		expect(state.sidebarOpen).toBe(true);
		expect(['dark', 'light']).toContain(state.theme);
		expect(state.viewMode).toBe('grid');
	});

	it('setTheme changes the theme', () => {
		const before = getUiState().theme;
		const newTheme = before === 'dark' ? 'light' : 'dark';

		setTheme(newTheme);

		expect(getUiState().theme).toBe(newTheme);
	});

	it('sidebarOpen can be toggled', () => {
		const state = getUiState();
		state.sidebarOpen = false;
		expect(getUiState().sidebarOpen).toBe(false);

		state.sidebarOpen = true;
		expect(getUiState().sidebarOpen).toBe(true);
	});

	it('viewMode can be changed', () => {
		const state = getUiState();
		state.viewMode = 'list';
		expect(getUiState().viewMode).toBe('list');

		state.viewMode = 'grid';
		expect(getUiState().viewMode).toBe('grid');
	});
});

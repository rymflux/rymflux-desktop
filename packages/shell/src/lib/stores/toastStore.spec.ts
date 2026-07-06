import { describe, it, expect } from 'vitest';
import { addToast, getToasts, removeToast } from './toastStore.svelte';

describe('toastStore', () => {
	it('starts with no toasts', () => {
		expect(Array.isArray(getToasts())).toBe(true);
	});

	it('addToast accepts a toast and returns an ID', () => {
		const id = addToast('Hello world', 'info');
		expect(typeof id).toBe('number');

		const toasts = getToasts();
		const found = toasts.find((t) => t.id === id);
		expect(found).toBeDefined();
		expect(found!.message).toBe('Hello world');
		expect(found!.type).toBe('info');
	});

	it('removeToast removes a toast by ID', () => {
		const id1 = addToast('first', 'info');
		const id2 = addToast('second', 'warning');

		removeToast(id1);

		const toasts = getToasts();
		expect(toasts.find((t) => t.id === id1)).toBeUndefined();
		expect(toasts.find((t) => t.id === id2)).toBeDefined();
	});

	it('enforces the max toast limit', () => {
		const ids: number[] = [];
		for (let i = 0; i < 7; i++) {
			ids.push(addToast(`toast ${i}`, 'info'));
		}

		const toasts = getToasts();
		expect(toasts.length).toBeLessThanOrEqual(5);
		for (let i = 2; i < 7; i++) {
			expect(toasts.find((t) => t.id === ids[i])).toBeDefined();
		}
	});

	it('supports warning and error toast types', () => {
		const idWarn = addToast('Warning!', 'warning');
		const idErr = addToast('Error!', 'error');

		const toasts = getToasts();
		expect(toasts.find((t) => t.id === idWarn)?.type).toBe('warning');
		expect(toasts.find((t) => t.id === idErr)?.type).toBe('error');
	});
});

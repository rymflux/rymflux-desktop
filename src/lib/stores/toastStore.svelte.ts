export type ToastType = 'error' | 'warning' | 'info';

export interface Toast {
	id: number;
	message: string;
	type: ToastType;
}

let nextId = 0;
let toasts = $state<Toast[]>([]);

const MAX_TOASTS = 5;
const DISMISS_MS = 5_000;

function removeToast(id: number) {
	toasts = toasts.filter((t) => t.id !== id);
}

export function addToast(message: string, type: ToastType = 'info') {
	const id = nextId++;
	toasts = [...toasts, { id, message, type }];

	// Evict oldest if over limit
	if (toasts.length > MAX_TOASTS) {
		toasts = toasts.slice(toasts.length - MAX_TOASTS);
	}

	// Auto-dismiss after 5s
	setTimeout(() => removeToast(id), DISMISS_MS);

	return id;
}

export function getToasts() {
	return toasts;
}

export { removeToast };

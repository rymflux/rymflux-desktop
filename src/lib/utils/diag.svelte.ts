let diagMode = $state(false);

export function isDiagMode(): boolean {
	return diagMode;
}

export function setDiagMode(v: boolean) {
	diagMode = v;
}

export function toggleDiagMode() {
	diagMode = !diagMode;
}

/** Gated diagnostic logging utility.
 *
 * `diag.error` always fires (replaces `console.error` for centralized
 * error reporting). `diag.log` and `diag.warn` are gated behind the
 * `isDiagMode` flag — silent when off.
 */
export const diag = {
	error(...args: unknown[]) {
		console.error(...args);
	},
	log(...args: unknown[]) {
		if (diagMode) console.log('[diag]', ...args);
	},
	warn(...args: unknown[]) {
		if (diagMode) console.warn('[diag]', ...args);
	}
};

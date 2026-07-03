import { invoke } from '@tauri-apps/api/core';

export async function setBackendDiagMode(enabled: boolean): Promise<void> {
	await invoke('set_diag_mode', { enabled });
}

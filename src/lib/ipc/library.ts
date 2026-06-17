import { invoke } from '@tauri-apps/api/core';
import type { ContentItem, ProgressRecord } from '$lib/types/ipc';

export async function listLibrary(domainId: string): Promise<ContentItem[]> {
	return invoke('library_list', { domainId });
}

export async function searchLibrary(
	domainId: string,
	query: string,
): Promise<ContentItem[]> {
	return invoke('library_search', { domainId, query });
}

export async function getProgress(contentId: string): Promise<ProgressRecord> {
	return invoke('progress_get', { contentId });
}

export async function setProgress(
	domainId: string,
	contentId: string,
	positionMs: number,
): Promise<void> {
	return invoke('progress_set', { domainId, contentId, positionMs });
}

export async function syncProgress(
	domainId: string,
): Promise<ProgressRecord[]> {
	return invoke('progress_sync', { domainId });
}

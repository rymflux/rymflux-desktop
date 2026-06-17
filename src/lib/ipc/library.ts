import { invoke } from '@tauri-apps/api/core';
import type { ContentItem, ProgressRecord, DomainItem } from '$lib/types/ipc';

/** Transform raw core ContentItem into frontend-friendly DomainItem */
function toDomainItem(raw: Record<string, unknown>): DomainItem {
	const meta = (raw.metadata_json as Record<string, unknown>) || {};
	return {
		content_id: raw.id as string,
		domain_id: raw.domain_id as string,
		title: (meta.title as string) || '',
		author: (meta.author as string) || '',
		source_uri: (raw.source_uri as string) || '',
		description: (meta.description as string) || '',
		duration_ms: meta.total_time_secs != null ? (meta.total_time_secs as number) * 1000 : null,
		cover_url: (meta.cover_url as string) || null,
		added_at: String(raw.added_at ?? ''),
	};
}

/** Transform raw core ContentItem[] into DomainItem[] */
function toDomainItems(raw: unknown[]): DomainItem[] {
	return raw.map((r) => toDomainItem(r as Record<string, unknown>));
}

export async function listLibrary(domainId: string): Promise<DomainItem[]> {
	const raw = await invoke('library_list', { domainId });
	return toDomainItems(raw as unknown[]);
}

export async function searchLibrary(
	domainId: string,
	query: string,
): Promise<DomainItem[]> {
	const raw = await invoke('library_search', { domainId, query });
	return toDomainItems(raw as unknown[]);
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

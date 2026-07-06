import { invoke } from '@tauri-apps/api/core';
import type { ContentItem, DomainRecord, ProgressRecord, DomainItem } from '$lib/types/ipc';

/** Store a ContentItem (built by a domain) into the library database. */
export async function storeItem(
	contentItem: ContentItem,
	identitySourceId: string,
	identityDurationMs: number | null,
): Promise<void> {
	await invoke('library_store_item', {
		contentItem,
		identitySourceId,
		identityDurationMs,
	});
}

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

export async function listDomains(): Promise<DomainRecord[]> {
	return invoke('library_list_domains') as Promise<DomainRecord[]>;
}

export async function countContent(domainId: string): Promise<number> {
	return invoke('library_count_content', { domainId }) as Promise<number>;
}

export async function countAllContent(): Promise<Record<string, number>> {
	return invoke('library_count_all') as Promise<Record<string, number>>;
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

export async function removeFromLibrary(contentId: string): Promise<void> {
	return invoke('library_remove_from', { contentId });
}

export async function clearLibrary(domainId: string): Promise<void> {
	return invoke('library_clear', { domainId });
}

/** Check whether a content item exists in the library. Returns null if not found. */
export async function getLibraryDetail(contentId: string): Promise<ContentItem | null> {
	try {
		return (await invoke('library_get_detail', { contentId })) as ContentItem;
	} catch {
		return null;
	}
}

export async function getProgress(contentId: string): Promise<ProgressRecord> {
	return invoke('progress_get', { contentId });
}

export async function setProgress(
	domainId: string,
	contentId: string,
	positionMs: number,
	ctx?: { chapter_index: number; chapter_offset_ms: number },
	speed?: number,
): Promise<void> {
	return invoke('progress_set', {
		domainId,
		contentId,
		positionMs,
		chapterIndex: ctx?.chapter_index ?? null,
		chapterOffsetMs: ctx?.chapter_offset_ms ?? null,
		speed: speed ?? null,
	});
}

export async function syncProgress(
	domainId: string,
): Promise<ProgressRecord[]> {
	return invoke('progress_sync', { domainId });
}

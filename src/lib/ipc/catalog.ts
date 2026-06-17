import { invoke } from '@tauri-apps/api/core';
import type { CatalogItem, CatalogDetail } from '$lib/types/ipc';

export async function searchCatalog(
	query: string,
	limit?: number,
	offset?: number,
): Promise<CatalogItem[]> {
	return invoke('catalog_search', { query, limit, offset });
}

export async function getBook(id: string): Promise<CatalogDetail> {
	return invoke('catalog_get_book', { id });
}

export async function addToLibrary(catalogId: string): Promise<void> {
	return invoke('library_add_from_catalog', { catalogId });
}

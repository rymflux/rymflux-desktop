import type { Component } from 'svelte';
import type { AudioSource, CatalogDetail, ContentItem, DomainItem } from '../types/ipc.js';

export interface LibraryViewProps {
	items: DomainItem[];
	loading: boolean;
	progressMap: Map<string, number>;
	onSelect: (item: DomainItem) => void;
}

export type DetailViewProps = Record<string, unknown>;
export type PlayerViewProps = Record<string, unknown>;

export interface DomainManifest {
	id: string;
	name: string;
	icon: string;
	views: {
		library: Component<LibraryViewProps>;
		detail: Component<DetailViewProps>;
		player: Component<PlayerViewProps>;
	};
	search?(
		query: string,
		searchType?: string,
		limit?: number,
		offset?: number
	): Promise<DomainItem[]>;
	getDetail?(id: string): Promise<CatalogDetail | null>;
	resolveSource?(listenUrl: string, durationMs: number): AudioSource;
	buildLibraryItem?(catalogId: string): Promise<ContentItem>;
}

const registry = new Map<string, DomainManifest>();

export function createDomainRegistry(domains: DomainManifest[]) {
	for (const d of domains) registry.set(d.id, d);
}

export function getDomainRegistry() {
	return registry;
}

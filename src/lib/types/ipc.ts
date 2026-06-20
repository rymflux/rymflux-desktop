// ── Core IPC types (mirrors rymflux_core::types) ──────────────────────────

export interface AudioSource {
	uri: string;
	duration_ms: number;
	mime_type: string;
}

export interface PlaybackState {
	position_ms: number;
	duration_ms: number;
	speed: number;
	volume: number;
	is_playing: boolean;
	is_loaded: boolean;
}

export interface ContentIdentity {
	identity_id: string;
	source_id: string | null;
	file_name: string | null;
	domain_id: string;
}

export interface ProgressRecord {
	domain_id: string;
	content_id: string;
	position_ms: number;
	extra: Record<string, unknown>;
	updated_at: string;
}

// ── Core ContentItem (raw shape from storage, snake_case from serde) ──────
// Display fields live in metadata_json; use DomainItem for UI consumption.

export interface ContentItem {
	id: string;
	domain_id: string;
	source_uri: string;
	metadata_json: Record<string, unknown> | null;
	content_hash: string;
	added_at: string;
}

// ── Domain record (mirrors rymflux_core::types::DomainRecord) ─────────────

export interface DomainRecord {
	id: string;
	display_name: string;
	version: string;
	enabled: boolean;
	install_path: string | null;
	permissions: string;
	added_at: number;
}

// ── Frontend display type (extracted from ContentItem.metadata_json) ──────

export interface DomainItem {
	content_id: string;
	domain_id: string;
	title: string;
	author: string;
	source_uri: string;
	description: string;
	duration_ms: number | null;
	cover_url: string | null;
	added_at: string;
}

// ── Catalog types (frontend-facing) ───────────────────────────────────────

export interface CatalogItem {
	id: string;
	title: string;
	author: string;
	description: string;
	total_time_secs: number | null;
	num_sections: number | null;
	cover_url: string | null;
}

export interface ChapterInfo {
	id: string;
	section_number: number;
	title: string;
	listen_url: string;
	playtime_secs: number | null;
}

export interface CatalogDetail {
	item: CatalogItem;
	sections: ChapterInfo[];
	archive_identifier: string | null;
}

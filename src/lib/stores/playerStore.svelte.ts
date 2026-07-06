import type { PlaybackState, AudioSource, ChapterInfo } from '../types/ipc.js';

let positionMs = $state(0);
let durationMs = $state(0);
let speed = $state(1.0);
let volume = $state(1.0);
let isPlaying = $state(false);
let isLoaded = $state(false);
let currentSource = $state<AudioSource | null>(null);
let currentContentId = $state<string | null>(null);
let currentDomainId = $state<string>('');
let currentCoverUrl = $state<string | null>(null);
let currentChapterIndex = $state<number>(0);
let currentSections = $state<ChapterInfo[]>([]);
let currentTitle = $state('');

const remainingMs = $derived(durationMs - positionMs);
const progressFraction = $derived(durationMs > 0 ? positionMs / durationMs : 0);

export function getPlayerState() {
	return {
		get positionMs() {
			return positionMs;
		},
		get durationMs() {
			return durationMs;
		},
		get speed() {
			return speed;
		},
		get volume() {
			return volume;
		},
		get isPlaying() {
			return isPlaying;
		},
		get isLoaded() {
			return isLoaded;
		},
		get currentSource() {
			return currentSource;
		},
		get currentContentId() {
			return currentContentId;
		},
		get currentCoverUrl() {
			return currentCoverUrl;
		},
		get currentChapterIndex() {
			return currentChapterIndex;
		},
		get currentSections() {
			return currentSections;
		},
		get currentDomainId() {
			return currentDomainId;
		},
		get currentTitle() {
			return currentTitle;
		},
		get remainingMs() {
			return remainingMs;
		},
		get progressFraction() {
			return progressFraction;
		}
	};
}

export function updatePlaybackState(s: PlaybackState) {
	positionMs = s.position_ms;
	durationMs = s.duration_ms;
	speed = s.speed;
	volume = s.volume;
	isPlaying = s.is_playing;
	isLoaded = s.is_loaded;
}

export function setCurrentTrack(
	source: AudioSource,
	contentId: string,
	title?: string,
	domainId?: string,
	coverUrl?: string | null,
	chapterIndex?: number,
	sections?: ChapterInfo[]
) {
	currentSource = source;
	currentContentId = contentId;
	if (title) currentTitle = title;
	if (domainId) currentDomainId = domainId;
	if (coverUrl !== undefined) currentCoverUrl = coverUrl;
	if (chapterIndex !== undefined) currentChapterIndex = chapterIndex;
	if (sections) currentSections = sections;
}

export function setChapterIndex(i: number) {
	currentChapterIndex = i;
}

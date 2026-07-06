import type { ChapterInfo } from '$lib/types/ipc';

export interface ProgressWriteContext {
	chapter_index: number;
	chapter_offset_ms: number;
}

/** Cumulative book offset (ms) at the start of `chapterIndex`. */
export function getChapterOffsetMs(sections: ChapterInfo[], chapterIndex: number): number {
	let offset = 0;
	for (let i = 0; i < chapterIndex; i++) {
		offset += (sections[i]?.playtime_secs ?? 0) * 1000;
	}
	return offset;
}

export function buildProgressContext(
	sections: ChapterInfo[],
	chapterIndex: number,
): ProgressWriteContext {
	return {
		chapter_index: chapterIndex,
		chapter_offset_ms: getChapterOffsetMs(sections, chapterIndex),
	};
}

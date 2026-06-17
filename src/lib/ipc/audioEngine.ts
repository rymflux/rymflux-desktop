import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import type { AudioSource, PlaybackState } from '$lib/types/ipc';

export class TauriAudioEngine {
	private _unlisten: UnlistenFn[] = [];

	async init(
		onProgress: (s: PlaybackState) => void,
		onFinished: () => void,
		onError: (e: string) => void,
	) {
		this._unlisten.push(
			await listen<PlaybackState>('audio:progress', (e) => onProgress(e.payload)),
			await listen<void>('audio:finished', () => onFinished()),
			await listen<string>('audio:error', (e) => onError(e.payload)),
		);
	}

	destroy() {
		for (const u of this._unlisten) u();
		this._unlisten = [];
	}

	play(source: AudioSource, contentId: string, positionMs: number): Promise<PlaybackState> {
		return invoke('play_audio', { source, contentId, positionMs });
	}

	pause(domainId: string, contentId: string): Promise<PlaybackState> {
		return invoke('pause_audio', { domainId, contentId });
	}

	seek(domainId: string, contentId: string, positionMs: number): Promise<PlaybackState> {
		return invoke('seek_audio', { domainId, contentId, positionMs });
	}

	setSpeed(rate: number): Promise<PlaybackState> {
		return invoke('set_audio_speed', { rate });
	}

	setVolume(volume: number): Promise<PlaybackState> {
		return invoke('set_audio_volume', { volume });
	}

	getState(): Promise<PlaybackState> {
		return invoke('get_audio_state');
	}

	stop(domainId: string, contentId: string): Promise<PlaybackState> {
		return invoke('stop_audio', { domainId, contentId });
	}
}

import { getContext, setContext } from 'svelte';
import type { TauriAudioEngine } from './audioEngine';

const ENGINE_KEY = Symbol('audioEngine');

export function setAudioEngine(engine: TauriAudioEngine) {
	setContext(ENGINE_KEY, engine);
}

export function getAudioEngine(): TauriAudioEngine {
	return getContext(ENGINE_KEY) as TauriAudioEngine;
}

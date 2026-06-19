<script lang="ts">
	import { type Snippet } from 'svelte';

	let {
		children,
		fallback,
		onError,
	}: {
		children: Snippet;
		fallback?: Snippet<[error: unknown, reset: () => void]>;
		onError?: (error: unknown, reset: () => void) => void;
	} = $props();

	let error = $state<unknown>(null);
	let resetFn: (() => void) | null = null;

	/** The <svelte:boundary> `onerror` callback — captures error and stores reset for retry. */
	function handleBoundaryError(e: unknown, reset: () => void) {
		error = e;
		resetFn = reset;
		onError?.(e, reset);
	}

	/** Clear the error and re-activate the boundary to retry children. */
	function handleRetry() {
		const r = resetFn;
		error = null;
		resetFn = null;
		r?.(); // Re-activate boundary — creates a fresh main effect for children
	}
</script>

<svelte:boundary onerror={handleBoundaryError}>
	{#if error}
		{#if fallback}
			{@render fallback(error, handleRetry)}
		{:else}
			<div class="flex flex-col items-center justify-center py-12 px-4 text-center">
				<div class="w-12 h-12 rounded-full bg-red-900/30 flex items-center justify-center mb-4">
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-6 h-6 text-red-400">
						<path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
					</svg>
				</div>
				<p class="text-gray-300 font-medium">Something went wrong</p>
				<p class="text-gray-500 text-sm mt-1 max-w-md">{String(error)}</p>
				<button
					onclick={handleRetry}
					class="mt-4 px-4 py-2 bg-white/10 rounded-lg text-sm text-gray-300 hover:bg-white/20 transition-colors"
				>
					Try again
				</button>
			</div>
		{/if}
	{:else}
		{@render children()}
	{/if}
</svelte:boundary>

<script lang="ts">
	import { onMount } from 'svelte';

	let {
		children,
		fallback,
		onError,
	}: {
		children: import('svelte').Snippet;
		fallback?: import('svelte').Snippet<[error: Error]>;
		onError?: (error: Error) => void;
	} = $props();

	let error = $state<Error | null>(null);
	let errorKey = $state(0);
	let element: HTMLDivElement | undefined = $state();

	function handleWindowError(e: ErrorEvent) {
		// Only catch errors that originated from within this boundary's subtree
		if (element && e.target && element.contains(e.target as Node)) {
			e.preventDefault();
			e.stopPropagation();
			error = e.error instanceof Error ? e.error : new Error(e.message || 'Unknown error');
			onError?.(error);
		}
	}

	onMount(() => {
		window.addEventListener('error', handleWindowError, true);
		return () => window.removeEventListener('error', handleWindowError, true);
	});

	function handleRetry() {
		error = null;
		errorKey += 1;
	}
</script>

<div bind:this={element} class="contents">
	{#if error}
		{#if fallback}
			{@render fallback(error)}
		{:else}
			<div class="flex flex-col items-center justify-center py-12 px-4 text-center">
				<div class="w-12 h-12 rounded-full bg-red-900/30 flex items-center justify-center mb-4">
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-6 h-6 text-red-400">
						<path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
					</svg>
				</div>
				<p class="text-gray-300 font-medium">Something went wrong</p>
				<p class="text-gray-500 text-sm mt-1 max-w-md">{error.message || 'An unexpected error occurred'}</p>
				<button
					onclick={handleRetry}
					class="mt-4 px-4 py-2 bg-white/10 rounded-lg text-sm text-gray-300 hover:bg-white/20 transition-colors"
				>
					Try again
				</button>
			</div>
		{/if}
	{:else}
		{#key errorKey}
			{@render children()}
		{/key}
	{/if}
</div>

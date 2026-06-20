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
</script>

<svelte:boundary onerror={onError}>
	{#snippet failed(error: unknown, reset: () => void)}
		{#if fallback}
			{@render fallback(error, reset)}
		{:else}
			<div class="flex flex-col items-center justify-center py-12 px-4 text-center">
				<div class="w-12 h-12 rounded-full bg-red-900/30 flex items-center justify-center mb-4">
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-6 h-6 text-red-400">
						<path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-2h2v2zm0-4h-2V7h2v6z"/>
					</svg>
				</div>
				<p class="text-gray-300 font-medium">Something went wrong</p>
				<p class="text-gray-500 text-sm mt-1 max-w-md">{String(error)}</p>
				<button
					onclick={reset}
					class="mt-4 px-4 py-2 bg-white/10 rounded-lg text-sm text-gray-300 hover:bg-white/20 transition-colors"
				>
					Try again
				</button>
			</div>
		{/if}
	{/snippet}
	{@render children()}
</svelte:boundary>

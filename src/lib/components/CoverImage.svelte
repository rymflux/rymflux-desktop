<script lang="ts">
	let { url, title = 'Cover', class: className = '' }: { url?: string | null; title?: string; class?: string } = $props();
	let failed = $state(false);
</script>

{#if url && !failed}
	<img
		src={url}
		alt={title}
		class={className}
		onerror={() => (failed = true)}
	/>
{:else}
	<div
		class="{className} bg-gray-700 flex items-center justify-center"
		aria-label={title}
	>
		<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-1/2 h-1/2 text-gray-500">
			<path d="M12 3v10.55c-.59-.34-1.27-.55-2-.55C7.79 13 6 14.79 6 17s1.79 4 4 4 4-1.79 4-4V7h4V3h-6z" />
		</svg>
	</div>
{/if}

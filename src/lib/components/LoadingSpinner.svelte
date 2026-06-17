<script lang="ts">
	let { size = 'md', class: className = '' }: { size?: 'sm' | 'md' | 'lg'; class?: string } = $props();
	let dims: Record<string, string> = { sm: 'w-4 h-4', md: 'w-6 h-6', lg: 'w-10 h-10' };
</script>

<div class="{className} flex items-center justify-center" role="status" aria-label="Loading">
	<svg
		class="{dims[size] ?? dims.md} animate-spin text-blue-500"
		xmlns="http://www.w3.org/2000/svg"
		fill="none"
		viewBox="0 0 24 24"
	>
		<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
		<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
	</svg>
</div>

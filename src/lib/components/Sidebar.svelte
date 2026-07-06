<script lang="ts">
	import { getUiState } from '../stores/uiStore.svelte';
	import { page } from '$app/stores';
	import { resolve } from '$app/paths';

	let ui = getUiState();
	let nav = $derived([
		{ href: '/', label: 'Home', icon: 'home' },
		{ href: '/search', label: 'Search', icon: 'search' },
		{ href: '/library', label: 'Library', icon: 'library' },
		{ href: '/settings', label: 'Settings', icon: 'settings' }
	] as const);
</script>

<aside
	class="flex flex-col transition-all duration-200 {ui.sidebarOpen
		? 'w-56'
		: 'w-0 overflow-hidden'}"
	style="background-color: var(--bg-primary); border-right-color: var(--border);"
	role="navigation"
	aria-label="Sidebar navigation"
>
	<div class="p-4 shrink-0">
		<a
			href={resolve('/')}
			class="text-lg font-bold tracking-tight"
			style="color: var(--text-primary);">rymflux</a
		>
	</div>

	<nav class="flex-1 px-2 space-y-1">
		{#each nav as item (item.href)}
			<a
			href={resolve(item.href)}
				class="block px-3 py-2 rounded-lg text-sm transition-colors
					{$page.url.pathname === item.href ? 'bg-blue-600 text-white' : 'hover:bg-white/5'}"
				style="color: {$page.url.pathname === item.href ? '' : 'var(--text-secondary)'};"
			>
				{item.label}
			</a>
		{/each}
	</nav>

	<div class="p-3 text-xs" style="color: var(--text-muted);">v0.1</div>
</aside>

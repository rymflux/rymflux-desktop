<script lang="ts">
	import { getUiState } from '$lib/stores/uiStore';
	import { page } from '$app/stores';

	let ui = getUiState();
	let nav = $derived([
		{ href: '/', label: 'Home', icon: 'home' },
		{ href: '/search', label: 'Search', icon: 'search' },
		{ href: '/library', label: 'Library', icon: 'library' },
		{ href: '/settings', label: 'Settings', icon: 'settings' },
	]);
</script>

<aside
	class="bg-gray-950 border-r border-white/10 flex flex-col transition-all duration-200 {ui.sidebarOpen ? 'w-56' : 'w-0 overflow-hidden'}"
	role="navigation"
	aria-label="Sidebar navigation"
>
	<div class="p-4 shrink-0">
		<a href="/" class="text-lg font-bold tracking-tight text-white">rymflux</a>
	</div>

	<nav class="flex-1 px-2 space-y-1">
		{#each nav as item}
			<a
				href={item.href}
				class="block px-3 py-2 rounded-lg text-sm transition-colors
					{$page.url.pathname === item.href ? 'bg-blue-600 text-white' : 'text-gray-400 hover:text-white hover:bg-white/5'}"
			>
				{item.label}
			</a>
		{/each}
	</nav>

	<div class="p-3 text-xs text-gray-600">
		v0.1
	</div>
</aside>

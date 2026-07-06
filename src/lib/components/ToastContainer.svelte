<script lang="ts">
	import Icon from './Icon.svelte';
	import { getToasts, removeToast } from '../stores/toastStore.svelte';
	import type { ToastType } from '../stores/toastStore.svelte';

	let items = $derived(getToasts());

	const iconColors: Record<ToastType, string> = {
		error: 'var(--text-danger)',
		warning: '#fbbf24',
		info: '#60a5fa'
	};
</script>

{#if items.length > 0}
	<div class="fixed top-4 right-4 z-[100] flex flex-col gap-2 max-w-sm w-full pointer-events-none">
		{#each items as item (item.id)}
			{@const c = iconColors[item.type] ?? iconColors.info}
			<div
				class="pointer-events-auto flex items-start gap-3 px-4 py-3 rounded-lg shadow-xl backdrop-blur-sm transition-all duration-300"
				style="background-color: color-mix(in srgb, var(--bg-secondary) 90%, transparent); border: 1px solid var(--border);"
				role="alert"
			>
				<div
					class="w-8 h-8 rounded-full flex items-center justify-center shrink-0 mt-0.5"
					style="background-color: color-mix(in srgb, {c} 20%, transparent);"
				>
					<Icon name={item.type} size={16} class="w-4 h-4" color={c} />
				</div>
				<p class="flex-1 text-sm leading-relaxed" style="color: var(--text-primary);">
					{item.message}
				</p>
				<button
					onclick={() => removeToast(item.id)}
					class="shrink-0 p-1 rounded-md transition-colors"
					style="color: var(--text-secondary);"
					aria-label="Dismiss"
				>
					<Icon name="x" size={16} class="w-4 h-4" />
				</button>
			</div>
		{/each}
	</div>
{/if}

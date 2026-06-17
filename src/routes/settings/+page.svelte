<script lang="ts">
	import { getUiState } from '$lib/stores/uiStore';

	let ui = getUiState();
</script>

<div class="max-w-2xl mx-auto">
	<h1 class="text-2xl font-bold mb-6">Settings</h1>

	<section class="space-y-4">
		<div class="flex items-center justify-between">
			<span class="text-sm">View mode</span>
			<select
				value={ui.viewMode}
				onchange={(e) => (ui.viewMode = (e.target as HTMLSelectElement).value as 'grid' | 'list')}
				class="bg-white/10 border border-white/20 rounded-lg px-3 py-1.5 text-sm"
			>
				<option value="grid">Grid</option>
				<option value="list">List</option>
			</select>
		</div>
	</section>

	<p class="text-xs text-gray-600 mt-8">rymflux v0.1</p>
</div>

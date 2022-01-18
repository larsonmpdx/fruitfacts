<script context="module" lang="ts">
	import Header from './Header.svelte';
	import { recentChangesData } from './store';
	import { format as timeAgo } from 'timeago.js';

	let fact: any = {};

	fetch(`${import.meta.env.VITE_BACKEND_BASE}/api/fact`)
		.then((response) => response.json())
		.then((data) => {
			fact = data;
		})
		.catch((error) => {
			console.log(error);
		});

	fetch(`${import.meta.env.VITE_BACKEND_BASE}/api/recent_changes`)
		.then((response) => response.json())
		.then((data) => {
			recentChangesData.set(data);
		})
		.catch((error) => {
			console.log(error);
		});
</script>

<Header />
<main>
	<div class="m-5">
		<a href="/dirs?path=">browse locations</a>
	</div>
	<div class="m-5">
		{#if fact.fact}
			<p>{fact.fact}<a href=" {fact.reference}">[ref]</a></p>
		{/if}
	</div>
	<div class="m-5">
		{#if $recentChangesData.recent_changes}
			<ul class="list-group d-inline-block">
				{#each $recentChangesData.recent_changes.collection_changes as update}
					<li class="list-group-item border border-2 rounded-lg py-1">
						<a href="/collections/{encodeURIComponent(`${update.path}${update.filename}`)}"
							>{update.filename}</a
						>
						{timeAgo(update.git_edit_time * 1000)}
					</li>
				{/each}
			</ul>
		{/if}
	</div>
	<div class="m-5">
		{#if $recentChangesData.recent_changes}
			<p>
				{$recentChangesData.recent_changes.base_plants_count} plants in {$recentChangesData
					.recent_changes.references_count} references
			</p>
		{/if}
		{#if $recentChangesData.build_info}
			<p>
				updated {timeAgo($recentChangesData.build_info.git_unix_time * 1000)} build count {$recentChangesData
					.build_info.git_commit_count} git hash {$recentChangesData.build_info.git_hash.substring(
					0,
					7
				)}
			</p>
		{/if}
	</div>
</main>

<style>
</style>

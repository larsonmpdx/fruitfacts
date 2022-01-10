<script lang="ts">
	import Header from './Header.svelte';
	import { recentChangesData } from './store';
	import { format as timeAgo } from 'timeago.js';

	fetch(`${import.meta.env.VITE_BACKEND_BASE}/recent_changes`)
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
	<a href="/dirs?path=">browse locations</a>
	{#if $recentChangesData.recent_changes}
		<p>
			{$recentChangesData.recent_changes.base_plants_count} plants in {$recentChangesData
				.recent_changes.references_count} references
		</p>
	{/if}
	{#if $recentChangesData.recent_changes}
		{#each $recentChangesData.recent_changes.collection_changes as update}
			<li>
				<a href="/collections?path={encodeURIComponent(`${update.path}${update.filename}`)}"
					>{update.filename}</a
				>
				{timeAgo(update.git_edit_time * 1000)}
			</li>
		{/each}
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
</main>

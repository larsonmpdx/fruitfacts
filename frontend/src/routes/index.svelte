<script lang="ts">
	import { recentChangesData } from './store';
	import { goto } from '$app/navigation';
	import AutoComplete from 'simple-svelte-autocomplete';
    import { format as timeAgo } from 'timeago.js';

	let selectedPlant;
	$: if (selectedPlant) {
		goto(`/plant?type=${selectedPlant.type}&name=${selectedPlant.name}`);
	}

	fetch(`http://localhost:8080/recent_changes`)
		.then((response) => response.json())
		.then((data) => {
			recentChangesData.set(data);
		})
		.catch((error) => {
			console.log(error);
			return [];
		});

	async function searchPlant(keyword) {
		const url = 'http://localhost:8080/search/variety/' + encodeURIComponent(keyword);

		const response = await fetch(url);
		return await response.json();
	}
</script>

<AutoComplete
	searchFunction={searchPlant}
	bind:selectedItem={selectedPlant}
	labelFieldName="name"
	localFiltering={false}
	maxItemsToShowInList="10"
	delay="200"
	minCharactersToSearch="3"
/>
<a href="/dirs?path=">browse locations</a>
{#if $recentChangesData.build_info}
    <p>updated {timeAgo($recentChangesData.build_info.git_unix_time * 1000)}</p>
	<p>build count {$recentChangesData.build_info.git_commit_count}</p>
	<p>hash {$recentChangesData.build_info.git_hash}</p>
{/if}
{#if $recentChangesData.recent_updates}
    {#each $recentChangesData.recent_updates as update}
        <li>{update.filename} {timeAgo(update.git_edit_time * 1000)}</li>
    {/each}
{/if}
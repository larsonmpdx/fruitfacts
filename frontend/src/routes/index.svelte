<script lang="ts">
	import { recentChangesData, login } from './store';
	import { goto } from '$app/navigation';
	import AutoComplete from 'simple-svelte-autocomplete';
	import { format as timeAgo } from 'timeago.js';
	import { browser } from '$app/env';

	let selectedPlant;
	let logged_in = false;
	$: if (selectedPlant) {
		goto(`/plant?type=${selectedPlant.type}&name=${selectedPlant.name}`);
	}

	fetch(`http://${import.meta.env.VITE_WEB_ADDRESS}:8080/recent_changes`)
		.then((response) => {
			if (response.status === 200) {
				recentChangesData.set(response.json());
			}
		})
		.catch((error) => {
			console.log(error);
		});

	if (browser) {
		fetch(`http://${import.meta.env.VITE_WEB_ADDRESS}:8080/checkLogin`, {
			credentials: 'include'
		})
			.then((response) => {
				if (response.status === 200) {
					login.set(response.json());
					logged_in = true;
				} else {
					logged_in = false;
				}
			})
			.then((data) => {
				login.set(data);
			})
			.catch((error) => {
				console.log(error);
			});
	}

	async function searchPlant(keyword) {
		const url = `http://${import.meta.env.VITE_WEB_ADDRESS}:8080/search/${encodeURIComponent(
			keyword
		)}`;

		const response = await fetch(url);
		return await response.json();
	}
</script>

{#if logged_in}
	logged in as {$login.email}
{:else}
	<a href="/login">log in</a>
{/if}
<AutoComplete
	searchFunction={searchPlant}
	bind:selectedItem={selectedPlant}
	labelFunction={(plant) => {
		if (plant.marketing_name) {
			return plant.name + ' (' + plant.marketing_name + ') ' + plant.type;
		} else {
			return plant.name + ' ' + plant.type;
		}
	}}
	localFiltering={false}
	maxItemsToShowInList="10"
	delay="200"
	minCharactersToSearch="3"
/>
<a href="/dirs?path=">browse locations</a>
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

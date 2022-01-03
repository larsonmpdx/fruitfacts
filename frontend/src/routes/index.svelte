<script lang="ts">
	import { recentChangesData, login, authURL } from './store';
	import { goto } from '$app/navigation';
	import AutoComplete from 'simple-svelte-autocomplete';
	import { format as timeAgo } from 'timeago.js';
	import { browser } from '$app/env';

	let selectedPlant;
	$: if (selectedPlant) {
		goto(`/plant?type=${selectedPlant.type}&name=${selectedPlant.name}`);
	}

	fetch(`http://fruitfacts.xyz:8080/recent_changes`)
		.then((response) => {
			if (response.status === 200) {
				recentChangesData.set(response.json());
			}
		})
		.catch((error) => {
			console.log(error);
		});

		if (browser) {
	fetch(`http://fruitfacts.xyz:8080/checkLogin`)
		.then((response) => {
			if (response.status === 200) {
				console.log("eh1");
				login.set(response.json());
			} else {
				fetch(`http://fruitfacts.xyz:8080/authURLs`)
					.then((response) => {
						if (response.status === 200) {
							console.log("eh2");
							authURL.set(response.json());
						}
					})
					.catch((error) => {
						console.log(error);
					});
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
		const url = 'http://fruitfacts.xyz:8080/search/' + encodeURIComponent(keyword);

		const response = await fetch(url);
		return await response.json();
	}
</script>

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

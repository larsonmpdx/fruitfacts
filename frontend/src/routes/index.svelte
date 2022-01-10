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

	fetch(`${import.meta.env.VITE_BACKEND_BASE}/recent_changes`)
		.then((response) => response.json())
		.then((data) => {
			recentChangesData.set(data);
		})
		.catch((error) => {
			console.log(error);
		});

	if (browser) {
		fetch(`${import.meta.env.VITE_BACKEND_BASE}/checkLogin`, {
			credentials: 'include'
		})
			.then((response) => {
				if (response.status === 200) {
					logged_in = true;
				} else {
					logged_in = false;
				}
				return response.json();
			})
			.then((data) => {
				login.set(data);
			})
			.catch((error) => {
				console.log(error);
			});
	}

	async function searchPlant(keyword) {
		const url = `${import.meta.env.VITE_BACKEND_BASE}/search/${encodeURIComponent(keyword)}`;

		const response = await fetch(url);
		return await response.json();
	}

	async function logOut() {
		fetch(`${import.meta.env.VITE_BACKEND_BASE}/logout`, {
			method: 'POST',
			credentials: 'include'
		})
			.then((response) => {
				if (response.status === 200) {
					logged_in = false;
					login.set({});
				}
				return response.json();
			})
			.catch((error) => {
				console.log(error);
			});
	}
</script>

{#if $login.user}
	logged in as {$login.user.name}
	<button type="button" on:click={logOut}>log out</button>
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

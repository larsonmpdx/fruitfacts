<script lang="ts">
	import { login } from './store';
	import { browser } from '$app/env';
	import { goto } from '$app/navigation';
	import AutoComplete from 'simple-svelte-autocomplete';

	if (browser) {
		fetch(`${import.meta.env.VITE_BACKEND_BASE}/api/checkLogin`, {
			credentials: 'include'
		})
			.then((response) => response.json())
			.then((data) => {
				login.set(data);
			})
			.catch((error) => {
				console.log(error);
			});
	}

	async function logOut() {
		fetch(`${import.meta.env.VITE_BACKEND_BASE}/api/logout`, {
			method: 'POST',
			credentials: 'include'
		})
			.then((response) => {
				if (response.status === 200) {
					login.set({});
				}
				return response.json();
			})
			.catch((error) => {
				console.log(error);
			});
	}

	let selectedPlant;
	$: if (selectedPlant) {
		goto(`/plant?type=${selectedPlant.type}&name=${selectedPlant.name}`);
	}

	async function searchPlant(keyword) {
		const url = `${import.meta.env.VITE_BACKEND_BASE}/api/search/${encodeURIComponent(keyword)}`;

		const response = await fetch(url);
		return await response.json();
	}
</script>

<div class="mt-2 mx-5">
	<a href="/">Fruitfacts</a>
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
	{#if $login.user}
		logged in as <a href="/user/">{$login.user.name}</a>
		<button type="button" on:click={logOut}>log out</button>
	{:else}
		<a href="/login">log in</a>
	{/if}
</div>

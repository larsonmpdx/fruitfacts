<script lang="ts">
	import { login } from './store';
	import { browser } from '$app/env';
	import { goto } from '$app/navigation';
	import AutoComplete from 'simple-svelte-autocomplete';

	if (browser) {
		fetch(`${import.meta.env.VITE_BACKEND_BASE}/checkLogin`, {
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
		fetch(`${import.meta.env.VITE_BACKEND_BASE}/logout`, {
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
		const url = `${import.meta.env.VITE_BACKEND_BASE}/search/${encodeURIComponent(keyword)}`;

		const response = await fetch(url);
		return await response.json();
	}
</script>
<head>
	blue<link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png">
	<link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png">
	<link rel="icon" type="image/png" sizes="16x16" href="/favicon-16x16.png">
	<link rel="manifest" href="/site.webmanifest">
	<link rel="mask-icon" href="/safari-pinned-tab.svg" color="#5bbad5">
	<meta name="msapplication-TileColor" content="#da532c">
	<meta name="theme-color" content="#ffffff">
</head>
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
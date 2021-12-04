<script lang="ts">
	//   import { onMount } from "svelte";
	import { apiData, locations, items } from './store';
	import { page } from '$app/stores';
	import { onMount, beforeUpdate } from 'svelte';

	let previousPath;

	const ifPathChanged = async (path) => {
		// needs to be in onMount because the query string isn't available in pre rendering
		if (path != previousPath) {
			previousPath = path;
			fetch(`http://localhost:8080/collections/${path}`)
				.then((response) => response.json())
				.then((data) => {
					apiData.set(data);
				})
				.catch((error) => {
					console.log(error);
					return [];
				});
		}
	};

	onMount(async () => {
		// needs to be in onMount because the query string isn't available in pre rendering
		const path = $page.query.get('path');
		ifPathChanged(path);
	});

	beforeUpdate(async () => {
		// this gets back button changes
		const path = $page.query.get('path');
		ifPathChanged(path);
	});
</script>

<main>
	<!--- todo header info --->
	<h1>Locations</h1>
	<ul>
		{#each $locations as location}
			<li>{location}</li>
		{/each}
	</ul>
	<h1>Plants</h1>
	<ul>
		{#each $items as item}
			<li>
				<a href="/plant?type={encodeURIComponent(item.type)}&name={encodeURIComponent(item.name)}"
					>{#if item.marketing_name}{item.name} {item.type} (marketed as {item.marketing_name}){:else}{item.name} {item.type}{/if}</a
				>
			</li>
		{/each}
	</ul>
</main>

<style>
</style>

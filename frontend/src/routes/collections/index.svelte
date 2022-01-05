<script lang="ts">
	//   import { onMount } from "svelte";
	import { apiData, collection, locations, items } from './store';
	import { page } from '$app/stores';
	import { onMount, beforeUpdate } from 'svelte';

	let previousPath;

	const ifPathChanged = async (path) => {
		// needs to be in onMount because the query string isn't available in pre rendering
		if (path != previousPath) {
			previousPath = path;
			fetch(`http://${import.meta.env.VITE_WEB_ADDRESS}:8080/collections/${path}`)
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
	<p>
		{$collection.title}
		{#if $collection.url}<a href={$collection.url}>[ref]</a>{/if}
	</p>
	<h1>Locations</h1>
	<ul>
		{#each $locations as location}
			<li>{location.location_name}</li>
		{/each}
	</ul>
	<h1>Plants</h1>
	<ul>
		{#each $items as item}
			<li>
				<a href="/plant?type={encodeURIComponent(item.type)}&name={encodeURIComponent(item.name)}"
					>{item.name} {item.type}</a
				>
				{#if item.marketing_name}(marketed as {item.marketing_name}){/if}
			</li>
		{/each}
	</ul>
</main>

<style>
</style>

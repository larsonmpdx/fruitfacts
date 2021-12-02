<script lang="ts">
	//   import { onMount } from "svelte";
	import { apiData, base, collection_entries } from './store';
	import { page } from '$app/stores';
	import { onMount, beforeUpdate } from 'svelte';

	let previousPath;

	const ifPathChanged = async (type_, name) => {
		let path = `${type_}/${name}`;

		// needs to be in onMount because the query string isn't available in pre rendering
		if (path != previousPath) {
			previousPath = path;
			fetch(`http://localhost:8080/plants/${path}`)
				.then((response) => response.json())
				.then((data) => {
					apiData.set(data);
				})
				.catch((error) => {
					console.log(error);
					return [];
				});
		} else {
			console.log('path unchanged');
		}
	};

	onMount(async () => {
		// needs to be in onMount because the query string isn't available in pre rendering
		const type_ = $page.query.get('type');
		const name = $page.query.get('name');
		console.log(`path: ${type_}/${name}`);

		ifPathChanged(type_, name);
	});

	beforeUpdate(async () => {
		// this gets back button changes
		const type_ = $page.query.get('type');
		const name = $page.query.get('name');
		console.log(`path from afterUpdate: ${type_}/${name}`);

		ifPathChanged(type_, name);
	});
</script>

<main>
	<h1>Base</h1>
	<!--- todo header info --->
	{base}
	<h1>Collection Entries</h1>
	<ul>
		{#each $collection_entries as entry}
			<li>{entry.description}</li>
		{/each}
	</ul>
</main>

<style>
</style>

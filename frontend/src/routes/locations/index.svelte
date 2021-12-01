<script lang="ts">
	//   import { onMount } from "svelte";
	import { apiData, directories, collections } from './store';
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
					//    console.log(data);
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
		console.log(`onmount`);
		const path = $page.query.get('path');
		console.log(`path ${path}`);

		ifPathChanged(path);
	});

	beforeUpdate(async () => {
		// this gets back button changes
		const path = $page.query.get('path');
		console.log(`path from afterUpdate: ${path}`);

		ifPathChanged(path);
	});

	// unused but I might need this later:
	//     import { goto } from '$app/navigation';
	// const handleClick = path => () => {
	//	let query = new URLSearchParams($page.query.toString());
	//    query.set('path', path);
	//     goto(`?${query.toString()}`);
	//}

	// used with:
	//  <button on:click={handleClick(directory)}>
	//     count: 1
	// </button>
</script>

<main>
	<h1>dirs</h1>
	<li>
		{#each $directories as directory}
			<li><a href="/locations?path={encodeURIComponent(directory)}">{directory}</a></li>
		{/each}
	</li>
	<h1>locations</h1>
	<li>
		{#each $collections as collection}
			<li>{collection.title}</li>
		{/each}
	</li>
</main>

<style>
</style>

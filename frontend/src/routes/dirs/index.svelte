<script lang="ts">
	import { apiData, directories, collections } from './store';
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
					//    console.log(data);
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
	{#if $directories && $directories.length > 0}
		<h1>dirs</h1>
		<ul>
			{#each $directories as directory}
				<li><a href="/dirs?path={encodeURIComponent(directory)}">{directory}</a></li>
			{/each}
		</ul>
	{/if}
	{#if $collections && $collections.length > 0}
		<h1>locations</h1>
		<ul>
			{#each $collections as collection}
				<li>
					<a
						href="/collections?path={encodeURIComponent(
							`${collection.path}${collection.filename}`
						)}">{collection.title}</a
					>
				</li>
			{/each}
		</ul>
	{/if}
</main>

<style>
</style>

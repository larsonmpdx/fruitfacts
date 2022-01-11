<script lang="ts">
	import Header from '../Header.svelte';
	import { apiData, collection, locations, items } from './store';
	import { page } from '$app/stores';
	import { onMount, beforeUpdate } from 'svelte';

	let previousPath;

	const ifPathChanged = async (path) => {
		// needs to be in onMount because the query string isn't available in pre rendering
		if (path != previousPath) {
			previousPath = path;
			fetch(`${import.meta.env.VITE_BACKEND_BASE}/collections/${path}`)
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

<Header />
<main>
	<div class="m-5">
		<p>
			{$collection.title}
			{#if $collection.url}<a href={$collection.url}>[ref]</a>{/if}
		</p>
		<h1>Locations</h1>
		<ul class="list-group d-inline-block">
			{#each $locations as location}
				<li class="list-group-item border border-2 rounded-lg py-1">{location.location_name}</li>
			{/each}
		</ul>
		<h1>Plants</h1>
		<ul class="list-group d-inline-block">
			{#each $items as item}
				<li class="list-group-item border border-2 rounded-lg py-1">
					<a href="/plant?type={encodeURIComponent(item.type)}&name={encodeURIComponent(item.name)}"
						>{item.name} {item.type}</a
					>
					{#if item.marketing_name}(marketed as {item.marketing_name}){/if}
				</li>
			{/each}
		</ul>
	</div>
</main>

<style>
</style>

<script lang="ts">
	import Header from '../Header.svelte';
	import { apiData, collection, locations, items } from './store';
	import { page } from '$app/stores';

	$: fetch(`${import.meta.env.VITE_BACKEND_BASE}/api/collections/${$page.params.path}`)
		.then((response) => response.json())
		.then((data) => {
			apiData.set(data);
		})
		.catch((error) => {
			console.log(error);
			return [];
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
					<a href="/plant/{encodeURIComponent(item.type)}/{encodeURIComponent(item.name)}"
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

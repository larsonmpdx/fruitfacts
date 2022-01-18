<script lang="ts">
	import Header from '../Header.svelte';
	import { apiData, directories, collections } from './store';
	import { page } from '$app/stores';

	$: fetch(`${import.meta.env.VITE_BACKEND_BASE}/api/collections/${$page.params.path}`)
		.then((response) => response.json())
		.then((data) => {
			//    console.log(data);
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
		{#if $directories && $directories.length > 0}
			<ul class="list-group d-inline-block">
				{#each $directories as directory}
					<li class="list-group-item border border-2 rounded-lg py-1">
						<a href="/dirs/{encodeURIComponent(directory)}">{directory}</a>
					</li>
				{/each}
			</ul>
		{/if}
		{#if $collections && $collections.length > 0}
			<h1>Locations</h1>
			<ul class="list-group d-inline-block">
				{#each $collections as collection}
					<li class="list-group-item border border-2 rounded-lg py-1">
						<a
							href="/collections/{encodeURIComponent(
								`${collection.path}${collection.filename}`
							)}">{collection.title}</a
						>
					</li>
				{/each}
			</ul>
		{/if}
	</div>
</main>

<style>
</style>

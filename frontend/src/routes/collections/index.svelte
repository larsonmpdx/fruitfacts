<script lang="ts">
	import Header from '../Header.svelte';
	import { apiData, collection, locations, items } from './store';
	import { onMount, beforeUpdate } from 'svelte';
	import { browser } from '$app/env';

	import { page } from '$app/stores';
	let pageLog: string = $page.path;
	$: if (pageLog !== $page.path) {
		pageLog = $page.path;
		// see https://github.com/sveltejs/kit/issues/560
		// todo - use this when it's released https://github.com/sveltejs/kit/pull/3293
		check();
	}

	const check = async () => {
		const query = new URLSearchParams(document.location.search);
		const path = query.get('path'); // we can't use $page.query.get() because of https://github.com/sveltejs/kit/issues/669
		ifPathChanged(path);
	};

	let previousPath;
	const ifPathChanged = async (path) => {
		// todo - use this when it's released https://github.com/sveltejs/kit/pull/3293
		// needs to be in onMount because the query string isn't available in pre rendering
		console.log(`previous: ${previousPath} current ${path}`);
		if (path != previousPath) {
			previousPath = path;
			fetch(`${import.meta.env.VITE_BACKEND_BASE}/api/collections/${path}`)
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
	if (browser) {
		onMount(async () => {
			// needs to be in onMount because the query string isn't available in pre rendering
			check();
		});

		beforeUpdate(async () => {
			// this gets back button changes
			check();
		});
	}
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

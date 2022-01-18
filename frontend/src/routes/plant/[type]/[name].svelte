<script lang="ts">
	import Header from '../../Header.svelte';
	import { beforeUpdate, onMount } from 'svelte';
	import { format as timeAgo } from 'timeago.js';
	import { page } from '$app/stores';

	//const fetched = (async () => {
	//	const response = await fetch(`${import.meta.env.VITE_BACKEND_BASE}/api/plants/${$page.params.type}/${$page.params.name}`)
   // 	return await response.json()
	//})()

	let promise = Promise.resolve({});
	async function fetchPlant() {
		const response = await self.fetch(`${import.meta.env.VITE_BACKEND_BASE}/api/plants/${$page.params.type}/${$page.params.name}`);

		if (response.ok) {
  			return response.json();	
		} else {
			throw new Error();
		}
	}

	promise = fetchPlant();
	// let fetched;
	/*
	async function reload() {
		console.log("hi1");
		previousPath = $page.url.pathname;
	fetch(`${import.meta.env.VITE_BACKEND_BASE}/api/plants/${$page.params.type}/${$page.params.name}`)
		.then((response) => response.json())
		.then((data) => {
			console.log(`fetched ${$page.params.type} ${$page.params.name} ${JSON.stringify(data)}`)
			fetched = data;
		})
		.catch((error) => {
			console.log(error);
			fetched = undefined;
		});
	}

	let previousPath;
	$: if ($page.url.pathname != previousPath) {
		console.log("hi2");
		reload();
	}
	console.log("hi3");

	onMount (async () => {
		console.log("hi4");
		reload();
	});

	beforeUpdate(async () => {
			// this gets back button changes
			console.log("hi5");
			reload();
		});

		reload();
*/
</script>

<Header />
<main>

	{#await promise}
	<p>...waiting</p>
{:then fetched}

	{#if fetched.base}
	<p>
	{fetched.base.name}
	{fetched.base.type}
	{#if fetched.base.marketing_name}(marketed as {fetched.base.marketing_name}){/if}
</p>
<p>
	{#if fetched.base.uspp_number}USPP{fetched.base.uspp_number}{/if}
	{#if fetched.base.uspp_expiration}{#if fetched.base.uspp_expiration * 1000 > new Date().getTime()}expires
		{:else}expired
		{/if}{timeAgo(fetched.base.uspp_expiration * 1000)}{/if}
</p>
<p>
	{#if fetched.base.aka}AKA {fetched.base.aka}{/if}
</p>
{#if fetched.base.release_year || fetched.base.released_by}
	<!--- todo link to release collection --->
	<p>
		{#if fetched.base.release_year}{fetched.base.release_year}{/if}
		{#if fetched.base.released_by}{fetched.base.released_by}{/if}
	</p>
{/if}
{/if}
{#if fetched.collection_entries}
<h1>Collection Entries</h1>
<ul class="list-group d-inline-block">
	{#each fetched.collection_entries as entry}
		<li class="list-group-item border border-2 rounded-lg py-1">
			<a href="/collections/{encodeURIComponent(`${entry.path_and_filename}`)}"
				>{entry.path_and_filename}</a
			>
			{#if entry.description}{entry.description}{/if}
		</li>
	{/each}
</ul>
<h1>Harvest Times</h1>
<ul>
	{#each fetched.collection_entries as entry}
		{#if entry.harvest_text}<li>
				{entry.harvest_text}
				<a
					href="/collections/{encodeURIComponent(`${entry.path_and_filename}`)}"
					title={entry.path_and_filename}>[ref]</a
				>
			</li>{/if}
	{/each}
</ul>
{/if}
{:catch error}
	<p style="color: red">{error.message}</p>
{/await}









</main>

<style>
</style>

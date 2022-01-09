<script lang="ts">
	import { apiData, base, collection_entries } from './store';
	import { page } from '$app/stores';
	import { onMount, beforeUpdate } from 'svelte';
	import { format as timeAgo } from 'timeago.js';

	let previousPath;

	const ifPathChanged = async (type_, name) => {
		let path = `${type_}/${name}`;

		// needs to be in onMount because the query string isn't available in pre rendering
		if (path != previousPath) {
			previousPath = path;
			fetch(`${import.meta.env.VITE_BACKEND_BASE}/plants/${path}`)
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
		const type_ = $page.query.get('type');
		const name = $page.query.get('name');
		ifPathChanged(type_, name);
	});

	beforeUpdate(async () => {
		// this gets back button changes
		const type_ = $page.query.get('type');
		const name = $page.query.get('name');
		ifPathChanged(type_, name);
	});
</script>

<main>
	<p>
		{$base.name}
		{$base.type}
		{#if $base.marketing_name}(marketed as {$base.marketing_name}){/if}
	</p>
	<p>
		{#if $base.uspp_number}USPP{$base.uspp_number}{/if}
		{#if $base.uspp_expiration}{#if $base.uspp_expiration * 1000 > new Date().getTime()}expires
			{:else}expired
			{/if}{timeAgo($base.uspp_expiration * 1000)}{/if}
	</p>
	<p>
		{#if $base.aka}AKA {$base.aka}{/if}
	</p>
	{#if $base.release_year || $base.released_by}
		<!--- todo link to release collection --->
		<p>
			{#if $base.release_year}{$base.release_year}{/if}
			{#if $base.released_by}{$base.released_by}{/if}
		</p>
	{/if}
	<h1>Collection Entries</h1>
	<ul>
		{#each $collection_entries as entry}
			<li>
				<a href="/collections?path={encodeURIComponent(`${entry.path_and_filename}`)}"
					>{entry.path_and_filename}</a
				>
				{#if entry.description}{entry.description}{/if}
			</li>
		{/each}
	</ul>
	<h1>Harvest Times</h1>
	<ul>
		{#each $collection_entries as entry}
			{#if entry.harvest_text}<li>
					{entry.harvest_text}
					<a
						href="/collections?path={encodeURIComponent(`${entry.path_and_filename}`)}"
						title={entry.path_and_filename}>[ref]</a
					>
				</li>{/if}
		{/each}
	</ul>
</main>

<style>
</style>

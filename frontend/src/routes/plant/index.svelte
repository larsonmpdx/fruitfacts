<script lang="ts">
	import Header from '../Header.svelte';
	import { apiData, base, collection_entries } from './store';
	import { onMount, beforeUpdate } from 'svelte';
	import { format as timeAgo } from 'timeago.js';
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
		const name = query.get('name'); // we can't use $page.query.get() because of https://github.com/sveltejs/kit/issues/669
		const type_ = query.get('type');
		ifPathChanged(type_, name);
	};

	let previousPath;
	const ifPathChanged = async (type_, name) => {
		// todo - use this when it's released https://github.com/sveltejs/kit/pull/3293
		let path = `${type_}/${name}`;
		console.log(`previous: ${previousPath} current ${path}`);
		// needs to be in onMount because the query string isn't available in pre rendering
		if (path != previousPath) {
			previousPath = path;
			fetch(`${import.meta.env.VITE_BACKEND_BASE}/api/plants/${path}`)
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
	<ul class="list-group d-inline-block">
		{#each $collection_entries as entry}
			<li class="list-group-item border border-2 rounded-lg py-1">
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

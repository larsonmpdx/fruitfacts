<script lang="ts">
	import { authURLs } from './store';

	fetch(`${import.meta.env.VITE_BACKEND_BASE}/authURLs`, {
		credentials: 'include'
	})
		.then((response) => response.json())
		.then((data) => {
			console.log(data);
			authURLs.set(data);
		})
		.catch((error) => {
			console.log(error);
		});
</script>

<main>
	<p>
		{#if $authURLs}
			<a href={$authURLs.google}>log in with google</a>
		{/if}
	</p>
</main>

<style>
</style>

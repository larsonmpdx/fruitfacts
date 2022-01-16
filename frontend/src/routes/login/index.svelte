<script lang="ts">
	import { authURLs } from './store';

	fetch(`${import.meta.env.VITE_BACKEND_BASE}/api/authURLs`, {
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
	<div class="d-flex flex-column min-vh-100 justify-content-center align-items-center">
		{#if $authURLs}
			<a href={$authURLs.google}>log in with google oauth</a>
		{/if}
	</div>
</main>

<style>
</style>

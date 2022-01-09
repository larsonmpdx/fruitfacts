<script lang="ts">
	import { authURLs } from './store';

	fetch(`http://${import.meta.env.VITE_WEB_ADDRESS}:8080/authURLs`, {
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

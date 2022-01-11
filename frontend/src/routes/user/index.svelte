<script lang="ts">
	import Header from '../Header.svelte';
	import { login } from '../store';
</script>

<Header />
<main />

{#if $login}
	<div class="m-5">
		<p>stored user info:</p>
		<code><pre>{JSON.stringify($login, null, 2)}</pre></code>
		<!-- todo, another API call to show all oauth entries and sessions for this user -->
	</div>
{/if}

<style>
</style>

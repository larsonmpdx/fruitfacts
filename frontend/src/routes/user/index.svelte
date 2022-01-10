<script lang="ts">
	import Header from '../Header.svelte';
	import { login } from '../store';
</script>

<Header />
<main />

{#if $login}
	stored user info: {JSON.stringify($login)}
    <!-- todo, another API call to show all oauth entries and sessions for this user -->
{/if}

<style>
</style>

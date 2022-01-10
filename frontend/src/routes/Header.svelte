<script lang="ts">
	import { login } from './store';
	import { browser } from '$app/env';

	if (browser) {
		fetch(`${import.meta.env.VITE_BACKEND_BASE}/checkLogin`, {
			credentials: 'include'
		})
			.then((response) => response.json())
			.then((data) => {
				login.set(data);
			})
			.catch((error) => {
				console.log(error);
			});
	}

	async function logOut() {
		fetch(`${import.meta.env.VITE_BACKEND_BASE}/logout`, {
			method: 'POST',
			credentials: 'include'
		})
			.then((response) => {
				if (response.status === 200) {
					login.set({});
				}
				return response.json();
			})
			.catch((error) => {
				console.log(error);
			});
	}
</script>

{#if $login.user}
	logged in as <a href="/user/">{$login.user.name}</a>
	<button type="button" on:click={logOut}>log out</button>
{:else}
	<a href="/login">log in</a>
{/if}

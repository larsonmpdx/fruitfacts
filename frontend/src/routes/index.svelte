<script lang="ts">
	import { recentChangesData } from './store';
	import { goto } from '$app/navigation';
	import AutoComplete from 'simple-svelte-autocomplete';
	import { format as timeAgo } from 'timeago.js';
	import { browser } from '$app/env';






	import {
  OidcContext,
  LoginButton,
  LogoutButton,
  RefreshTokenButton,
  authError,
  accessToken,
  idToken,
  isAuthenticated,
  isLoading,
  login,
  logout,
  userInfo,
} from '@dopry/svelte-oidc';

const metadata = {
            // added to overcome missing value in auth0 .well-known/openid-configuration
            // see: https://github.com/IdentityModel/oidc-client-js/issues/1067
            // see: https://github.com/IdentityModel/oidc-client-js/pull/1068
            end_session_endpoint: `process.env.OIDC_ISSUER/v2/logout?client_id=process.env.OIDC_CLIENT_ID`,
        };






	let selectedPlant;
	$: if (selectedPlant) {
		goto(`/plant?type=${selectedPlant.type}&name=${selectedPlant.name}`);
	}

	fetch(`http://localhost:8080/recent_changes`)
		.then((response) => response.json())
		.then((data) => {
			recentChangesData.set(data);
		})
		.catch((error) => {
			console.log(error);
			return [];
		});

	async function searchPlant(keyword) {
		const url = 'http://localhost:8080/search/' + encodeURIComponent(keyword);

		const response = await fetch(url);
		return await response.json();
	}
</script>




{#if browser}
<OidcContext
 issuer="https://accounts.google.com"
 client_id="785945969813-ksp512anaa5qcage1qbgrkovdjloeltj.apps.googleusercontent.com"
 redirect_uri="http://localhost:8080/auth"
 post_logout_redirect_uri="http://localhost:3000"
 metadata={metadata}
 extraOptions={{
   mergeClaims: true,
   resource: "some_identifier",
 }}
 >

 <LoginButton>Login</LoginButton>
 <LogoutButton>Logout</LogoutButton>
 <br />
 <pre>isLoading: {$isLoading}</pre>
 <pre>isAuthenticated: {$isAuthenticated}</pre>
 <pre>authToken: {$accessToken}</pre>
 <pre>idToken: {$idToken}</pre>
 <pre>userInfo: {JSON.stringify($userInfo, null, 2)}</pre>
 <pre>authError: {$authError}</pre>
</OidcContext>
{/if}










<AutoComplete
	searchFunction={searchPlant}
	bind:selectedItem={selectedPlant}
	labelFunction={(plant) => {
		if (plant.marketing_name) {
			return plant.name + ' (' + plant.marketing_name + ') ' + plant.type;
		} else {
			return plant.name + ' ' + plant.type;
		}
	}}
	localFiltering={false}
	maxItemsToShowInList="10"
	delay="200"
	minCharactersToSearch="3"
/>
<a href="/dirs?path=">browse locations</a>
{#if $recentChangesData.recent_changes}
	<p>
		{$recentChangesData.recent_changes.base_plants_count} plants in {$recentChangesData
			.recent_changes.references_count} references
	</p>
{/if}
{#if $recentChangesData.build_info}
	<p>
		updated {timeAgo($recentChangesData.build_info.git_unix_time * 1000)} build count {$recentChangesData
			.build_info.git_commit_count} git hash {$recentChangesData.build_info.git_hash.substring(
			0,
			7
		)}
	</p>
{/if}
{#if $recentChangesData.recent_changes}
	{#each $recentChangesData.recent_changes.collection_changes as update}
		<li>
			<a href="/collections?path={encodeURIComponent(`${update.path}${update.filename}`)}"
				>{update.filename}</a
			>
			{timeAgo(update.git_edit_time * 1000)}
		</li>
	{/each}
{/if}

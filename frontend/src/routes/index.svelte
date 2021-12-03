<script lang="ts">
    import { apiData } from './store';
    import { goto } from '$app/navigation';
    import AutoComplete from "simple-svelte-autocomplete";

    let selectedPlant;
    $: if(selectedPlant) {
        goto(`/plant?type=${selectedPlant.type}&name=${selectedPlant.name}`) 
    }

    fetch(`http://localhost:8080/build_info`)
				.then((response) => response.json())
				.then((data) => {
					apiData.set(data);
				})
				.catch((error) => {
					console.log(error);
					return [];
				});

async function searchPlant(keyword) {
    const url = "http://localhost:8080/search/variety/"
    + encodeURIComponent(keyword);

    const response = await fetch(url);
    return await response.json();
}
</script>
<AutoComplete
    searchFunction={searchPlant}
    bind:selectedItem={selectedPlant}
    labelFieldName="name"
    localFiltering={false}
    maxItemsToShowInList="10"
    delay=200
    minCharactersToSearch=3
     />
<a href="/dirs?path=">browse locations</a>
{#if $apiData.git_hash}
<p>build count {$apiData.git_commit_count}</p>
<p>hash {$apiData.git_hash}</p>
<p>time {$apiData.git_unix_time}</p>
{/if}
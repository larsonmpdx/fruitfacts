<script lang="ts">
    import { goto } from '$app/navigation';
    import AutoComplete from "simple-svelte-autocomplete";

    let selectedPlant;
    $: if(selectedPlant) {
        goto(`/plant?type=${selectedPlant.type}&name=${selectedPlant.name}`) 
    }

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
<a href="/dirs?path=">browse</a>
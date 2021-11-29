<script lang="ts">
    import { onMount } from "svelte";
    import { apiData, directories } from './store';
    
    onMount(async () => {
      fetch("http://localhost:8080/collections/")
      .then(response => response.json())
      .then(data => {
            console.log(data);
        apiData.set(data);
      }).catch(error => {
        console.log(error);
        return [];
      });
    });
</script>
    
<main>
    <h1>dirs</h1>
    <ul>
    {#each $directories as directory}
        <li>{directory}</li>
    {/each}
    </ul>
</main>

<style>

</style>
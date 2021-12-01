import { writable, derived } from 'svelte/store';

/** Store for your data. 
This assumes the data you're pulling back will be an array
If it's going to be an object, default this to an empty object
**/
export const apiData = writable({});

export const directories = derived(apiData, ($apiData: any) => {
	if ($apiData.directories) {
		return $apiData.directories;
	}
	return [];
});

export const collections = derived(apiData, ($apiData: any) => {
	if ($apiData.collections) {
		return $apiData.collections;
	}
	return [];
});

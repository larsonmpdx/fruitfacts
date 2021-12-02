import { writable, derived } from 'svelte/store';

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

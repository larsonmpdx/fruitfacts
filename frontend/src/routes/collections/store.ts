import { writable, derived } from 'svelte/store';

export const apiData = writable({});

export const collection: any = derived(apiData, ($apiData: any) => {
	if ($apiData.collection) {
		return $apiData.collection;
	}
	return {};
});

export const locations = derived(apiData, ($apiData: any) => {
	if ($apiData.locations) {
		return $apiData.locations;
	}
	return [];
});

export const items = derived(apiData, ($apiData: any) => {
	if ($apiData.items) {
		return $apiData.items;
	}
	return [];
});

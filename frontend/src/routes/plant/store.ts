import { writable, derived } from 'svelte/store';

export const apiData = writable({});

export const base = derived(apiData, ($apiData: any) => {
	if ($apiData.base) {
		return $apiData.base;
	}
	return {};
});

export const collection_entries = derived(apiData, ($apiData: any) => {
	if ($apiData.collection) {
		return $apiData.collection;
	}
	return [];
});

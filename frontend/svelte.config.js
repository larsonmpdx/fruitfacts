import adapter from '@sveltejs/adapter-static'; // was "adapter-auto"
import preprocess from 'svelte-preprocess';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://github.com/sveltejs/svelte-preprocess
	// for more information about preprocessors
	preprocess: preprocess(),

	kit: {
		adapter: adapter({
			// default options are shown
			pages: 'build',
			assets: 'build',
			fallback: null
		}),

		// hydrate the <div id="svelte"> element in src/app.html
		target: '#svelte',
		vite: {
			envDir: '../' // look up a dir so we can share one .env file with the backend
		}
	}
};

export default config;

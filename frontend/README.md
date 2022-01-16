# create-svelte

Everything you need to build a Svelte project, powered by [`create-svelte`](https://github.com/sveltejs/kit/tree/master/packages/create-svelte);

## Creating a project

If you're seeing this, you've probably already done this step. Congrats!

```bash
# create a new project in the current directory
npm init svelte@next

# create a new project in my-app
npm init svelte@next my-app
```

> Note: the `@next` is temporary

# Michael's notes

## sveltekit

- https://kit.svelte.dev/
- `npm init svelte@next frontend` - in frontend dir - `npm install` - `npm run dev -- -open`

# js hints

- install nvm (there is a related windows project)
  - `nvm install lts` and `nvm use lts`
- `npm run dev` run a node host (with server-side rendering)
- `npm run build` populate the build directory with static files (based on the sveltekit adapter I chose, the static one)
- `ncu -u` update package.json versions (after installing `npm i -g npm-check-updates`)

# external issues I'm tracking

- sveltekit: better support for relative paths in static sites
  - https://github.com/sveltejs/kit/issues/1480
  - https://github.com/sveltejs/kit/issues/595#issuecomment-842278606
- sveltekit: debug server side code
  - https://github.com/sveltejs/kit/issues/1144
  - https://github.com/vitejs/vite/pull/3928
- sveltekit: routing is totally insane and broken and needs a location.ts store workaround
  - https://github.com/sveltejs/kit/issues/552

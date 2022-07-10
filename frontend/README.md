# js hints

- install nvm (there is a related windows project)
  - `nvm install lts` and `nvm use lts`
  - `nvm install lts` will also update to newer versions
- `npm run dev/build/start` next.js things
- `ncu -u` update package.json versions (after installing `npm i -g npm-check-updates`)
- `npm run lint` run next lint

# external issues I'm tracking

- `Jan 2022`: not enough options for redirects so we can't have nested paths with some paths ending in '/' and some not
  - https://github.com/vercel/next.js/discussions/23988
- `May 2022`: react-zoom-thing has been abandoned for 10 months and hasn't gotten a react 18 update:
  - for now, use `npm install --force` because it still works as a react 17 thing (add `--force` to other npm commands too as needed)
  - https://github.com/prc5/react-zoom-pan-pinch/issues/292
  - 2nd problem, it sets `fit-content` which makes zooming stuff be the wrong size. fixed with some global css to disable `fit-content`
  - https://github.com/prc5/react-zoom-pan-pinch/issues/112

# js hints

- install nvm (there is a related windows project)
  - `nvm install lts` and `nvm use lts`
  - this sequence will also update to newer versions
- `npm run dev/build/start` next.js things
- `ncu -u` update package.json versions (after installing `npm i -g npm-check-updates`)
- `npm run lint` run next lint

# external issues I'm tracking

- `Jul 2022`: can't use getStaticProps and getServerSideProps together, so for example I can't generate the list of plant types for the search page's dropdown and then also make the search page server-side generated https://github.com/vercel/next.js/discussions/11424
- `Jan 2022`: not enough options for redirects so we can't have nested paths with some paths ending in '/' and some not
  - https://github.com/vercel/next.js/discussions/23988
- `May 2022`: react-zoom-thing has been abandoned for 10 months and hasn't gotten a react 18 update:
  - see overrides in package.json - remove these when possible
  - https://github.com/prc5/react-zoom-pan-pinch/issues/292
  - 2nd problem, it sets `fit-content` which makes zooming stuff be the wrong size. fixed with some global css to disable `fit-content`
  - https://github.com/prc5/react-zoom-pan-pinch/issues/112

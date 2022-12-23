# help wanted - code
please contact me if you want to help and I can give more details and make sure work isn't being duplicated
* general frontend/css styling. I'm not interested in this so I've mostly phoned it in
* cherry and apple pollination checking
  * I already have s-allele data imported for cherries, it needs some kind of calculator page
* additional OAuth sources, especially github.  my initial version is google-only
  * an interesting project may be to break the OAuth code into a reusable crate (without database code), it may be useful to other projects
* better search:
  * add locations, types, pages, and full text search of the original json data files
  * some kind of advanced search page with results there instead of only having a search box and dropdown
  * fix limitations like two-letter names and special characters
* filtering and sorting of plant lists
* allow creating a garden map that a user can upload a top view image for and then click-and-drag plant markers
* note taking, calendar markers (like for spray timings)
* user data import/export
* chart customization (colors, sorting, etc.)
* scion trade support (trade calculations, search, notifications etc.)
* unix housekeeping: break backend and frontend into separate users and tighten up folder permissions, consider chroot type stuff
* USDA zone ratings: many references have these and I don't import them. they could be handled with a voting/averaging system similar to the relative harvest days, being aggregated across references.  search could use it as a filter (min/max zone)
* youtube thumbnail fetcher in the style of the pdf and webpage ones in /backend/src/bin/
  * I think this library has what's needed: https://docs.rs/google-youtube3/latest/google_youtube3/api/struct.Thumbnail.html

# Code and Data for [www.fruitfacts.xyz](https://www.fruitfacts.xyz)

* [Backend README](backend/README.md)
  * written in rust using a sqlite database
* [Frontend README](frontend/README.md)
  * written in javascript with next.js and react

# contributing
* The plant database is based on imported publications.  Please search in your local area and see if any useful publications aren't yet imported or look at this list:
* [a list of data files that need help](plant_database/help_needed.md)
* I need software help too:
* [help wanted code projects](help%20wanted.md)

# fruitfacts
1. a project to track typical harvest times for crops, especially tree fruits which have consistent harvest times year-to-year
2. a cross-referencing system for common tree fruits, with an emphasis on information from university agricultural extension publications and other evidence-based sources so gardeners can quickly research the best varieties for their situation
3. an emphasis on citations rather than on editorializing or paraphrasing other works

## goals
* able to reproduce all of the common charts like DWN, ACN, and charts in university extension publications with some level of beauty. charts can be private or public and can be saved to a permalink
* an extensive plant database with harvest dates and references that users can start with, pull into their own charts and then modify. when adding new varieties, a default harvest time should be suggested based on the closest available data or some formula
* each variety's page should contain a list of references with harvest dates and also dates from users if they've set their data to be public
* support a few methods for harvest windows: day of year ranges, relative start like "redhaven+5", or "early/mid/late" (rated in % through the season)
* users shouldn't need to stick to the existing plant database when creating their own charts, but the existing one should be selected as a linked variety whenever possible to help share data and provide good references
* the plant database should be in a simple text format and hosted on github so it can be shared and extended by semi-technical users without working with a database or programming environment
* a map interface to see what nearby u-picks or public gardens are growing so users can find proven varieties to fill in harvest windows
* the web UI should be simple enough to be used by typical retiree gardeners
* all of an individual's data should be able to be imported/exported in a simple text format

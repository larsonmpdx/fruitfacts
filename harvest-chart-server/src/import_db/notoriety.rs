// calculate a notoriety score for each base plant entry, and also each reference
// this helps sort search results and filter browsing to only relevant varieties
// the database has many old varieties that have fallen out of circulation, these should be filtered out most of the time but still accessible if needed

// Number of references > N
// Each reference having a notoriety rating 1-100, and combining that with the publication year of the reference to give a max notoriety
//    -> reference score
// Plant release year
// These three can be combined to an overall score in post processing. Can work on the formula later
//    -> plant score
// Filter out anything that doesn’t clear some bars unless “show all” is checked, or bumping non-notorious ones to the bottom of a search, and generally sorting by notoriety score if it makes more sense than sorting by name or search match quality

// Notoriety categories
// * 100 Extension growing guide
// * 80 U-pick variety list
// * 50 Journal article (not a release article, an actual study)
// * 25 Breeding program list
// * 25 release article
// * 15 breeding program list which is primarily commercial varieties
// * 1 Dictionary
// *   + years-old decay. Extension guides from the 90s are worth a lot less than from the 2010s. Account for “published, updated, reviewed” fields - newest one
// * Put notoriety score text into a field somewhere?

// would like extension guide to decay below u-pick within about 20 years
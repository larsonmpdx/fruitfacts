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

// *   + years-old decay. Extension guides from the 90s are worth a lot less than from the 2010s. Account for “published, updated, reviewed” fields - newest one
// * Put notoriety score text into a field somewhere?

// would like extension guide (100) to decay below u-pick (80) within about 20 years


struct NotorietyEntry<'a> {
    type_: &'a str,
    value: f32,
}

const REFERENCE_NOTORIETY_TABLE: [NotorietyEntry; 8] = [
    NotorietyEntry {
        type_: "state extension guide",
        value: 100.0,
    },
    NotorietyEntry {
        type_: "local extension guide", // guide published by a local office, not a full state guide
        value: 90.0,
    },
    NotorietyEntry {
        type_: "U-pick variety list",
        value: 80.0,
    },
    NotorietyEntry {
        type_: "journal article", // not a release article for one variety - an actual growing test like the OSU table grape trial
        value: 50.0,
    },
    NotorietyEntry {
        type_: "public breeding program list", // PRI apples for example
        value: 25.0,
    },
    NotorietyEntry {
        type_: "release article", // article for one or more varieties
        value: 25.0,
    },
    NotorietyEntry {
        type_: "private breeding program list", // especially breeding programs that supply mainly commercial customers
        value: 15.0,
    },
    NotorietyEntry {
        type_: "dictionary", // for example the register of new fruit and nut cultivars
        value: 1.0,
    },
];

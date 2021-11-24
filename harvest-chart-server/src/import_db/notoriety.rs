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

#[derive(Default)]
pub struct collection_notoriety_decoded {
    pub score: f32,
    pub explanation: String,
}

pub fn collection_notoriety_text_decoder(text: &str) -> collection_notoriety_decoded {

    struct NotorietyEntry<'a> {
        type_: &'a str,
        score: f32,
    }
    
    const REFERENCE_NOTORIETY_TABLE: [NotorietyEntry; 13] = [
        NotorietyEntry {
            type_: "state extension guide",
            score: 100.0,
        },
        NotorietyEntry {
            type_: "local extension guide", // guide published by a local office, not a full state guide
            score: 90.0,
        },
        NotorietyEntry {
            type_: "public garden guide", // not a list of public garden varieties, some actual recommendations from them. for example WWFRF recommendations
            score: 85.0,
        },
        NotorietyEntry {
            type_: "U-pick variety list",
            score: 80.0,
        },
        NotorietyEntry {
            type_: "journal article", // not a release article for one variety - an actual growing test like the OSU table grape trial
            score: 50.0,
        },
        NotorietyEntry {
            type_: "extension test", // same as "journal article" but not published in a journal
            score: 50.0,
        },
        NotorietyEntry {
            type_: "home grower variety list",
            score: 35.0,
        },
        NotorietyEntry {
            type_: "public breeding program list", // PRI apples for example
            score: 25.0,
        },
        NotorietyEntry {
            type_: "release article", // article for one or more varieties
            score: 25.0,
        },
        NotorietyEntry {
            type_: "private breeding program list", // especially breeding programs that supply mainly commercial customers
            score: 15.0,
        },
        NotorietyEntry {
            type_: "IP company list",
            score: 15.0,
        },
        NotorietyEntry {
            type_: "nursery catalog",
            score: 10.0,
        },
        NotorietyEntry {
            type_: "dictionary", // for example the register of new fruit and nut cultivars
            score: 1.0,
        },
    ];

    let mut output: collection_notoriety_decoded = Default::default();
    for entry in REFERENCE_NOTORIETY_TABLE {
        if entry.type_.to_lowercase() == text.to_lowercase() {
            output.score = entry.score; // todo - we may factor in publication year to reduce notoriety of older collections
            output.explanation = format!("{}: score {}/100", entry.type_, entry.score);

            return output;
        }
    }

    panic!("unknown collection type {}", text);
}
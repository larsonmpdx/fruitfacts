// calculate a notoriety score for each base plant entry, and also each reference
// this helps sort search results and filter browsing to only relevant varieties
// the database has many old varieties that have fallen out of circulation, these should be filtered out most of the time but still accessible if needed

#[derive(Default)]
pub struct CollectionNotoriety {
    pub score: f32,
    pub explanation: String,
}

pub fn collection_notoriety_text_decoder(text: &str) -> CollectionNotoriety {
    struct NotorietyEntry<'a> {
        type_: &'a str,
        score: f32,
    }

    const REFERENCE_NOTORIETY_TABLE: [NotorietyEntry; 15] = [
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
            type_: "journal article test", // not a release article for one variety - an actual growing test like the OSU table grape trial
            score: 50.0,
        },
        NotorietyEntry {
            type_: "extension test", // same as "journal article test" but not published in a journal
            score: 50.0,
        },
        NotorietyEntry {
            type_: "localized grower or nursery recommendations",
            score: 45.0,
        },
        NotorietyEntry {
            type_: "journal article", // a journal article that's more or less a survey or dictionary list
            score: 40.0,
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
            type_: "release article or journal overview", // article for one or more varieties
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

    // todo: add age decay. Extension guides from the 90s are worth a lot less than from the 2010s. Account for “published, updated, reviewed” fields - newest one
    // would like extension guide (100) to decay below u-pick (80) within about 20 years

    let mut output: CollectionNotoriety = Default::default();
    for entry in REFERENCE_NOTORIETY_TABLE {
        if entry.type_.to_lowercase() == text.to_lowercase() {
            output.score = entry.score; // todo - we may factor in publication year to reduce notoriety of older collections
            output.explanation = format!("{}: score {}/100", entry.type_, entry.score);

            return output;
        }
    }

    panic!("unknown collection type {}", text);
}

pub struct BasePlantNotorietyInput<'a> {
    pub notoriety_highest_collection_score: Option<f32>,
    pub notoriety_highest_collection_score_name: String,
    pub current_year: i32,
    pub release_year: Option<i32>,
    pub number_of_references: i32,
    pub uspp_number: Option<&'a String>,
}

#[derive(Debug, Default, PartialEq)]
pub struct BasePlantNotoriety {
    pub score: f32,
    pub explanation: String,
}

pub fn base_plant_notoriety_calc(input: &BasePlantNotorietyInput) -> BasePlantNotoriety {
    let mut output: BasePlantNotoriety = Default::default();

    if let Some(score) = input.notoriety_highest_collection_score {
        output.score = score;
        output.explanation = format!(
            "{} ({})",
            score, input.notoriety_highest_collection_score_name
        );
    } else {
        output.score = 1.0; // same as dictionary
        output.explanation = "1.0 (no collection)".to_string();
    }

    // goals: it should be possible for a plant to jump up one or two categories with enough multipliers
    // say from 50 to 80 (1.6x)

    let age_multiplier;
    let age_explanation;
    if let Some(release_year) = input.release_year {
        let age = input.current_year - release_year;

        age_multiplier = match age {
            i32::MIN..=30 => {
                age_explanation = "<=30 years old";
                1.0
            }
            31..=40 => {
                age_explanation = ">30 years old";
                0.9
            }
            41..=i32::MAX => {
                age_explanation = ">40 years old";
                0.85
            }
        };
    } else {
        age_multiplier = 0.8;
        age_explanation = "no release year";
    }
    output.score *= age_multiplier;
    output.explanation += &format!(" *{} ({})", age_multiplier, age_explanation);

    let references_multiplier = match input.number_of_references {
        6..=i32::MAX => 1.3,
        5 => 1.2,
        4 => 1.1,
        3 => 1.0,
        2 => 0.9,
        i32::MIN..=1 => 0.8,
    };
    output.score *= references_multiplier;
    output.explanation += &format!(
        " *{} ({} references)",
        references_multiplier, input.number_of_references
    );

    // multiply by 1.2 if patented
    if input.uspp_number.is_some() {
        let patent_multiplier = 1.2;
        output.score *= patent_multiplier;
        output.explanation += &format!(" *{} (uspp)", patent_multiplier);
    } else {
        output.explanation += &" *1.0 (no uspp number)".to_string();
    }

    output
}

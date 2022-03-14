import { MONTH_START_DAYS, FRUIT_BAR_COLORS } from './constants';

export function minAndMaxX(items) {
    let min_x = 0;
    let min_x_set = false;
    let max_x = 0;
    let max_x_set = false;
    items.forEach((item) => {
        if ((item.x && !min_x_set) || item.x < min_x) {
            min_x_set = true;
            min_x = item.x;
        }
        if (item.x && item.width && (!max_x_set || item.x + item.width > max_x)) {
            max_x_set = true;
            max_x = item.x + item.width;
        }
    });

    return { min_x, max_x };
}
function applySortRelative(items, sortType) {
    switch (sortType) {
        case 'start':
            return items.sort((a, b) => {
                if (a.calc_harvest_relative == b.calc_harvest_relative) {
                    // tiebreaker - sort by name. names are unique within a given type
                    if (a.name < b.name) {
                        return -1;
                    }
                    if (a.name > b.name) {
                        return 1;
                    }
                    return 0;
                }
                return a.calc_harvest_relative - b.calc_harvest_relative;
            });
        default:
            return items;
    }
}

function applySortAbsolute(items, sortType) {
    switch (sortType) {
        case 'start':
            return items.sort((a, b) => {
                if (a.harvest_start == b.harvest_start) {
                    return a.harvest_end - b.harvest_end; // tiebreaker - put varieties with longer windows 2nd
                }
                return a.harvest_start - b.harvest_start;
            });
        default:
            return items;
    }
}

function typeFromSequence(items) {
    let output = '';
    for (const item of items) {
        if (output == '') {
            output = item.type;
        } else {
            if (output != item.type) {
                return 'Mixed';
            }
        }
    }
    return output;
}

const MAX_SEQUENCE_HEIGHT = 40;

function getTypedSequences(items) {
    // phase 1: break items into types
    let output_array = [];
    let output_object = {};
    for (const item of items) {
        if (!(item.type in output_object)) {
            output_object[item.type] = [];
        }
        output_object[item.type].push(item);
    }

    for (const [key, value] of Object.entries(output_object)) {
        output_array.push(value);
    }

    return output_array;
}

const DEFAULT_HARVEST_LENGTH = 10;
const MAX_SEQUENCE_HEIGHT_FOR_A_SINGLE_SEQUENCE = 30;

export function countChartItemsAbsolute(items) {
    const has_absolute_time = items.filter((item) => {
        return item.harvest_start && item.harvest_end;
    });

    return has_absolute_time.length;
}

var isNumber = function isNumber(value) {
    return typeof value === 'number' && isFinite(value);
};

export function countChartItemsRelativeForComparison(items) {
    // we're trying to answer the question - was this chart intended to be relative or absolute? so we only want to look at
    // round == 0.0 which is a direct parse. relative harvest times are filled in from other charts too so we can't just count
    // all values as many charts would become relative when they should be absolute
    const has_relative_time = items.filter((item) => {
        return (
            isNumber(item.calc_harvest_relative) &&
            item.calc_harvest_relative_to &&
            item.calc_harvest_relative_to_type &&
            item.calc_harvest_relative_round == 0.0
        );
    });

    return has_relative_time.length;
}

export function countChartItemsRelative(items) {
    // in this count, get all relative items, even if not round 0
    const has_relative_time = items.filter((item) => {
        return (
            item.calc_harvest_relative &&
            item.calc_harvest_relative_to &&
            item.calc_harvest_relative_to_type
        );
    });

    return has_relative_time.length;
}

function countUnique(iterable) {
    return new Set(iterable).size;
}

export function getChartItemsRelative({ items, sortType, auto_width }) {
    let has_relative_time = items.filter((item) => {
        return (
            isNumber(item.calc_harvest_relative) &&
            item.calc_harvest_relative_to &&
            item.calc_harvest_relative_to_type
        );
    });

    const no_relative_time = items.filter((item) => {
        return !(
            isNumber(item.calc_harvest_relative) &&
            item.calc_harvest_relative_to &&
            item.calc_harvest_relative_to_type
        );
    });

    // if we have only one type, we don't care if it's listed in /backend/generated/relative-relative.json,
    // we can put a 0-line on the chart for it regardless
    // if we have multiple types, we need to filter for only those types in relative-relative.json so we can chart them together
    // in a sensible way

    let types = items.map((item) => item.type);
    let type_count = countUnique(types);

    // set x and width which will be used for follow-on functions
    // todo: set width based on regular harvest times if available
    has_relative_time = has_relative_time.map((item) => {
        return { ...item, x: item.calc_harvest_relative, width: 10 };
    });

    const sequences = getTypedSequences(has_relative_time);
    const output = [];

    sequences.forEach((sequence) => {
        output.push({
            type: typeFromSequence(sequence),
            sequence: applySortRelative(sequence, sortType)
        });
    });

    let multi_sequence_holder = [];
    for (const sequence of output) {
        // todo - make sure we don't leave sequences of a single item. trimmed parts must be N items large
        if (sequence.sequence.length > MAX_SEQUENCE_HEIGHT) {
            while (sequence.sequence.length > MAX_SEQUENCE_HEIGHT) {
                multi_sequence_holder.push({
                    type: sequence.type,
                    sequence: sequence.sequence.splice(0, MAX_SEQUENCE_HEIGHT)
                });
            }
        }
    }

    let typeDays = [];
    if (type_count == 0) {
        // todo - think about what to return in this case?
        return { typeDays, not_charted: no_relative_time };
    }
    if (type_count == 1) {
        return {
            typeDays: [{ type: types[0], x: 0 }],
            sequences: output.concat(multi_sequence_holder),
            not_charted: no_relative_time
        };
    } else {
        // todo: select only types that are in relative-relative.json, then make a vertical line for each and edit other types' days
        // based on the value relative to the earliest type
    }
}

// sortType: only option is "harvest_start" but may include "midpoint" in the future
export function getChartItemsAbsolute({ items, sortType, auto_width }) {
    if (auto_width) {
        items = items.map((item) => {
            if (item.harvest_start && !item.harvest_end) {
                return { ...item, harvest_end: item.harvest_start + DEFAULT_HARVEST_LENGTH };
            } else {
                return item;
            }
        });
    } else {
        // todo, I guess omit items that don't have a start+end date?
    }

    let has_absolute_time = items.filter((item) => {
        return item.harvest_start && item.harvest_end;
    });

    const no_absolute_time = items.filter((item) => {
        return !(item.harvest_start && item.harvest_end);
    });

    // set x and width which will be used for follow-on functions
    has_absolute_time = has_absolute_time.map((item) => {
        return { ...item, x: item.harvest_start, width: item.harvest_end - item.harvest_start };
    });
    //  console.log(JSON.stringify(has_absolute_time, null, 2));

    // if we have few enough items, return them mixed together
    if (has_absolute_time.length <= MAX_SEQUENCE_HEIGHT_FOR_A_SINGLE_SEQUENCE) {
        return {
            sequences: [
                {
                    type: typeFromSequence(has_absolute_time),
                    sequence: applySortAbsolute(has_absolute_time, sortType)
                }
            ],
            not_charted: no_absolute_time
        };
    }

    const sequences = getTypedSequences(has_absolute_time);
    const output = [];

    sequences.forEach((sequence) => {
        output.push({
            type: typeFromSequence(sequence),
            sequence: applySortAbsolute(sequence, sortType)
        });
    });

    // after sorting, break any single types that are too long into multiple sequences
    let multi_sequence_holder = [];
    for (const sequence of output) {
        // todo - make sure we don't leave sequences of a single item. trimmed parts must be N items large
        if (sequence.sequence.length > MAX_SEQUENCE_HEIGHT) {
            while (sequence.sequence.length > MAX_SEQUENCE_HEIGHT) {
                multi_sequence_holder.push({
                    type: sequence.type,
                    sequence: sequence.sequence.splice(0, MAX_SEQUENCE_HEIGHT)
                });
            }
        }
    }

    return { sequences: output.concat(multi_sequence_holder), not_charted: no_absolute_time };
}

function getFruitFillColor(type) {
    if (type in FRUIT_BAR_COLORS) {
        return FRUIT_BAR_COLORS[type].fill;
    } else {
        return '#E6E6FA'; // default
    }
}

// given a set of new chart bars, and all of the existing chart bars, detect overlaps
// and then add vertical space as needed until there is no overlap
function setVerticalOffset(new_bars, existing_bars) {
    let y_offset = 0;
    let re_run = false;

    const BUFFER = 2 * PIXEL_SCALE;

    while (true) {
        //   console.log(`testing y offset ${y_offset}`);
        outer: for (const barA of new_bars) {
            for (const barB of existing_bars) {
                if (
                    barA.x - BUFFER < barB.x + barB.width &&
                    barA.x + barA.width + BUFFER > barB.x &&
                    barA.y + barA.height + y_offset + BUFFER > barB.y &&
                    barA.y + y_offset - BUFFER < barB.y + barB.height
                ) {
                    y_offset += PER_ITEM_HEIGHT * PIXEL_SCALE;
                    //   console.log(`y offset is now ${y_offset}`);
                    re_run = true;
                    break outer;
                }
            }
        }
        if (re_run) {
            re_run = false;
        } else {
            break;
        }
    }

    if (y_offset != 0) {
        for (const bar of new_bars) {
            bar.y += y_offset;
        }
    }
}

// todo: fixed dimensions with a ratio for days instead of pixel-per-day

const MARGIN_X_DAYS = 5;
const PER_ITEM_HEIGHT = 3;
const MARGIN_Y = 5;
const PIXEL_SCALE = 10; // how many pixels per day (or height unit)? use this so our "1px = 1 day" lines don't look so thick on small charts

// create one bar for each item. x is days
export function getBars(sequences) {
    // we want this sort function to be as stable as possible, so it's a little complicated
    sequences.sort((a, b) => {
        if (a.sequence.length == b.sequence.length) {
            // sort by longest sequence first
            if (a.type == b.type) {
                if (a.sequence[0].x == b.sequence[0].x) {
                    // tiebreaker - sort by name of the first element
                    // names are unique within a given type
                    if (a.sequence[0].name < b.sequence[0].name) {
                        return -1;
                    }
                    if (a.sequence[0].name > b.sequence[0].name) {
                        return 1;
                    }
                    return 0;
                } else {
                    // sort by start
                    return a.sequence[0].x - b.sequence[0].x;
                }
            } else {
                // sort by alphabetical type name
                if (a.type < b.type) {
                    return -1;
                }
                if (a.type > b.type) {
                    return 1;
                }
                return 0;
            }
        }
        return a.x - b.x;
    });

    // placement algorithm:
    // place the first sequence at the top
    // place the Nth sequence at the top and look for overlap with any previous sequence

    let overall_output = [];
    for (const sequence of sequences) {
        let y = MARGIN_Y;

        const this_sequence_output = sequence.sequence.map((item) => {
            let output = {
                ...item,
                x: (item.x + MARGIN_X_DAYS) * PIXEL_SCALE,
                width: item.width * PIXEL_SCALE,
                y: y * PIXEL_SCALE,
                height: PER_ITEM_HEIGHT * PIXEL_SCALE,
                fill: getFruitFillColor(item.type)
            };
            y = y + PER_ITEM_HEIGHT;
            return output;
        });

        setVerticalOffset(this_sequence_output, overall_output);

        overall_output = overall_output.concat(this_sequence_output);
    }

    // get extents from bars as-placed
    // console.log(JSON.stringify(overall_output, null, 2));
    const { min_x, max_x } = minAndMaxX(overall_output);
    // console.log(`min x ${min_x} max x ${max_x}`);

    const min_x_scaled = (min_x - MARGIN_X_DAYS) * PIXEL_SCALE;
    const max_x_scaled = (max_x + MARGIN_X_DAYS) * PIXEL_SCALE;

    const min_y = 0;
    let max_y = 0;

    for (const bar of overall_output) {
        if (bar.y + bar.height > max_y) {
            max_y = bar.y + bar.height;
        }
    }
    max_y = max_y + MARGIN_Y * PIXEL_SCALE;

    const width = max_x - min_x;
    const height = max_y - min_y;

    return {
        bars: overall_output,
        extents: {
            min_x: min_x_scaled,
            max_x: max_x_scaled,
            min_x,
            max_x,
            min_y,
            max_y,
            width,
            height
        }
    };
}

export function getTypeLines(extents, typeDays) {
    let majorLines = [];
    let interLines = [];
    let labels = [];

    // create one line per type, with a label
    typeDays.forEach((typeDay) => {
        console.log(typeDay);
        majorLines.push({
            x1: (typeDay.x + MARGIN_X_DAYS) * PIXEL_SCALE,
            x2: (typeDay.x + MARGIN_X_DAYS) * PIXEL_SCALE,
            y1: extents.min_y + MONTH_LABEL_HEIGHT_OFFSET * 2.5 * PIXEL_SCALE,
            y2: extents.max_y - MONTH_LABEL_HEIGHT_OFFSET * 2.5 * PIXEL_SCALE
        });
    });

    // then create interlines at ...-10, +10, +20... days from the first type line



    return { majorLines, interLines, labels };
}

const MONTH_LABEL_HEIGHT_OFFSET = 2;

// a dark line for each month. and text
// todo: optional ligher week lines for 1/4 through each month
export function getMonthLines(extents) {
    let majorLines = [];
    let interLines = [];
    let labels = [];

    let first_month = true;
    MONTH_START_DAYS.forEach((month) => {
        if (
            month.day > extents.min_harvest_start - 20 &&
            month.day < extents.max_harvest_end + 20
        ) {
            majorLines.push({
                x1: (month.day + MARGIN_X_DAYS) * PIXEL_SCALE,
                x2: (month.day + MARGIN_X_DAYS) * PIXEL_SCALE,
                y1: extents.min_y + MONTH_LABEL_HEIGHT_OFFSET * 2.5 * PIXEL_SCALE,
                y2: extents.max_y - MONTH_LABEL_HEIGHT_OFFSET * 2.5 * PIXEL_SCALE
            });
            if (first_month) {
                // only put the midpoint line in before a month one time, the rest of the time it'll come from the after-month midpoint
                interLines.push({
                    x1: (month.day - month.minus_quarter * 2 + MARGIN_X_DAYS) * PIXEL_SCALE,
                    x2: (month.day - month.minus_quarter * 2 + MARGIN_X_DAYS) * PIXEL_SCALE,
                    y1: extents.min_y + MONTH_LABEL_HEIGHT_OFFSET * 2.5 * PIXEL_SCALE,
                    y2: extents.max_y - MONTH_LABEL_HEIGHT_OFFSET * 2.5 * PIXEL_SCALE
                });
                first_month = false;
            }
            interLines.push(
                {
                    x1: (month.day - month.minus_quarter + MARGIN_X_DAYS) * PIXEL_SCALE,
                    x2: (month.day - month.minus_quarter + MARGIN_X_DAYS) * PIXEL_SCALE,
                    y1: extents.min_y + MONTH_LABEL_HEIGHT_OFFSET * 2.5 * PIXEL_SCALE,
                    y2: extents.max_y - MONTH_LABEL_HEIGHT_OFFSET * 2.5 * PIXEL_SCALE
                },
                {
                    x1: (month.day + month.plus_quarter + MARGIN_X_DAYS) * PIXEL_SCALE,
                    x2: (month.day + month.plus_quarter + MARGIN_X_DAYS) * PIXEL_SCALE,
                    y1: extents.min_y + MONTH_LABEL_HEIGHT_OFFSET * 2.5 * PIXEL_SCALE,
                    y2: extents.max_y - MONTH_LABEL_HEIGHT_OFFSET * 2.5 * PIXEL_SCALE
                },
                {
                    x1: (month.day + month.plus_quarter * 2 + MARGIN_X_DAYS) * PIXEL_SCALE,
                    x2: (month.day + month.plus_quarter * 2 + MARGIN_X_DAYS) * PIXEL_SCALE,
                    y1: extents.min_y + MONTH_LABEL_HEIGHT_OFFSET * 2.5 * PIXEL_SCALE,
                    y2: extents.max_y - MONTH_LABEL_HEIGHT_OFFSET * 2.5 * PIXEL_SCALE
                }
            );
            labels.push({
                x: (month.day + 15 + MARGIN_X_DAYS) * PIXEL_SCALE,
                y: extents.min_y + MONTH_LABEL_HEIGHT_OFFSET * PIXEL_SCALE,
                name: month.name
            });
            labels.push({
                x: (month.day + 15 + MARGIN_X_DAYS) * PIXEL_SCALE,
                y: extents.max_y - MONTH_LABEL_HEIGHT_OFFSET * PIXEL_SCALE,
                name: month.name
            });
        }
    });

    return { majorLines, interLines, labels };
}

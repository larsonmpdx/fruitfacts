import { MONTH_START_DAYS, FRUIT_BAR_COLORS } from './constants';

export function minAndMaxDate(sequences) {
    let min_harvest_start = 180;
    let max_harvest_end = 190;
    sequences.forEach((sequence) => {
        sequence.sequence.forEach((item) => {
            if (item.harvest_start && item.harvest_start < min_harvest_start) {
                min_harvest_start = item.harvest_start;
            }
            if (item.harvest_end && item.harvest_end > max_harvest_end) {
                max_harvest_end = item.harvest_end;
            }
        });
    });

    return { min_harvest_start, max_harvest_end };
}

function applySort(items, sortType) {
    switch (sortType) {
        case 'harvest_start':
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

function getSequences(items) {
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

export function getValidChartItems({ items, sortType, auto_width }) {
    if (auto_width) {
        items = items.map((item) => {
            if (item.harvest_start && !item.harvest_end) {
                return { ...item, harvest_end: item.harvest_start + DEFAULT_HARVEST_LENGTH };
            } else {
                return item;
            }
        });
    }

    const filtered = items.filter((item) => {
        return item.harvest_start && item.harvest_end;
    });

    if (filtered.length <= MAX_SEQUENCE_HEIGHT_FOR_A_SINGLE_SEQUENCE) {
        return [{ type: typeFromSequence(filtered), sequence: applySort(filtered, sortType) }];
    }

    const sequences = getSequences(filtered);
    const output = [];

    sequences.forEach((sequence) => {
        output.push({ type: typeFromSequence(sequence), sequence: applySort(sequence, sortType) });
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

    return output.concat(multi_sequence_holder);
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
    const sorted_by_length = sequences.sort((a, b) => {
        if (a.sequence.length == b.sequence.length) {
            if (a.type == b.type) {
                if (a.sequence[0].harvest_start == b.sequence[0].harvest_start) {
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
                    // sort by harvest_start
                    return a.sequence[0].harvest_start - b.sequence[0].harvest_start;
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
        return a.harvest_start - b.harvest_start;
    });

    // placement algorithm:
    // place the first sequence at the top
    // place the Nth sequence at the top and look for overlap with any previous sequence

    let overall_output = [];
    for (const sequence of sequences) {
        let y = MARGIN_Y;

        const this_sequence_output = sequence.sequence.map((item) => {
            let output = {
                x: (item.harvest_start + MARGIN_X_DAYS) * PIXEL_SCALE,
                width: (item.harvest_end - item.harvest_start) * PIXEL_SCALE,
                y: y * PIXEL_SCALE,
                height: PER_ITEM_HEIGHT * PIXEL_SCALE,
                fill: getFruitFillColor(item.type),
                ...item
            };
            y = y + PER_ITEM_HEIGHT;
            return output;
        });

        setVerticalOffset(this_sequence_output, overall_output);

        overall_output = overall_output.concat(this_sequence_output);
    }

    // todo: move Nth element down as needed to fit

    // get extents from bars as-placed
    const { min_harvest_start, max_harvest_end } = minAndMaxDate(sequences);

    const min_x = (min_harvest_start - MARGIN_X_DAYS) * PIXEL_SCALE;
    const max_x = (max_harvest_end + MARGIN_X_DAYS) * PIXEL_SCALE;

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
        extents: { min_harvest_start, max_harvest_end, min_x, max_x, min_y, max_y, width, height }
    };
}

const MONTH_LABEL_HEIGHT_OFFSET = 2;

// a dark line for each month. and text
// todo: optional ligher week lines for 1/4 through each month
export function getMonthLines(extents) {
    let monthLines = [];
    let interLines = [];
    let labels = [];
    const HALF_MONTH = 15;
    const QUARTER_MONTH = 7.5;
    MONTH_START_DAYS.forEach((month) => {
        if (
            month.day > extents.min_harvest_start - 20 &&
            month.day < extents.max_harvest_end + 20
        ) {
            monthLines.push({
                x1: (month.day + MARGIN_X_DAYS) * PIXEL_SCALE,
                x2: (month.day + MARGIN_X_DAYS) * PIXEL_SCALE,
                y1: extents.min_y + MONTH_LABEL_HEIGHT_OFFSET * 2.5 * PIXEL_SCALE,
                y2: extents.max_y - MONTH_LABEL_HEIGHT_OFFSET * 2.5 * PIXEL_SCALE
            });
            interLines.push(
                {
                    x1: (month.day - month.minus_quarter * 2 + MARGIN_X_DAYS) * PIXEL_SCALE,
                    x2: (month.day - month.minus_quarter * 2 + MARGIN_X_DAYS) * PIXEL_SCALE,
                    y1: extents.min_y + MONTH_LABEL_HEIGHT_OFFSET * 2.5 * PIXEL_SCALE,
                    y2: extents.max_y - MONTH_LABEL_HEIGHT_OFFSET * 2.5 * PIXEL_SCALE
                },
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

    return { monthLines, interLines, labels };
}

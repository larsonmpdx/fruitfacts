import { MONTH_START_DAYS, FRUIT_BAR_COLORS } from './constants';

export function minAndMaxDate (items) {
    let min_harvest_start = 180
    let max_harvest_end = 190
    items.forEach(item => {
        if (item.harvest_start && item.harvest_start < min_harvest_start) {
            min_harvest_start = item.harvest_start
        }
        if (item.harvest_end && item.harvest_end > max_harvest_end) {
            max_harvest_end = item.harvest_end
        }
    })

    return { min_harvest_start, max_harvest_end }
}

export function getValidChartItems ({ items, sort, auto_width }) {
    if (auto_width) {
        items = items.map(item => {
            if (item.harvest_start && !item.harvest_end) {
                return { ...item, harvest_end: item.harvest_start + 7 }
            } else {
                return item
            }
        })
    }

    const filtered = items.filter(item => {
        return item.harvest_start && item.harvest_end
    })

    switch (sort) {
        case 'harvest_start':
            return filtered.sort((a, b) => {
                if (a.harvest_start == b.harvest_start) {
                    return a.harvest_end - b.harvest_end // tiebreaker - put varieties with longer windows 2nd
                }
                return a.harvest_start - b.harvest_start
            })
        default:
            return filtered
    }
}

const MARGIN_X_DAYS = 5
const PER_ITEM_HEIGHT = 5
const MARGIN_Y = 5

export function height_px (count) {
    return count * PER_ITEM_HEIGHT + MARGIN_Y * 2
}

export function getExtents ({ min_harvest_start, max_harvest_end, count }) {
    const min_x = min_harvest_start - MARGIN_X_DAYS
    const max_x = max_harvest_end + MARGIN_X_DAYS
    const width = max_x - min_x

    const min_y = 0
    const max_y = height_px(count)
    const height = max_y - min_y

    return { min_x, max_x, min_y, max_y, width, height }
}

function getFruitFillColor(type) {
    if (type in FRUIT_BAR_COLORS) {
        return FRUIT_BAR_COLORS[type].fill;
    } else {
        return '#E6E6FA'; // default
    }
}

// todo: auto colors by variety
// todo: break into variety blocks, or don't
// todo: fixed dimensions with a ratio for days instead of pixel-per-day

// create one bar for each item. x is days
export function getBars (items) {
    let y = MARGIN_Y
    return items.map(item => {
        let output = {
            x: item.harvest_start,
            width: item.harvest_end - item.harvest_start,
            y,
            height: PER_ITEM_HEIGHT,
            fill: getFruitFillColor(item.type),
            ...item
        }
        y = y + PER_ITEM_HEIGHT
        return output
    })
}

const MONTH_LABEL_HEIGHT_OFFSET = 2

// a dark line for each month. and text
// todo: optional ligher week lines for 1/4 through each month
export function getMonthLines (extents) {
    let monthLines = []
    let labels = []
    MONTH_START_DAYS.forEach(month => {
        if (month.day > extents.min_x && month.day < extents.max_x) {
            monthLines.push({
                x1: month.day,
                x2: month.day,
                y1: extents.min_y + MONTH_LABEL_HEIGHT_OFFSET * 2.5,
                y2: extents.max_y - MONTH_LABEL_HEIGHT_OFFSET * 2.5
            })
            labels.push({
                x: month.day,
                y: extents.min_y + MONTH_LABEL_HEIGHT_OFFSET,
                name: month.name
            })
            labels.push({
                x: month.day,
                y: extents.max_y - MONTH_LABEL_HEIGHT_OFFSET,
                name: month.name
            })
        }
    })

    return { monthLines, labels }
}

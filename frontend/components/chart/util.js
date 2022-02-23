export function minAndMaxDate(items) {
    let min_harvest_start = 180;
    let max_harvest_end = 190;
    items.forEach((item) => {
        if (item.harvest_start && item.harvest_start < min_harvest_start) {
            min_harvest_start = item.harvest_start;
        }
        if (item.harvest_end && item.harvest_end > max_harvest_end) {
            max_harvest_end = item.harvest_end;
        }
    });

    return { min_harvest_start, max_harvest_end };
}

export function getValidChartItems({ items, sort, auto_width }) {
    if (auto_width) {
        items = items.map((item) => {
            if (item.harvest_start && !item.harvest_end) {
                return { ...item, harvest_end: item.harvest_start + 7 };
            } else {
                return item;
            }
        });
    }

    const filtered = items.filter((item) => {
        return item.harvest_start && item.harvest_end;
    });

    switch (sort) {
        case 'harvest_start':
            return filtered.sort((a, b) => {
                return a.harvest_start - b.harvest_start;
            });
        default:
            return filtered;
    }
}

const MARGIN_X_DAYS = 5;
const PER_ITEM_HEIGHT = 5;
const MARGIN_Y = 5;

export function height_px(count) {
    return count * PER_ITEM_HEIGHT + MARGIN_Y * 2;
}

function getExtents({ min_harvest_start, max_harvest_end, count }) {
    const min_x = min_harvest_start - MARGIN_X_DAYS;
    const min_y = 0;
    const width = max_harvest_end - min_harvest_start + MARGIN_X_DAYS * 2;
    const height = height_px(count);

    return { min_x, min_y, width, height };
}

export function viewBox({ min_harvest_start, max_harvest_end, count }) {
    // min-x min-y width height
    const extents = getExtents({ min_harvest_start, max_harvest_end, count });
    return `${extents.min_x} ${extents.min_y} ${extents.width} ${extents.height}`;
}

export function getBars(items) {
    let y = MARGIN_Y;
    return items.map((item) => {
        let output = {
            x: item.harvest_start,
            width: item.harvest_end - item.harvest_start,
            y,
            height: PER_ITEM_HEIGHT,
            ...item
        };
        y = y + PER_ITEM_HEIGHT;
        return output;
    });
}

import { countChartItemsAbsolute, countChartItemsRelativeForComparison, countChartItemsRelative, getChartItemsAbsolute, getAbsoluteBars, getRelativeBars, getMonthLines } from './util';

export default function Home({ items }) {

    let absolute_count = countChartItemsAbsolute(items);
    let relative_count = countChartItemsRelativeForComparison(items);
    let relative_count_maximal = countChartItemsRelative(items);

    let sequences = [];
    let not_charted = [];
    let bars = [];
    let extents, monthLines, interLines, labels;

    if(absolute_count >= relative_count) {
        ({sequences, not_charted} = getChartItemsAbsolute({ items, sortType: 'harvest_start', auto_width: true }));
        ({ bars, extents } = getAbsoluteBars(sequences)); // todo - probably get a recommended div height from this which is doing placement
        ({ monthLines, interLines, labels } = getMonthLines(extents));
    } else {
        ({sequences, not_charted, typeLines} = getChartItemsRelative({ items, sortType: 'harvest_start', auto_width: true }));
        ({ bars, extents } = getRelativeBars(sequences));
    }

    if(bars.length > 0) {
    return (
        <div>
            <svg viewBox={`${extents.min_x} ${extents.min_y} ${extents.width} ${extents.height}`}>
                {monthLines.map((line) => (
                    <line
                        x1={line.x1}
                        y1={line.y1}
                        x2={line.x2}
                        y2={line.y2}
                        stroke="black"
                        stroke-width="1"
                    />
                ))}
                {interLines.map((line) => (
                    <line
                        x1={line.x1}
                        y1={line.y1}
                        x2={line.x2}
                        y2={line.y2}
                        stroke="#dbdbdb"
                        stroke-width="1"
                    />
                ))}
                {bars.map((bar) => (
                    <g>
                        <rect
                            x={bar.x}
                            y={bar.y}
                            width={bar.width}
                            height={bar.height}
                            rx={20}
                            ry={20}
                            fill={bar.fill}
                        />
                        <text
                            x={bar.x + 20}
                            y={bar.y + 22}
                            fontFamily="Verdana"
                            fontSize="20"
                            fill="blue"
                        >
                            {bar.name}
                        </text>
                    </g>
                ))}
                {labels.map((label) => (
                    <text x={label.x} y={label.y} fontFamily="Verdana" fontSize="20" fill="blue">
                        {label.name}
                    </text>
                ))}
            </svg>
        </div>
    );
                }
                else {
                    // todo - show a grid of un-charted items
                    return (
                        <div></div>
                    );
                }
}

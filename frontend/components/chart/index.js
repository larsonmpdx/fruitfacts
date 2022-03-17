import {
    countChartItemsAbsolute,
    countChartItemsRelativeForComparison,
    countChartItemsRelative,
    getChartItemsRelative,
    getChartItemsAbsolute,
    getBars,
    getMonthLines,
    getTypeLines
} from './util';

export default function Home({ items }) {
    let absolute_count = countChartItemsAbsolute(items);
    let relative_count = countChartItemsRelativeForComparison(items);
    let relative_count_maximal = countChartItemsRelative(items);

    let sequences = [];
    let not_charted = [];
    let bars = [];
    let majorLines = [];
    let interLines = [];
    let extents = {};
    let labels = [];

    if (absolute_count >= relative_count) {
        ({ sequences, not_charted } = getChartItemsAbsolute({
            items,
            sortType: 'start',
            auto_width: true
        }));
        //   console.log(JSON.stringify(sequences, null, 2));
        ({ bars, extents } = getBars(sequences)); // todo - probably get a recommended div height from this which is doing placement
        //   console.log(JSON.stringify(bars, null, 2));
        //   console.log(JSON.stringify(extents, null, 2));
        ({ majorLines, interLines, labels } = getMonthLines(extents));
        console.log(JSON.stringify(majorLines, null, 2));
    } else {
        let types = [];
        ({ sequences, not_charted, types } = getChartItemsRelative({
            items,
            sortType: 'start',
            auto_width: true
        }));
        ({ bars, extents } = getBars(sequences));
        ({ majorLines, interLines, labels } = getTypeLines(extents, types));
        console.log(JSON.stringify(majorLines, null, 2));
    }

    if (bars.length > 0) {
        return (
            <div>
                <svg
                    className="max-h-screen"
                    viewBox={`${extents.min_x} ${extents.min_y} ${extents.width} ${extents.height}`}
                >
                    {majorLines.map((line) => (
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
                        <text
                            x={label.x}
                            y={label.y}
                            fontFamily="Verdana"
                            fontSize={label.fontSize}
                            fill="blue"
                        >
                            {label.name}
                        </text>
                    ))}
                </svg>
            </div>
        );
    } else {
        // todo - show a grid of un-charted items
        return <div></div>;
    }
}

import { minAndMaxDate, getValidChartItems, height_px, getBars, getExtents, getMonthLines } from './util';

export default function Home({ items }) {
    const valid_items = getValidChartItems({ items, sort: 'harvest_start', auto_width: true });
    const { min_harvest_start, max_harvest_end } = minAndMaxDate(valid_items);
    const bars = getBars(valid_items);

    const extents = getExtents({
        min_harvest_start,
        max_harvest_end,
        count: valid_items.length
    });
    const {monthLines, labels} = getMonthLines(extents);

    console.dir(monthLines);
    return (
        <div className={`w-[50vw] h-[${height_px(valid_items.length)}px]`}>
            <svg
                viewBox={`${extents.min_x} ${extents.min_y} ${extents.width} ${extents.height}`}
            >
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
                {bars.map((bar) => (
                    <g>
                        <rect
                            x={bar.x}
                            y={bar.y}
                            width={bar.width}
                            height={bar.height}
                            rx={2}
                            ry={2}
                            fill={bar.fill}
                        />
                        <text
                            x={bar.x + 2}
                            y={bar.y + 2.5}
                            fontFamily="Verdana"
                            fontSize="2"
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
                        fontSize="2"
                        fill="blue"
                    >
                        {label.name}
                    </text>
                ))}
            </svg>
        </div>
    );
}

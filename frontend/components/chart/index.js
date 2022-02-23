import { minAndMaxDate, getValidChartItems, height_px, viewBox, getBars } from './util';

export default function Home({ items }) {
    const valid_items = getValidChartItems({ items, sort: 'harvest_start', auto_width: true });
    const { min_harvest_start, max_harvest_end } = minAndMaxDate(valid_items);
    const bars = getBars(valid_items);

    return (
        <div className={`w-full h-[${height_px(valid_items.length)}px]`}>
            <svg
                viewBox={viewBox({
                    min_harvest_start,
                    max_harvest_end,
                    count: valid_items.length
                })}
            >
                {bars.map((bar) => (
                    <g>
                        <rect
                            x={bar.x}
                            y={bar.y}
                            width={bar.width}
                            height={bar.height}
                            rx={2}
                            ry={2}
                            fill="lavender"
                        />
                        <text
                            x={bar.x + 2}
                            y={bar.y + 2.5}
                            font-family="Verdana"
                            fontSize="2"
                            fill="blue"
                        >
                            {bar.name}
                        </text>
                    </g>
                ))}
            </svg>
        </div>
    );
}

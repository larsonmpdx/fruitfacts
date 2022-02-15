import Link from 'next/link';

export async function getStaticProps() {
    const json5 = require('json5');
    const fs = require('fs');
    const path = require('path');

    let typesFile = fs.readFileSync(path.join(process.cwd(), '../plant_database/types.json5'));
    const types = json5.parse(typesFile);

    return {
        props: {
            types
        }
    };
}

export default function Home({ types }) {
    return (
        <>
            {types && (
                <>
                    <h1>Plant Types</h1>
                    {types.map((group) => (
                        <>
                            <p>{group.group_name}</p>
                            {group.types.map((type, index) => (
                                <ul key={index}>
                                    <li key={type.name}>
                                        <Link href={`/plants/${type.name}`}>
                                            <img
                                                className="object-scale-down h-24 w-24"
                                                src={'/fruit_icons/' + type.name + '.svg'}
                                            />
                                        </Link>
                                        <Link href={`/plants/${type.name}`}>{type.name}</Link>
                                    </li>
                                </ul>
                            ))}
                        </>
                    ))}
                </>
            )}
        </>
    );
}

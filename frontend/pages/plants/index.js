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
        <article className="prose m-5">
            {types && (
                <>
                    <h2>Plant Types</h2>
                    {types.map((group) => (
                        <>
                            <h3>{group.group_name}</h3>
                            {group.types.map((type, index) => (
                                <ul key={index}>
                                    <li key={type.name}>
                                        <Link href={`/plants/${type.name}`}>
                                            <img
                                                className="my-0 mx-2 inline h-24 w-24 object-scale-down"
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
        </article>
    );
}

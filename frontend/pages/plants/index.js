import Link from 'next/link'

export async function getStaticProps (context) {
    const json5 = require('json5');
    const fs = require('fs');
    const path = require('path');

    let typesFile = fs.readFileSync(path.join(process.cwd(), '../plant_database/types.json5'));
    const types = json5.parse(typesFile);

    return {
        props: {
            types
        }
    }
}

export default function Home({ types }) {
    return (
        <>
            {types && (
                <>
                    <h1>Plant Types</h1>
                    <ul>
                        {types.map(type => (
                            <li key={type.name}>
                                <Link href={`/types/${type.name}`}>
                                    {type.name}
                                </Link>
                            </li>
                        ))}
                    </ul>
                </>
            )}
        </>
    )
}

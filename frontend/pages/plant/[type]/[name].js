import Link from 'next/link';

export async function getServerSideProps(context) {
    const { type, name } = context.query;
    const plant = await fetch(`${process.env.BACKEND_BASE}/api/plants/${type}/${name}`)
        .then((response) => {
            if (response.status !== 200) {
                return {};
            }
            return response.json();
        })
        .catch((error) => {
            console.log(error);
            return {};
        });

    return {
        props: {
            plant
        }
    };
}

export default function Home({ plant }) {
    return (
        <div>
            {plant.base && (
                <h2>
                    {plant.base.name} {plant.base.type}
                </h2>
            )}
            {plant.base?.marketing_name && <h2>marketed as {plant.base.marketing_name} </h2>}

            {plant.base?.uspp_number && <p>USPP {plant.base.uspp_number}</p>}

            {plant.base?.uspp_expiration && <p>expires {plant.base.uspp_expiration}</p>}

            {plant.base?.aka && <p>AKA {plant.base.aka}</p>}

            {plant.base?.release_year && plant.base?.released_by && (
                <p>
                    {plant.base?.release_year && <p>{plant.base.release_year}</p>}
                    {plant.base?.released_by && <p>{plant.base.released_by}</p>}
                </p>
            )}

            {plant.collection && (
                <>
                    {' '}
                    <h1>Collections</h1>
                    <ul>
                        {plant.collection.map((entry) => (
                            <>
                                <li>
                                    <Link
                                        href={`/collections/${encodeURIComponent(
                                            entry.path_and_filename
                                        )}`}
                                    >
                                        {entry.path_and_filename}
                                    </Link>
                                    {entry.description && <p>{entry.description}</p>}
                                </li>
                            </>
                        ))}
                    </ul>
                </>
            )}

            {plant.collection && (
                <>
                    <h1>Harvest Times</h1>
                    <ul>
                        {plant.collection.map((entry) => (
                            <>
                                {entry.harvest_text && (
                                    <p>
                                        {entry.harvest_text}
                                        <Link
                                            href={`/collections/${encodeURIComponent(
                                                entry.path_and_filename
                                            )}`}
                                            title={entry.path_and_filename}
                                        >
                                            [ref]
                                        </Link>
                                    </p>
                                )}
                            </>
                        ))}
                    </ul>
                </>
            )}
        </div>
    );
}

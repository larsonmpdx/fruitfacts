import Link from 'next/link';
import Head from 'next/head';
import { formatPatentDate } from '../../../components/functions';

export async function getServerSideProps(context) {
    const { type, name } = context.query;
    const plant = await fetch(
        `${process.env.NEXT_PUBLIC_BACKEND_BASE}/api/plants/${type}/${encodeURIComponent(name)}`
    )
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
            plant,
            type,
            name
        }
    };
}

export default function Home({ plant, type, name }) {
    return (
        <>
            <Head>
                <title>{`${name} ${type}`}</title>
            </Head>
            <article className="prose m-5">
                {plant.base && (
                    <h2>
                        <img
                            className="my-0 mx-2 inline h-24 w-24 object-contain"
                            src={'/fruit_icons/' + plant.base.type + '.svg'}
                        />
                        {plant.base.name} {plant.base.type}
                    </h2>
                )}
                {plant.base?.marketing_name && (
                    <h2> marketed under the {plant.base.marketing_name} brand </h2>
                )}
                <p>
                    {plant.base?.uspp_number && <>USPP{plant.base.uspp_number} </>}

                    {plant.base?.uspp_expiration && (
                        <>
                            until{' '}
                            {formatPatentDate(
                                plant.base.uspp_expiration,
                                plant.base.uspp_expiration_estimated
                            )}
                        </>
                    )}

                    {plant.base?.uspp_number && (
                        <>
                            {' '}
                            <a
                                href={`https://patents.google.com/patent/USPP${plant.base.uspp_number}`}
                            >
                                google patents
                            </a>
                        </>
                    )}
                </p>
                {plant.base?.aka && <p>AKA {plant.base.aka}</p>}

                {(plant.base?.release_year || plant.base?.released_by) && (
                    <p>
                        released
                        {plant.base?.release_year && <> {plant.base.release_year}</>}
                        {plant.base?.released_by && <> by {plant.base.released_by}</>}
                    </p>
                )}

                {plant.base?.harvest_relative_to && plant.base?.harvest_relative != undefined && (
                    <p>
                        {`${plant.base.harvest_relative_to} ${
                            plant.base.harvest_relative >= 0 ? '+' : ''
                        }${plant.base.harvest_relative}`}
                    </p>
                )}

                {plant.collection && (
                    <>
                        {' '}
                        <h1>Collections</h1>
                        <ul className="list-disc">
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
                        <ul className="list-disc">
                            {plant.collection.map((entry) => (
                                <>
                                    {entry.harvest_text && (
                                        <li>
                                            {`${entry.harvest_text} `}
                                            <Link
                                                href={`/collections/${encodeURIComponent(
                                                    entry.path_and_filename
                                                )}`}
                                                title={entry.path_and_filename}
                                            >
                                                [ref]
                                            </Link>
                                        </li>
                                    )}
                                </>
                            ))}
                        </ul>
                    </>
                )}
            </article>
        </>
    );
}

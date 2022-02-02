import Head from 'next/head';
import Link from 'next/link';
import { format as timeAgo } from 'timeago.js';

export async function getServerSideProps(context) {
    const fact = await fetch(`${process.env.BACKEND_BASE}/api/fact`)
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

    const recentChangesData = await fetch(`${process.env.BACKEND_BASE}/api/recent_changes`)
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
            fact,
            recentChangesData
        }
    };
}

export default function Home({ fact, recentChangesData }) {
    return (
        <div className="container">
            <Head>
                <title>fruitfacts</title>
                <link rel="icon" href="/favicon.ico" />
            </Head>

            <main>
                <div>
                    <Link href="/dirs/">browse locations</Link>
                </div>
                <div>
                    {fact.fact && (
                        <p>
                            {fact.fact}
                            <a href={` ${fact.reference}`}>[ref]</a>
                        </p>
                    )}
                </div>
                <div>
                    {recentChangesData.recent_changes && (
                        <ul>
                            {recentChangesData.recent_changes.collection_changes.map((update) => (
                                <li>
                                    <Link
                                        href={`/collections/${encodeURIComponent(
                                            update.path + update.filename
                                        )}`}
                                    >
                                        {update.filename}
                                    </Link>
                                    {timeAgo(update.git_edit_time * 1000)}
                                </li>
                            ))}
                        </ul>
                    )}
                </div>
                <div>
                    {recentChangesData.recent_changes && (
                        <p>
                            {recentChangesData.recent_changes.base_plants_count} plants in{' '}
                            {recentChangesData.recent_changes.references_count} references
                        </p>
                    )}
                    {recentChangesData.build_info && (
                        <p>
                            updated {timeAgo(recentChangesData.build_info.git_unix_time * 1000)}{' '}
                            build count {recentChangesData.build_info.git_commit_count} git hash{' '}
                            {recentChangesData.build_info.git_hash.substring(0, 7)}
                        </p>
                    )}
                </div>
            </main>

            <footer></footer>
        </div>
    );
}

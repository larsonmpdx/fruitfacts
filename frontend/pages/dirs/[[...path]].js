// ideally we could have one file to render both directory listings and collections like this:
// for a path ending in "/" like /collections/Oregon/ treat this like a folder listing
// for a path ending in something else like /collections/Oregon/USDA-OSU Releases, treat it as an individual collection display

// but (Jan 2022) this isn't possible in next.js because of dumb redirect rules (all URLs get rewritten to either end in '/' or not)
// see https://github.com/vercel/next.js/discussions/23988

// so we have this split between /dirs/[...path].js (directory listings) and /collections/[...path].js (individual collections)
import Link from 'next/link';

export async function getServerSideProps(context) {
    const { path } = context.query;
    let pathUsed;
    if (path) {
        pathUsed = path.join('/') + '/'; // with trailing slash - directory listing
    } else {
        pathUsed = ''; // this combind with the [[...path]].js filename gets us the base path "/dirs" or "/dirs/"
    }
    const data = await fetch(`${process.env.BACKEND_BASE}/api/collections/${pathUsed}`)
        .then((response) => {
            if (response.status !== 200) {
                return [];
            }
            return response.json();
        })
        .catch((error) => {
            console.log(error);
            return [];
        });

    return {
        props: {
            data
        }
    };
}

export default function Home({ data }) {
    return (
        <div>
            {/* multi collection (directory listing) */}

            {data.directories && data.directories.length > 0 && (
                <ul>
                    {data.directories.map((directory) => (
                        <li>
                            <Link href={`/dirs/${directory}`}>{directory}</Link>
                        </li>
                    ))}
                </ul>
            )}

            {data.collections && data.collections.length > 0 && (
                <>
                    <h1>Locations</h1>
                    <ul>
                        {data.collections.map((collection) => (
                            <li>
                                <Link
                                    href={`/collections/${
                                        collection.path + encodeURIComponent(collection.filename)
                                    }`}
                                >
                                    {collection.title}
                                </Link>
                            </li>
                        ))}
                    </ul>
                </>
            )}
        </div>
    );
}
// ideally we could have one file to render both directory listings and collections like this:
// for a path ending in "/" like /collections/Oregon/ treat this like a folder listing
// for a path ending in something else like /collections/Oregon/USDA-OSU Releases, treat it as an individual collection display

// but (Jan 2022) this isn't possible in next.js because of dumb redirect rules (all URLs get rewritten to either end in '/' or not)
// see https://github.com/vercel/next.js/discussions/23988

// so we have this split between /dirs/[...path].js (directory listings) and /collections/[...path].js (individual collections)
import Link from 'next/link';

export async function getServerSideProps(context) {
    const { path } = context.query;
    const data = await fetch(
        `${process.env.NEXT_PUBLIC_BACKEND_BASE}/api/collections/${path.join('/')}`
    ) // no trailing slash - individual collection
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
        <div className="m-5">
            {/* single collection */}
            {data.collection && (
                <article className="prose">
                    <p>
                        {data.collection.title}
                        {data.collection.url && <a href={data.collection.url}>[ref]</a>}
                    </p>
                    <h1>Locations</h1>
                    <ul className="list-disc">
                        {data.locations.map((location) => (
                            <li key={location.id}>{location.location_name}</li>
                        ))}
                    </ul>
                    <h1>Plants</h1>
                    <ul className="list-disc">
                        {data.items.map((item) => (
                            <li key={item.id}>
                                <img
                                    className="my-0 mx-2 inline h-6 w-6 object-scale-down"
                                    src={'/fruit_icons/' + item.type + '.svg'}
                                />
                                <Link
                                    href={`/plant/${encodeURIComponent(
                                        item.type
                                    )}/${encodeURIComponent(item.name)}`}
                                >
                                    {item.name + ' ' + item.type}
                                </Link>

                                {item.marketing_name && (
                                    <>(marketed under the {item.marketing_name} brand)</>
                                )}
                            </li>
                        ))}
                    </ul>
                </article>
            )}
        </div>
    );
}

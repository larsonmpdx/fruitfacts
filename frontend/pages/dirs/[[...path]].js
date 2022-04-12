// ideally we could have one file to render both directory listings and collections like this:
// for a path ending in "/" like /collections/Oregon/ treat this like a folder listing
// for a path ending in something else like /collections/Oregon/USDA-OSU Releases, treat it as an individual collection display

// but (Jan 2022) this isn't possible in next.js because of dumb redirect rules (all URLs get rewritten to either end in '/' or not)
// see https://github.com/vercel/next.js/discussions/23988

// so we have this split between /dirs/[...path].js (directory listings) and /collections/[...path].js (individual collections)
import Link from 'next/link';
import Head from 'next/head';
import dynamic from 'next/dynamic';
import React from 'react';
import throttle from 'lodash/throttle';

// see https://nextjs.org/docs/advanced-features/dynamic-import
const Map = dynamic(() => import('../../components/map'), { ssr: false });

export async function getServerSideProps(context) {
  const { path } = context.query;
  let pathUsed;
  if (path) {
    pathUsed = path.join('/') + '/'; // with trailing slash - directory listing
  } else {
    pathUsed = ''; // this combind with the [[...path]].js filename gets us the base path "/dirs" or "/dirs/"
  }
  const data = await fetch(`${process.env.NEXT_PUBLIC_BACKEND_BASE}/api/collections/${pathUsed}`)
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

  //  console.log(JSON.stringify(data, null, 2));
  return {
    props: {
      data,
      pathUsed
    }
  };
}

export default function Home({ data, pathUsed }) {
  const [click_lonlat, setClick] = React.useState({});
  const [extents, setExtentsForFetch] = React.useState({});
  const [locations, setLocations] = React.useState([]);

  const runFetchLocations = React.useMemo(
    // useMemo(): cache results for each input and don't re-run. appears to not be doing anything
    () =>
      throttle(async (extents, callback) => {
        console.log('hi' + JSON.stringify(extents));

        const response = await fetch(
          `${process.env.NEXT_PUBLIC_BACKEND_BASE}/api/locations?` +
            new URLSearchParams({
              // extents are "[minx, miny, maxx, maxy]"
              min_lon: extents[0],
              min_lat: extents[1],
              max_lon: extents[2],
              max_lat: extents[3],
              limit: 50
            })
        )
          .then((response) => {
            if (response.status !== 200) {
              return;
            }
            return response.json();
          })
          .catch((error) => {
            console.log(error);
            return;
          });

        callback(response);
      }, 650 /* ms to wait */),
    []
  );

  React.useEffect(() => {
    runFetchLocations(extents, (results) => {
      setLocations(results);
      //  console.log(JSON.stringify(results, null, 2));
    });
  }, [extents, runFetchLocations]);

  return (
    <>
      <Head>
        <title>{`dir: ${pathUsed}`}</title>
      </Head>
      <article className="prose m-5">
        {/* multi collection (directory listing) */}
        <Map locations={locations} setClick={setClick} setExtentsForFetch={setExtentsForFetch} />
        <p>click: {`${JSON.stringify(click_lonlat, null, 2)}`}</p>
        <p>extents: {`${JSON.stringify(extents, null, 2)}`}</p>
        {data.directories && data.directories.length > 0 && (
          <ul className="list-disc">
            {data.directories.map((directory, index) => (
              <li key={index}>
                <Link href={`/dirs/${directory}`}>{directory}</Link>
              </li>
            ))}
          </ul>
        )}

        {data.collections && data.collections.length > 0 && (
          <>
            <h1>Locations</h1>
            <ul className="list-disc">
              {data.collections.map((collection) => (
                <li key={collection.id}>
                  <Link
                    href={`/collections/${collection.path}${encodeURIComponent(collection.filename)}
                    `}
                  >
                    {collection.title}
                  </Link>
                </li>
              ))}
            </ul>
          </>
        )}
      </article>
    </>
  );
}

// ideally we could have one file to render both directory listings and collections like this:
// for a path ending in "/" like /collections/Oregon/ treat this like a folder listing
// for a path ending in something else like /collections/Oregon/USDA-OSU Releases, treat it as an individual collection display

// but (Jan 2022) this isn't possible in next.js because of dumb redirect rules (all URLs get rewritten to either end in '/' or not)
// see https://github.com/vercel/next.js/discussions/23988

// so we have this split between /dirs/[...path].js (directory listings) and /collections/[...path].js (individual collections)
import React from 'react';
import Link from 'next/link';
import Head from 'next/head';
import Chart from '../../components/chart';

export async function getServerSideProps(context) {
  let errorMessage = null;
  const { path, loc } = context.query;
  let location_number = parseInt(loc);
  if (isNaN(location_number)) {
    location_number = 1;
  }

  const data = await fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_BASE}/api/collections/${path.join('/')}`
  ) // no trailing slash - individual collection
    .then((response) => {
      if (response.status !== 200) {
        errorMessage = "can't reach backend";
        return { items: [], locations: [] };
      }
      return response.json();
    })
    .catch((error) => {
      errorMessage = `can't reach backend: ${error.message}`;
      console.log(error);
      return { items: [], locations: [] };
    });

  console.log(JSON.stringify(data.locations, null, 2));

  // cut down data to only this location or location #1 if not specified
  data.items = data.items.filter((item) => {
    return item.location_number == location_number;
  });

  const location = data.locations.find((location) => {
    return location.location_number == location_number;
  }) || { location_name: `unknown location #${location_number}` };

  return {
    props: {
      data,
      location,
      path,
      errorMessage
    }
  };
}

// github link example
// plant_database/references/Oregon/2017%20-%20Table%20Grape%20Cultivar%20Performance%20in%20Oregon's%20Willamette%20Valley.json5

export default function Home({
  data,
  location,
  path,
  errorMessage,
  setErrorMessage,
  setContributingLinks
}) {
  let data_link = `plant_database/references/${path
    .map((x) => encodeURIComponent(x))
    .join('/')}.json5`;
  React.useEffect(() => {
    setContributingLinks([
      {
        link: data_link,
        description: `data for this collection`
      },
      {
        link: `/frontend/pages/collections/[...path].js`,
        description: `collections/[...path].js`
      },
      { link: `/frontend/components/chart/`, description: `chart component` }
    ]);
  }, []);

  setErrorMessage(errorMessage);
  return (
    <>
      <article className="prose m-5 max-w-none">
        <Head>
          <title>{`Collection: ${path.join('/')}`}</title>
        </Head>
        {data.collection && (
          <>
            {data.collection.needs_help == 1 && (
              <p>
                {'the data for this collection is marked "needs help". '}
                <a href={`${process.env.NEXT_PUBLIC_GITHUB_BASE}${data_link}`}>
                  click here to view it on github
                </a>
              </p>
            )}
            <p>
              {`${data.collection.title} `}
              {data.collection.url && <a href={data.collection.url}>[ref]</a>}
            </p>
            <h1>Locations</h1>
            <ul className="list-disc">
              {data.locations.length > 1 ? (
                <>
                  {data.locations.map((location) => (
                    <li key={location.id}>
                      <Link
                        href={`/collections/${path.join('/')}?loc=${location.location_number}`}
                        legacyBehavior
                      >
                        {location.location_name}
                      </Link>
                    </li>
                  ))}
                </>
              ) : (
                <>
                  {data.locations.map((location) => (
                    <li key={location.id}>{location.location_name}</li>
                  ))}
                </>
              )}
            </ul>
            {data.locations.length > 1 ? (
              <h1>{`Chart (${location.location_name})`}</h1>
            ) : (
              <h1>Chart</h1>
            )}
            <div className="border-2 border-solid">
              <Chart items={data.items} />
            </div>
            {data.locations.length > 1 ? (
              <h1>{`Plants (${location.location_name})`}</h1>
            ) : (
              <h1>Plants</h1>
            )}
            <ul className="list-none">
              {data.items.map((item) => (
                <li key={item.id}>
                  <img
                    className="my-0 mx-2 inline h-6 w-6 object-contain"
                    src={'/fruit_icons/' + item.type + '.svg'}
                  />
                  <Link
                    href={`/plant/${encodeURIComponent(item.type)}/${encodeURIComponent(
                      item.name
                    )}`}
                    legacyBehavior
                  >
                    {item.name + ' ' + item.type}
                  </Link>
                  {item.marketing_name && <> (marketed under the {item.marketing_name} brand)</>}
                </li>
              ))}
            </ul>
          </>
        )}
      </article>
    </>
  );
}

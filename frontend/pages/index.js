import React from 'react';
import Link from 'next/link';
import Head from 'next/head';
import { format as timeAgo } from 'timeago.js';

// todo:
// - flatten and sort by common name (see types.name_alphabetical)
// - sort by latin name
// - grid/mobile view

export async function getServerSideProps() {
  let errorMessage = null;
  const fact = await fetch(`${process.env.NEXT_PUBLIC_BACKEND_BASE}/api/fact`)
    .then((response) => {
      if (response.status !== 200) {
        errorMessage = "can't reach backend";
        return {};
      }
      return response.json();
    })
    .catch((error) => {
      errorMessage = `can't reach backend: ${error.message}`;
      console.log(error);
      return {};
    });

  const recentChangesData = await fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_BASE}/api/recent_changes`
  )
    .then((response) => {
      if (response.status !== 200) {
        errorMessage = "can't reach backend";
        return {};
      }
      return response.json();
    })
    .catch((error) => {
      errorMessage = `can't reach backend: ${error.message}`;
      console.log(error);
      return {};
    });

  return {
    props: {
      fact,
      recentChangesData,
      errorMessage
    }
  };
}

export default function Home({
  fact,
  recentChangesData,
  errorMessage,
  setErrorMessage,
  setContributingLinks
}) {
  React.useEffect(() => {
    setContributingLinks([{ link: `/frontend/pages/index.js`, description: `index.js` }]);
  }, []);

  setErrorMessage(errorMessage);
  return (
    <article className="prose m-5">
      <Head>
        <title>fruitfacts</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main>
        <div>
          {fact.fact && (
            <p>
              <b>Fact:</b> {`${fact.fact} `}
              <a href={` ${fact.reference}`}>[ref]</a>
            </p>
          )}
        </div>
        <div>
          {recentChangesData.recent_changes && (
            <>
              <h3>Recent Changes</h3>
              <ul className="list-disc">
                {recentChangesData.recent_changes.collection_changes.map((update, index) => (
                  <li key={index}>
                    <Link
                      href={`/collections/${encodeURIComponent(update.path + update.filename)}`}
                    >
                      {update.filename}
                    </Link>
                    <div className="m-1 inline">{timeAgo(update.git_edit_time * 1000)}</div>
                  </li>
                ))}
              </ul>
            </>
          )}
        </div>
        <div>
          {(recentChangesData.recent_changes || recentChangesData.build_info) && (
            <h3>Build Info</h3>
          )}
          <>
            {recentChangesData.recent_changes && (
              <div className="inline">
                {recentChangesData.recent_changes.base_plants_count} plants in{' '}
                {recentChangesData.recent_changes.references_count} references.
              </div>
            )}
            {recentChangesData.build_info && (
              <div className="m-1 inline">
                updated {timeAgo(recentChangesData.build_info.git_unix_time * 1000)}
                {', '}
                build count {recentChangesData.build_info.git_commit_count},{' '}
                <a
                  href={` ${`${process.env.NEXT_PUBLIC_GITHUB_HOMEPAGE}/tree/${recentChangesData.build_info.git_hash}`}`}
                >
                  git hash {recentChangesData.build_info.git_hash.substring(0, 7)}
                </a>
              </div>
            )}
          </>
        </div>
      </main>

      <footer></footer>
    </article>
  );
}

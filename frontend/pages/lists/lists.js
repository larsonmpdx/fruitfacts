import React from 'react';
import * as qs from 'qs';
import { useRouter } from 'next/router';
import Link from 'next/link';

export default function Home({ user, setContributingLinks, setErrorMessage }) {
  React.useEffect(() => {
    setContributingLinks([
      { link: `/frontend/pages/lists/list.js`, description: `list.js` },
      { link: `/backend/src/queries/list.rs`, description: `backend C/U/D lists` },
      { link: `/backend/src/queries/search.rs`, description: `backend read lists` }
    ]);
  }, []);

  const router = useRouter();
  const query = qs.parse(router.asPath.split(/\?/)[1]);

  const [searchReturn, setSearchReturn] = React.useState(null);

  React.useEffect(() => {
    // todo - this is too many lines for what it does. simplify (and other occurrences)
    const fetchData = async () => {
      const data = await fetch(
        `${process.env.NEXT_PUBLIC_BACKEND_BASE}/api/search?searchType=user_loc&user=${query.user}`,
        {
          // todo - probably should use user name
          credentials: 'include'
        }
      )
        .then((response) => {
          if (response.status === 200) {
            return response.json();
          } else {
            setErrorMessage("couldn't get user lists");
            return null;
          }
        })
        .catch((error) => {
          setErrorMessage(`couldn't get user lists: ${error.message}`);
          console.log(error);
          return null;
        });

      setSearchReturn(data);
    };

    fetchData();
  }, []);

  // todo:
  // add list button
  // list of lists, with link to view each list, counts, edit buttons, delete buttons

  return (
    <>
      <p>user lists</p>
      {user?.id && `id:${user?.id}` == query.user && (
        <Link
          href={`/lists/addList`}
          className="mt-4 mr-4 block text-teal-200 hover:text-white lg:mt-0 lg:inline-block"
        >
          add list
        </Link>
      )}
      {searchReturn?.locations?.length ? (
        <>
          {searchReturn.locations.map((location) => (
            <li key={location.location_name}>
            <Link
              href={`/collections/user/${query.user}/${location.location_name}`}
              className="mt-4 mr-4 block text-teal-200 hover:text-white lg:mt-0 lg:inline-block"
            >
              {location.location_name}
            </Link>
            </li>
          ))}
        </>
      ) : (
        <p>no lists</p>
      )}
    </>
  );
}

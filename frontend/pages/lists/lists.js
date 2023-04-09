import React from 'react';
import * as qs from 'qs';
import { useRouter } from 'next/router';
import Link from 'next/link';
import { XCircleIcon } from '@heroicons/react/20/solid';
import ConfirmModal from '../../components/confirmModal';
import { name_to_path } from '../../components/util';

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
  const [toDelete, setToDelete] = React.useState({});
  const [deleteModalVisible, setDeleteModalVisible] = React.useState(false);

  const deleteList = async () => {
    await fetch(`${process.env.NEXT_PUBLIC_BACKEND_BASE}/api/list`, {
      method: 'POST',
      credentials: 'include',
      body: JSON.stringify({
        location: toDelete.location_name,
        user: query.user,
        delete: true
      })
    })
      .then((response) => {
        if (response.status === 200) {
          // todo - update list by deleting the item in-place?
          fetchData();
        } else {
          setErrorMessage("couldn't delete list");
        }
      })
      .catch((error) => {
        setErrorMessage(`couldn't delete list: ${error.message}`);
        console.log(error);
      });

    setDeleteModalVisible(false);
  };

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

  React.useEffect(() => {
    // fetch once at page load
    fetchData();
  }, []);

  // todo:
  // add list button
  // list of lists, with link to view each list, counts, edit buttons, delete buttons

  return (
    <>
      <ConfirmModal
        enabled={deleteModalVisible}
        okFunction={deleteList}
        cancelFunction={() => {
          setDeleteModalVisible(false);
        }}
        title={`Delete List "${toDelete.location_name}"?`}
        text="this will delete your account and all of your lists"
      ></ConfirmModal>
      <p>user lists</p>
      {user?.id && `id:${user?.id}` == query.user && (
        <Link
          href={`/lists/addList`}
          className="mr-4 mt-4 hover:bg-indigo-800 hover:text-white lg:mt-0"
        >
          add list
        </Link>
      )}
      {searchReturn?.locations?.length ? (
        <>
          {searchReturn.locations.map((location) => (
            <li key={location.location_name}>
              <button
                className="ml-1 mr-4 mt-4 block text-teal-200 hover:text-teal-800 lg:mt-0 lg:inline-block"
                onClick={() => {
                  setToDelete(location);
                  setDeleteModalVisible(true);
                }}
              >
                <XCircleIcon className="mx-2 my-0 inline h-6 w-6 object-contain" />
              </button>
              <Link
                href={`/collections/user/${query.user}/${name_to_path(location.location_name)}`}
                className="mr-4 mt-4 hover:bg-indigo-800 hover:text-white lg:mt-0"
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

import React from 'react';
import { useRouter } from 'next/router';
import * as qs from 'qs';
import Button from '../../components/button';

export default function Home({ user, setErrorMessage, setContributingLinks }) {
  React.useEffect(() => {
    setContributingLinks([
      { link: `/frontend/pages/lists/addPlant.js`, description: `list.js` },
      { link: `/backend/src/queries/list.rs`, description: `backend C/U/D lists` }
    ]);
  }, []);

  const [name, setName] = React.useState('');

  const router = useRouter();
  const query = qs.parse(router.asPath.split(/\?/)[1]);

  const handleSubmit = async () => {
    await fetch(`${process.env.NEXT_PUBLIC_BACKEND_BASE}/api/list/entry`, {
      method: 'POST',
      credentials: 'include',
      body: JSON.stringify({
        location_name: name,
        latitude: lat,
        longitude: lon,
        user: 'id:' + user.id,
        location_number: 0, // special case for user collections
        notoriety_score: 0.0, // unused here but set NOT NULL
        ignore_for_nearby_searches: 0 // unused here but set NOT NULL
      })
    })
      .then((response) => {
        if (response.status !== 200) {
          response.text().then((text) => {
            setErrorMessage(`backend API error: ${text}`);
          });
        } else {
          // todo - redirect to edit list
        }
      })
      .catch((error) => {
        setErrorMessage(`can't reach backend: ${error.message}`);
        console.log(error);
      });
  };

  return (
    <>
      <p>add </p>
      <label>
        Name:
        <input
          type="text"
          value={name}
          className="block w-80 rounded-lg border border-gray-300 bg-gray-50 p-2.5 text-sm text-gray-900 focus:border-blue-500 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white dark:placeholder-gray-400 dark:focus:border-blue-500 dark:focus:ring-blue-500"
          onChange={(event) => setName(event.target.value)}
        />
      </label>
      <Button
        enabled={true}
        onClick={async () => {
          await handleRequestLocation();
        }}
        className="focus:shadow-outline h-12 w-80 rounded-lg bg-indigo-700 px-6 text-indigo-100 transition-colors duration-150 hover:bg-indigo-800"
        label="get location from my browser"
      />
      <label>
        lat:
        <input
          type="text"
          value={lat}
          className="block w-80 rounded-lg border border-gray-300 bg-gray-50 p-2.5 text-sm text-gray-900 focus:border-blue-500 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white dark:placeholder-gray-400 dark:focus:border-blue-500 dark:focus:ring-blue-500"
          onChange={(event) => setLat(event.target.value)}
        />
      </label>
      <label>
        lon:
        <input
          type="text"
          value={lon}
          className="block w-80 rounded-lg border border-gray-300 bg-gray-50 p-2.5 text-sm text-gray-900 focus:border-blue-500 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white dark:placeholder-gray-400 dark:focus:border-blue-500 dark:focus:ring-blue-500"
          onChange={(event) => setLon(event.target.value)}
        />
      </label>
      <label>
        <input type="checkbox" defaultChecked={makePublic} onClick={handleMakePublic} />
        make public?
      </label>
      <Button
        enabled={true}
        onClick={async () => {
          await handleSubmit();
        }}
        className="focus:shadow-outline h-12 w-80 rounded-lg bg-indigo-700 px-6 text-indigo-100 transition-colors duration-150 hover:bg-indigo-800"
        label="submit"
      />
    </>
  );
}
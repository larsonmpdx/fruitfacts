import React from 'react';
import Button from '../../components/button';

export default function Home({ setErrorMessage, setContributingLinks }) {
  React.useEffect(() => {
    setContributingLinks([
      { link: `/frontend/pages/lists/addList.js`, description: `list.js` },
      { link: `/backend/src/queries/list.rs`, description: `backend C/U/D lists` }
    ]);
  }, []);

  const [name, setName] = React.useState('');
  const [lat, setLat] = React.useState('');
  const [lon, setLon] = React.useState('');

  const handleRequestLocation = () => {
    navigator.geolocation.getCurrentPosition(function (position) {
      console.log('Latitude is :', position.coords.latitude);
      console.log('Longitude is :', position.coords.longitude);
      setLat(position.coords.latitude);
      setLon(position.coords.longitude);
    });

    return; // todo
  };

  const handleSubmit = async () => {
    const data = await fetch(`${process.env.NEXT_PUBLIC_BACKEND_BASE}/api/list`,
    {
      method: 'POST',
      credentials: 'include',
      body: JSON.stringify({
        location_name: name,
        latitude: lat,
        longitude: lon,
        user_id: "todo",
      })
    })
    .then((response) => {
      if (response.status !== 200) {
        setErrorMessage("can't reach the backend");
        return [];
      }
      return response.json();
    })
    .catch((error) => {
      setErrorMessage(`can't reach backend: ${error.message}`);
      console.log(error);
      return [];
    });
  };

  // todo:
  // support both new list creation and editing
  // submit box only goes active when fields are ready. maybe with hints?
  // list name (text box)
  // location (get user location, or pick on a map, or (todo) zip code)

  // todo - notoriety score is currently not null, switch that to optional
  // initial design - name only

  return (
    <>
      <p>create list</p>
      <form onSubmit={handleSubmit}>
        <label>
          Name:
          <input type="text" value={name} onChange={(event) => setName(event.target.value)} />
        </label>
          <label>
            lat:
            <input type="text" value={lat} onChange={(event) => setLat(event.target.value)} />
          </label>
          <label>
          lon:
          <input type="text" value={lon} onChange={(event) => setLon(event.target.value)} />
        </label>
      </form>
      <Button
        enabled={true}
        onClick={async () => {
          await handleRequestLocation();
        }}
        className="focus:shadow-outline h-12 w-full rounded-lg bg-indigo-700 px-6 text-indigo-100 transition-colors duration-150 hover:bg-indigo-800"
        label="get location from my browser"
      />

      <Button
        enabled={true}
        onClick={async () => {
          await handleSubmit();
        }}
        className="focus:shadow-outline h-12 w-full rounded-lg bg-indigo-700 px-6 text-indigo-100 transition-colors duration-150 hover:bg-indigo-800"
        label="submit"
      />
    </>
  );
}

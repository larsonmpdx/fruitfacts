import React from 'react';
import Button from '../../components/button';

export default function Home({ setContributingLinks }) {
  React.useEffect(() => {
    setContributingLinks([
      { link: `/frontend/pages/lists/addList.js`, description: `list.js` },
      { link: `/backend/src/queries/list.rs`, description: `backend C/U/D lists` }
    ]);
  }, []);

  const [zipEnabled, setZipEnabled] = React.useState('');
  const [name, setName] = React.useState('');
  const [zip, setZip] = React.useState('');
  const [latLon, setLatLon] = React.useState('');

  const handleEnterAsZip = () => {
    setZipEnabled(true);
    return; // todo
  };

  const handleRequestLocation = () => {
    setZipEnabled(false);
    navigator.geolocation.getCurrentPosition(function (position) {
      console.log('Latitude is :', position.coords.latitude);
      console.log('Longitude is :', position.coords.longitude);
      setLatLon(`${position.coords.latitude},${position.coords.longitude}`)
    });

    return; // todo
  };

  const handleSubmit = () => {
    return; // todo
  };

  // todo:
  // support both new list creation and editing
  // submit box only goes active when fields are ready. maybe with hints?
  // list name (text box)
  // location (get user location, or pick on a map, or zip code)

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
        {zipEnabled ? (
          <label>
            zip code:
            <input type="text" value={zip} onChange={(event) => setZip(event.target.value)} />
          </label>
        ) : (
          <label>
            lat,lon:
            <input type="text" value={latLon} onChange={(event) => setLatLon(event.target.value)} />
          </label>
        )}
      </form>
      <Button
        enabled={true}
        onClick={async () => {
          await handleEnterAsZip();
        }}
        className="focus:shadow-outline h-12 w-full rounded-lg bg-indigo-700 px-6 text-indigo-100 transition-colors duration-150 hover:bg-indigo-800"
        label="use a zip code for location"
      />
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

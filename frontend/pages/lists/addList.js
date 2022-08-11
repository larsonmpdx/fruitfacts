import React from 'react';
import Button from '../../components/button';

export default function Home({ setContributingLinks }) {
  React.useEffect(() => {
    setContributingLinks([
      { link: `/frontend/pages/lists/addList.js`, description: `list.js` },
      { link: `/backend/src/queries/list.rs`, description: `backend C/U/D lists` }
    ]);
  }, []);

  const submit = () => {
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
      <p>user lists</p>
      <Button
                enabled={true}
                onClick={async () => {
                  await submit();
                }}
                className="focus:shadow-outline h-12 w-full rounded-lg bg-indigo-700 px-6 text-indigo-100 transition-colors duration-150 hover:bg-indigo-800"
                label="submit"
              />

    </>
  );
}

// after the backend completes an oauth login and gets a redirect back to the backend
// it will check the login info against the database. if no account is found it redirects here
// so the user is prompted to create an account (or not)
import React from 'react';
import Button from '../components/button';
import ButtonLink from '../components/buttonLink';
import { DebounceInput } from 'react-debounce-input';

export default function Home({ setErrorMessage }) {
  const [user, setUser] = React.useState();
  const [clicked, setClicked] = React.useState();

  const [name, setName] = React.useState('');
  const [nameFetched, setNameFetched] = React.useState(false);
  const [createEnabled, setCreateEnabled] = React.useState(false);

  const handleNameChanged = async (name) => {
    setName(name);
    setNameFetched(false);

    // check name availability
    await fetch(
      `${process.env.NEXT_PUBLIC_BACKEND_BASE}/api/createAccount?name=${name}&check_only=true`,
      {
        credentials: 'include'
      }
    )
      .then((response) => {
        setNameFetched(true);
        if (response.status !== 200) {
          setCreateEnabled(false);
        } else {
          setCreateEnabled(name.length > 0);
        }
      })
      .catch((error) => {
        setErrorMessage(`failed checking username availability: ${error.message}`);
        console.log(error);
        return { failed: error };
      });
  };

  const createAccount = async () => {
    if (clicked) {
      return;
    }
    setClicked(true);

    const user = await fetch(
      `${process.env.NEXT_PUBLIC_BACKEND_BASE}/api/createAccount?name=${name}`,
      {
        credentials: 'include'
      }
    )
      .then((response) => {
        if (response.status !== 200) {
          return response.text().then((text) => {
            setErrorMessage(`failed creating user: ${text}`);
            return { failed: text };
          });
        }
        return response.json();
      })
      .catch((error) => {
        setErrorMessage(`failed creating user: ${error.message}`);
        console.log(error);
        return { failed: error };
      });
    setUser(user);
  };

  return (
    <div className="flex h-screen items-center justify-center">
      <div className="columns-1">
        {!user && (
          <>
            <div className="rounded-lg border bg-indigo-800 p-10 font-bold text-white shadow-lg">
              <p>no {process.env.NEXT_PUBLIC_SITE_NAME} account found. create one?</p>
            </div>
            <div className="rounded-lg border bg-indigo-800 p-10 font-bold text-white shadow-lg">
              <div className="flow-root">
                <label>Choose an account name (this name will be visible to other users):</label>
                <div className="flow-root">
                  <DebounceInput
                    debounceTimeout={300}
                    type="text"
                    value={name}
                    className="float-left h-12 w-3/5 rounded-lg border border-gray-300 bg-gray-50 p-2.5 text-sm text-gray-900 focus:border-blue-500 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white dark:placeholder-gray-400 dark:focus:border-blue-500 dark:focus:ring-blue-500"
                    onChange={(event) => handleNameChanged(event.target.value)}
                  />
                  {nameFetched && name.length > 0 && (
                    <div className="float-right flex h-12 w-2/5 flex-col items-center justify-center rounded-lg px-6 align-middle text-sm font-bold text-indigo-100 shadow-lg">
                      {' '}
                      {createEnabled ? (
                        <p className="text-white">name available</p>
                      ) : (
                        <p className="text-red-500">name taken</p>
                      )}{' '}
                    </div>
                  )}
                </div>
              </div>
            </div>
            <div className="rounded-lg border bg-indigo-800 p-10 font-bold text-white shadow-lg">
              <Button
                enabled={!clicked && createEnabled}
                onClick={async () => {
                  await createAccount();
                }}
                className="focus:shadow-outline h-12 w-80 rounded-lg bg-indigo-700 px-6 text-indigo-100 transition-colors duration-150 hover:bg-indigo-800"
                label="create account"
              />
            </div>
          </>
        )}
        {user && (
          <div className="rounded-lg border bg-indigo-800 p-10 font-bold text-white shadow-lg">
            {user?.failed ? (
              <p>failed creating account: {user.failed}</p>
            ) : (
              <pre className="whitespace-pre-wrap break-all">
                account created: {JSON.stringify(user, null, 2)}
              </pre>
            )}
          </div>
        )}
        <div className="rounded-lg border bg-indigo-800 p-10 font-bold text-white shadow-lg">
          <ButtonLink
            enabled={true}
            href="/"
            label={`Back to ${process.env.NEXT_PUBLIC_SITE_NAME}`}
            className="focus:shadow-outline h-12 w-80 rounded-lg bg-indigo-700 px-6 text-indigo-100 transition-colors duration-150 hover:bg-indigo-800"
          />
        </div>
      </div>
    </div>
  );
}

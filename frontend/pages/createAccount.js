// after the backend completes an oauth login and gets a redirect back to the backend
// it will check the login info against the database. if no account is found it redirects here
// so the user is prompted to create an account (or not)
import React from 'react';
import ButtonLink from '../components/buttonLink';
import Button from '../components/button';

export default function Home({ setErrorMessage }) {
  const [user, setUser] = React.useState();
  const [clicked, setClicked] = React.useState();

  const [name, setName] = React.useState('');
  const [createEnabled, setCreateEnabled] = React.useState(false);

  const handleSetName = (name) => {
    setName(name);

    setCreateEnabled(name.length > 0);
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
              <p>
                external login was successful but no {process.env.NEXT_PUBLIC_SITE_NAME} account was
                found. create one?
              </p>
            </div>
            <label>
              Choose an account name (this name will be visible to other users):
              <input
                type="text"
                value={name}
                className="block w-80 rounded-lg border border-gray-300 bg-gray-50 p-2.5 text-sm text-gray-900 focus:border-blue-500 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white dark:placeholder-gray-400 dark:focus:border-blue-500 dark:focus:ring-blue-500"
                onChange={(event) => handleSetName(event.target.value)}
              />
            </label>
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
              <pre>account created: {JSON.stringify(user, null, 2)}</pre>
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

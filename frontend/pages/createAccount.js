// after the backend completes an oauth login and gets a redirect back to the backend
// it will check the login info against the database. if no account is found it redirects here
// so the user is prompted to create an account (or not)
import React from 'react';
import Link from 'next/link';

export default function Home({ setErrorMessage }) {
  const [user, setUser] = React.useState();
  const [disabled, setDisabled] = React.useState();

  const createAccount = async () => {
    if (disabled) {
      return;
    }
    setDisabled(true);

    const user = await fetch(`${process.env.NEXT_PUBLIC_BACKEND_BASE}/api/createAccount`, {
      credentials: 'include'
    })
      .then((response) => {
        if (response.status !== 200) {
          setErrorMessage('failed creating user');
          return { failed: response.status };
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
    <>
      {!user && (
        <p>
          external login was successful but no {process.env.NEXT_PUBLIC_SITE_NAME} account was
          found. create one?
        </p>
      )}
      <button
        disabled={disabled}
        onClick={async () => {
          await createAccount();
        }}
      >
        create account
      </button>
      {user && (
        <>
          {user?.failed ? (
            <p>failed creating account: {user.failed}</p>
          ) : (
            <p>account created: {JSON.stringify(user)}</p>
          )}
        </>
      )}
      <Link href="/">{`Back to ${process.env.NEXT_PUBLIC_SITE_NAME}`}</Link>
    </>
  );
}

import React from 'react';

export default function Home({ setErrorMessage }) {
  const [authURLs, setAuthURLs] = React.useState(null);

  React.useEffect(() => {
    // todo - this is too many lines for what it does. simplify (and other occurrences)
    const fetchData = async () => {
      const data = await fetch(`${process.env.NEXT_PUBLIC_BACKEND_BASE}/api/authURLs`, {
        credentials: 'include'
      })
        .then((response) => {
          if (response.status === 200) {
            return response.json();
          } else {
            setErrorMessage("couldn't log in");
            return null;
          }
        })
        .catch((error) => {
          setErrorMessage(`couldn't log in: ${error.message}`);
          console.log(error);
          return null;
        });

      setAuthURLs(data);
    };

    fetchData();
  }, []);

  return <>{authURLs && <a href={authURLs.google}>log in with google oauth</a>}</>;
}

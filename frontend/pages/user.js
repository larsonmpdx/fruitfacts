import React from 'react';

export default function Home({ user, setContributingLinks }) {
  React.useEffect(() => {
    setContributingLinks([{ link: `/frontend/pages/user.js`, description: `user.js` }]);
  }, []);

  return <>{user && <p>{JSON.stringify(user)}</p>}</>;
}

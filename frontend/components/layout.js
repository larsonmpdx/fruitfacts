import React from 'react';
import { useRouter } from 'next/router';
import Navbar from './navbar';
import Errorbar from './errorbar';

export default function Layout({ children }) {
  const [user, setUser] = React.useState({});
  const [errorMessage, setErrorMessage] = React.useState(null);
  const [contributingLinks, setContributingLinks] = React.useState([]);

  const router = useRouter();
  if (!['/login', '/createAccount'].includes(router.pathname)) {
    return (
      <>
        <noscript>
          <p>{process.env.NEXT_PUBLIC_SITE_NAME} works better with javascript</p>
        </noscript>
        <Navbar user={user} setUser={setUser} contributingLinks={contributingLinks} />
        <Errorbar errorMessage={errorMessage} />
        <main>
          {React.cloneElement(children, {
            user,
            setUser,
            setErrorMessage, // share these to every other thing within <main>
            setContributingLinks
          })}
        </main>
      </>
    );
  } else {
    return (
      <>
        <noscript>
          <p>{process.env.NEXT_PUBLIC_SITE_NAME} works better with javascript</p>
        </noscript>
        <Errorbar errorMessage={errorMessage} />
        <main>
          {React.cloneElement(children, {
            setErrorMessage // share this to every other thing within <main>
          })}
        </main>
      </>
    );
  }
}

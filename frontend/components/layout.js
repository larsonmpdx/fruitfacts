import Link from './Link';
import React from 'react';
import { useRouter } from 'next/router';
import Search from './navbarSearch';
import Login from './navbarLogin';

export default function Layout({ children }) {
    const [user, setUser] = React.useState({});

    const router = useRouter();
    if (!['/login', '/createAccount'].includes(router.pathname)) {
        return (
            <>
                <noscript>
                    <p>{process.env.NEXT_PUBLIC_SITE_NAME} works better with javascript</p>
                </noscript>
                <Link href="/">Fruitfacts</Link>
                <Search />
                <Login user={user} setUser={setUser} />
                <main>
                    {React.cloneElement(children, {
                        user
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
                <main>{children}</main>
            </>
        );
    }
}

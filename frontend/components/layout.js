import React from 'react';
import { useRouter } from 'next/router';
import Navbar from './navbar';

export default function Layout({ children }) {
    const [user, setUser] = React.useState({});

    const router = useRouter();
    if (!['/login', '/createAccount'].includes(router.pathname)) {
        return (
            <>
                <noscript>
                    <p>{process.env.NEXT_PUBLIC_SITE_NAME} works better with javascript</p>
                </noscript>
                <Navbar user={user} setUser={setUser} />
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

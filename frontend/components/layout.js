import Link from 'next/link';
import React from 'react';
import { useRouter } from 'next/router';
import Search from './navbarSearch';
import Login from './navbarLogin';

export default function Layout({ children }) {
    const [user, setUser] = React.useState({});

    const router = useRouter();
    if (router.pathname != '/login') {
        return (
            <>
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
        return <main>{children}</main>;
    }
}

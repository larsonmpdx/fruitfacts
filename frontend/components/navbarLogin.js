import * as React from 'react';

export default function Home({ user, setUser }) {
    React.useEffect(() => {
        const fetchData = async () => {
            const data = await fetch(`${process.env.NEXT_PUBLIC_BACKEND_BASE}/api/checkLogin`, {
                credentials: 'include'
            })
                .then((response) => {
                    if (response.status !== 200) {
                        return {};
                    } else {
                        return response.json();
                    }
                })
                .catch((error) => {
                    console.log(error);
                    return {};
                });

            setUser(data);
        };

        fetchData();
    }, []);

    function logOut() {
        fetch(`${process.env.NEXT_PUBLIC_BACKEND_BASE}/api/logout`, {
            method: 'POST',
            credentials: 'include'
        })
            .then((response) => {
                if (response.status === 200) {
                    setUser({});
                }
                return response.json();
            })
            .catch((error) => {
                console.log(error);
            });
    }

    return (
        <>
            {user.user ? (
                <p>
                    logged in as <a href="/user/">{user.user.name}</a>
                    <button onClick={logOut}>log out</button>
                </p>
            ) : (
                <a href="/login">log in</a>
            )}
        </>
    );
}

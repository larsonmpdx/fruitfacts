import * as React from 'react';
import Link from '../components/Link';

export async function getStaticProps(context) {
    var fs = require('fs');
    const path = require('path');

    const icons_dir = path.join(process.cwd(), 'public', 'fruit_icons');
    return {
        props: {
            icons: fs.readdirSync(icons_dir)
        }
    };
}

// next.js advises keeping logic client-side in 404s so we can limit server load. ok?
export default function Custom404(props) {
    const [icon, setIcon] = React.useState();
    const [fact, setFact] = React.useState();
    React.useEffect(() => {
        setIcon(props.icons[Math.floor(Math.random() * props.icons.length)]);

        const fetchData = async () => {
            const fact = await fetch(`${process.env.NEXT_PUBLIC_BACKEND_BASE}/api/fact`)
                .then((response) => {
                    if (response.status !== 200) {
                        return;
                    }
                    return response.json();
                })
                .catch((error) => {
                    console.log(error);
                    return;
                });

            setFact(fact);
        };

        fetchData();
    }, []);

    return (
        <center>
            {fact?.fact && (
                <p>
                    {fact.fact}
                    <a href={` ${fact.reference}`}>[ref]</a>
                </p>
            )}
            {icon && (
                <Link href={`/plants/${icon.substr(0, icon.indexOf('.'))}`}>
                    <img src={'/fruit_icons/' + icon} height="100" />
                </Link>
            )}
            <h1>404 - Page Not Found</h1>
        </center>
    );
}

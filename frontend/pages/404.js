import * as React from 'react';
import Link from 'next/link';

export async function getStaticProps() {
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
    <div className="grid place-items-center">
      <article className="prose">
        <div className="grid place-items-center">
          <p>
            <b>Fact:</b> This is a 404 page
          </p>
          {fact?.fact && (
            <p>
              <b>Fact:</b> {`${fact.fact} `}
              <a href={` ${fact.reference}`}>[ref]</a>
            </p>
          )}
          {icon && (
            <Link href={`/plants/${icon.substr(0, icon.indexOf('.'))}`}>
              <a>
                <img className="h-48 w-48 object-contain" src={'/fruit_icons/' + icon} />
              </a>
            </Link>
          )}
        </div>
      </article>
    </div>
  );
}

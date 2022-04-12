import Link from 'next/link';
import Head from 'next/head';
import Button from '../../../components/buttonLink';

export async function getServerSideProps(context) {
  const { type, page } = context.query;
  let pageNum = parseInt(page);
  if (isNaN(pageNum)) {
    pageNum = 1;
  }

  const plants = await fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_BASE}/api/plants/${type}/?page=${pageNum - 1}` // todo - perPage isn't in this API yet, add it when the backend gets it
  )
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
  return {
    props: {
      plants: plants?.plants || [],
      last_page: plants?.last_page + 1 || 1,
      type,
      pageNum
    }
  };
}

export default function Home({ plants, last_page, type, pageNum }) {
  return (
    <>
      <Head>
        <title>{`${type} page ${pageNum}/${last_page}`}</title>
      </Head>
      <article className="prose m-5">
        <h2>
          {type} Page {pageNum}/{last_page}
        </h2>

        <Button href={`/plants/${type}?page=1`} enabled={pageNum != 1} label="first" />
        <Button
          href={`/plants/${type}?page=${parseInt(pageNum) - 1}`}
          enabled={pageNum > 1}
          label="previous"
        />
        <Button
          href={`/plants/${type}?page=${parseInt(pageNum) + 1}`}
          enabled={pageNum < last_page}
          label="next"
        />
        <Button
          href={`/plants/${type}?page=${parseInt(last_page)}`}
          enabled={pageNum != last_page}
          label="last"
        />

        <ul className="list-none">
          {plants.map((item, index) => (
            <>
              <li key={index}>
                <img
                  className="my-0 mx-2 inline h-6 w-6 object-contain"
                  src={'/fruit_icons/' + item.type + '.svg'}
                />
                <Link
                  href={`/plant/${encodeURIComponent(item.type)}/${encodeURIComponent(item.name)}`}
                >
                  {item.name + ' ' + item.type}
                </Link>
                {item.marketing_name && <> (marketed under the {item.marketing_name} brand)</>}
              </li>
            </>
          ))}
        </ul>
      </article>
    </>
  );
}

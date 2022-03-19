import Link from 'next/link';
import Head from 'next/head';
import Button from '../../components/button';
import { formatPatentDate } from '../../components/functions';

export async function getServerSideProps(context) {
  const { page } = context.query;
  let pageNum = parseInt(page);
  if (isNaN(pageNum)) {
    pageNum = 0;
  }

  const patent_info = await fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_BASE}/api/patents?perPage=50&page=${pageNum}`
  )
    .then((response) => {
      if (response.status !== 200) {
        return {};
      }
      return response.json();
    })
    .catch((error) => {
      console.log(error);
      return {};
    });

  return {
    props: {
      patent_info,
      pageNum
    }
  };
}

export default function Home({ patent_info, pageNum }) {
  return (
    <>
      <Head>
        <title>{`Patents page ${pageNum}`}</title>
      </Head>
      <article className="prose m-5">
        <h2>Patents Page {pageNum}</h2>

        <Button
          href={`/patents/${parseInt(patent_info.last_page_past)}`}
          enabled={pageNum > patent_info.last_page_past}
          label="first"
        />
        <Button
          href={`/patents/${parseInt(pageNum) - 1}`}
          enabled={pageNum > patent_info.last_page_past}
          label="previous"
        />
        <Button href="/patents/0" enabled={true} label="current" />
        <Button
          href={`/patents/${parseInt(pageNum) + 1}`}
          enabled={pageNum < patent_info.last_page_future}
          label="next"
        />
        <Button
          href={`/patents/${parseInt(patent_info.last_page_future)}`}
          enabled={pageNum < patent_info.last_page_future}
          label="last"
        />

        <ul className="list-none">
          {patent_info.patents && (
            <>
              {patent_info.patents.map((item, index) => (
                <>
                  <li key={index}>
                    <img
                      className="my-0 mx-2 inline h-6 w-6 object-contain"
                      src={'/fruit_icons/' + item.type + '.svg'}
                    />
                    <Link
                      href={`/plant/${encodeURIComponent(item.type)}/${encodeURIComponent(
                        item.name
                      )}`}
                    >
                      {item.name + ' ' + item.type}
                    </Link>
                    {item.marketing_name && <> (marketed under the {item.marketing_name} brand)</>}{' '}
                    {formatPatentDate(item.uspp_expiration, item.uspp_expiration_estimated)}
                  </li>
                </>
              ))}
            </>
          )}
        </ul>
      </article>
    </>
  );
}

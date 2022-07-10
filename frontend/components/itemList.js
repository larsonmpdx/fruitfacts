import React from 'react';
import Link from 'next/link';
import Button from './buttonLink';
import { formatPatentDate } from './functions';

export default function Home({ name, data }) {
  return (
    <>
      <article className="prose m-5">
        {data?.page && (
        <>
        <h2>
          {name} Page {data.page}
        </h2>

        <Button href={`/patents/1`} enabled={data.page > 1} label="first" />
        <Button
          href={`/patents/${parseInt(data.page) - 1}`}
          enabled={data.page > 1}
          label="previous"
        />
        {data.patentMidpointPage && (
          <Button href={`/patents/${data.patentMidpointPage}`} enabled={true} label="current" />
        )}
        <Button
          href={`/patents/${parseInt(data.page) + 1}`}
          enabled={data.page < parseInt(data.lastPage)}
          label="next"
        />
        <Button
          href={`/patents/${parseInt(data.lastPage)}`}
          enabled={data.page < parseInt(data.lastPage)}
          label="last"
        />
        </>
        )}
        <ul className="list-none">
          {data?.basePlants && (
            <>
              {data.basePlants.map((item, index) => (
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
                    {data.query.patents && (
                      <>
                        ({formatPatentDate(item.uspp_expiration, item.uspp_expiration_estimated)})
                      </>
                    )}
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

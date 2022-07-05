import React from 'react';
import Link from 'next/link';
import Button from './buttonLink';
import { formatPatentDate } from './functions';

export default function Home({name, data, showExpiration}) {
  return (
    <>
    <article className="prose m-5">
    <h2>{name} Page {data.page}</h2>

    <Button
      href={`/patents/0`}
      enabled={data.page > 0}
      label="first"
    />
    <Button
      href={`/patents/${parseInt(data.page) - 1}`}
      enabled={data.page > 0}
      label="previous"
    />
    <Button href="/patents/0" enabled={true} label="current" />
    <Button
      href={`/patents/${parseInt(data.page) + 1}`}
      enabled={data.page < 10}
      label="next"
    />
    <Button
      href={`/patents/10`}
      enabled={data.page < 10}
      label="last"
    />

    <ul className="list-none">
      {data.base_plants && (
        <>
          {data.base_plants.map((item, index) => (
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
                {showExpiration && <>(
                  {formatPatentDate(item.uspp_expiration, item.uspp_expiration_estimated)}
                )</>}   
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

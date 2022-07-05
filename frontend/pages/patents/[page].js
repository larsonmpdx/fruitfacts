import React from 'react';
import Head from 'next/head';
import ItemList from '../../components/itemList';

export async function getServerSideProps(context) {
  let errorMessage = null;
  const { page } = context.query;
  let pageNum = parseInt(page);
  if (isNaN(pageNum)) {
    pageNum = 0;
  }

  const data = await fetch(
    `${
      process.env.NEXT_PUBLIC_BACKEND_BASE
    }/api/search?searchType=base&perPage=50&patents=true&orderBy=patent_expiration&page=${pageNum}`
  )
    .then((response) => {
      if (response.status !== 200) {
        errorMessage = "can't reach backend";
        return {};
      }
      return response.json();
    })
    .catch((error) => {
      errorMessage = `can't reach backend: ${error.message}`;
      console.log(error);
      return {};
    });

  return {
    props: {
      data,
      pageNum,
      errorMessage
    }
  };
}

export default function Home({
  data,
  pageNum,
  errorMessage,
  setErrorMessage,
  setContributingLinks
}) {
  React.useEffect(() => {
    setContributingLinks([
      { link: `/frontend/pages/patents/[page].js`, description: `patents/[page].js` }
    ]);
  }, []);

  setErrorMessage(errorMessage);
  return (
    <>
      <Head>
        <title>{`Patents page ${pageNum}`}</title>
      </Head>
      <ItemList data={data}></ItemList>
    </>
  );
}

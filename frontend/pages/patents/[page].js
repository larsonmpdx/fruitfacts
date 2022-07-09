import React from 'react';
import Head from 'next/head';
import ItemList from '../../components/itemList';
import { useRouter } from 'next/router';

export async function getServerSideProps(context) {

  let errorMessage = null;
  const { page } = context.query;
  let pageNum = parseInt(page);
  if (isNaN(pageNum)) {
    pageNum = 0;
  }

  const data = await fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_BASE}/api/search?searchType=base&perPage=50&patents=true&orderBy=patent_expiration&page=${pageNum}`
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
      errorMessage,
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

  const router = useRouter();

  // get defaults from query - see struct in search.rs
  let querySearchType = router.query.searchType || "patent_expiration";
  let querySearch = router.query.search || undefined;
  let queryName = router.query.name || undefined;
  let queryPatents = router.query.patents || "true";
  let queryType = router.query.type || undefined; // apple, pear, etc.
  let queryPage = router.query.page || "1";
  let queryPerPage = router.query.perPage || "50";
  let queryOrderBy = router.query.orderBy || "patent_expiration";
  let queryOrder = router.query.order || undefined;
  let queryRelativeHarvestMin = router.query.relativeHarvestMin || undefined;
  let queryRelativeHarvestMax = router.query.relativeHarvestMax || undefined;

  let queryCollectionID = router.query.collectionID || undefined;
  let queryCollectionPath = router.query.collectionPath || undefined;

  let queryNotorietyMin = router.query.notorietyMin || undefined;

  // todo
  let queryDistance = router.query.distance || undefined;
  let queryFrom = router.query.from || undefined;

  // create backend query string from the above query params (exclude undefined stuff)
  // store it so we can de-duplicate backend queries
  // update backend stuff on query string change
  // rewrite frontend query string on change
  
  




  
  
  


  React.useEffect(() => {
    setContributingLinks([
      { link: `/frontend/pages/patents/[page].js`, description: `patents/[page].js` }
    ]);
  }, []);

  setErrorMessage(errorMessage);

  const [orderByValue, setOrderByValue] = React.useState(queryOrderBy);
  const handleOrderByChange = () => {
    setOrderByValue(!open);
  };


  return (
    <>
      <Head>
        <title>{`Patents page ${pageNum}`}</title>
      </Head>
      <select id="orderBy"
      value={orderByValue}
      onChange={handleOrderByChange}

      class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500">
        <option selected>sort by</option>
        <option value="name_then_type">name then type</option>
        <option value="type_then_name">type then name</option>
        <option value="patent_expiration">patent expiration</option>
        <option value="harvest_time">harvest time</option>
        <option value="search_quality">search quality</option>
      </select>

      <select id="perPage"
      
      
      class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500">
        <option selected value="50">50 per page</option>
        <option value="200">200 per page</option>
        <option value="unlimited">unlimited</option>
      </select>
      <ItemList data={data}></ItemList>
    </>
  );
}

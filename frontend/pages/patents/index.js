import React from 'react';
import Head from 'next/head';
import ItemList from '../../components/itemList';
import { useRouter } from 'next/router';
import * as qs from 'qs'

const nullIfEmptyQuote = (value) => {
  if(value == "") {
    return null;
  }
  return value;
};

export default function Home({
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

  const router = useRouter();

  // get defaults from query - see struct in search.rs
  let querySearchType = router.query.searchType || "base";
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

  const [queryObject, setQueryObject] = React.useState(
    {
      searchType: querySearchType,
      search : querySearch,
      name : queryName,
      patents : queryPatents,
      type : queryType,
      page : queryPage,
      perPage : queryPerPage,
      orderBy : queryOrderBy,
      order : queryOrder, // asc/desc
      relativeHarvestMin : queryRelativeHarvestMin,
      relativeHarvestMax : queryRelativeHarvestMax,
      collectionID : queryCollectionID,
      collectionPath : queryCollectionPath,
      notorietyMin : queryNotorietyMin,
      distance : queryDistance,
      from : queryFrom,
    }
  );

  // create backend query string from the above query params (exclude undefined stuff)
  // store it so we can de-duplicate backend queries
  // update backend stuff on query string change
  // rewrite frontend query string on change

  // set frontend query string for history/bookmarking
  const [data, setData] = React.useState({});
  React.useEffect(() => {
    // without null and undefined
    let queryCleaned = Object.fromEntries(Object.entries(queryObject).filter(([_, v]) => v != null));
    const queryString = qs.stringify(queryCleaned);


    const fetchData = async () => {
      const data = await fetch(`${process.env.NEXT_PUBLIC_BACKEND_BASE}/api/search?` + queryString)
        .then((response) => {
          if (response.status !== 200) {
            setErrorMessage("can't reach the backend");
            return;
          }
          return response.json();
        })
        .catch((error) => {
          setErrorMessage(`can't reach backend: ${error.message}`);
          console.log(error);
          return;
        });

      setData(data);
    };
    fetchData();

    
    
    router.query = queryString; // set frontend query string
    router.push(router);
  }, [queryObject]);


  const handleOrderByChange = (event) => {
    setQueryObject({...queryObject, orderBy: nullIfEmptyQuote(event.target.value)});
  };

  return (
    <>
      <Head>
        <title>{`Patents page ${pageNum}`}</title>
      </Head>
      <select id="orderBy"
      value={queryObject.orderBy}
      onChange={handleOrderByChange}

      class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500">
        <option value="" selected>sort by</option>
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

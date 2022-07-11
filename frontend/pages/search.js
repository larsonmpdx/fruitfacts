import * as qs from 'qs';
import React from 'react';
import Head from 'next/head';
import { useRouter } from 'next/router';
import TextField from '@mui/material/TextField';
import Autocomplete from '@mui/material/Autocomplete';
import ItemList from '../components/itemList';
import Button from '../components/button';

// todo - get types somehow. we aren't allowed to get them from getStaticProps() because we're using getSSP() - sad!
//import { getTypesForAutocomplete } from '../components/getTypes';
/*
export async function getStaticProps() {
  const types = getTypesForAutocomplete();

  return {
    props: {
      types
    }
  };
}
*/
export async function getServerSideProps(context) {
  let queryCleaned = Object.fromEntries(
    Object.entries(context.query).filter(([_, v]) => v != null)
  );
  const queryString = qs.stringify(queryCleaned);

  const fetchData = async () => {
    return await fetch(`${process.env.NEXT_PUBLIC_BACKEND_BASE}/api/search?` + queryString)
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
  };

  const data = await fetchData();

  return {
    props: {
      data
    }
  };
}

const nullIfEmptyQuote = (value) => {
  if (value == '') {
    return null;
  }
  return value;
};

export default function Home({
  data,
  types,
  pageNum,
  errorMessage,
  setErrorMessage,
  setContributingLinks
}) {
  types = [];
  React.useEffect(() => {
    setContributingLinks([
      { link: `/frontend/pages/patents/[page].js`, description: `patents/[page].js` }
    ]);
  }, []);
  setErrorMessage(errorMessage);

  const router = useRouter();

  console.dir(router.asPath);
  const query = qs.parse(router.asPath.split(/\?/)[1]);

  // get defaults from query - see struct in search.rs
  let querySearchType = query.searchType || 'base';
  let querySearch = query.search || undefined;
  let queryName = query.name || undefined;
  let queryPatents = query.patents == 'true' || true;
  let queryType = query.type || undefined; // apple, pear, etc.
  let queryPage = query.page || '1';
  let queryPerPage = query.perPage || '50';
  let queryOrderBy = query.orderBy || 'patent_expiration';
  let queryOrder = query.order || undefined;
  let queryRelativeHarvestMin = query.relativeHarvestMin || undefined;
  let queryRelativeHarvestMax = query.relativeHarvestMax || undefined;

  let queryCollectionID = query.collectionID || undefined;
  let queryCollectionPath = query.collectionPath || undefined;

  let queryNotorietyMin = query.notorietyMin || undefined;

  let queryDistance = query.distance || undefined; // todo
  let queryFrom = query.from || undefined; // todo

  const [queryObject, setQueryObject] = React.useState({
    searchType: querySearchType,
    search: querySearch,
    name: queryName,
    patents: queryPatents,
    type: queryType,
    page: queryPage,
    perPage: queryPerPage,
    orderBy: queryOrderBy,
    order: queryOrder, // asc/desc
    relativeHarvestMin: queryRelativeHarvestMin,
    relativeHarvestMax: queryRelativeHarvestMax,
    collectionID: queryCollectionID,
    collectionPath: queryCollectionPath,
    notorietyMin: queryNotorietyMin,
    distance: queryDistance,
    from: queryFrom
  });

  // create backend query string from the above query params (exclude undefined stuff)
  // store it so we can de-duplicate backend queries
  // update backend stuff on query string change
  // rewrite frontend query string on change

  // set frontend query string for history/bookmarking
  //const [data, setData] = React.useState({});
  React.useEffect(() => {
    // without null and undefined
    let queryCleaned = Object.fromEntries(
      Object.entries(queryObject).filter(([_, v]) => v != null)
    );
    const queryString = qs.stringify(queryCleaned);

    router.query = queryString; // set frontend query string
    router.push(router);
  }, [queryObject]);

  const handleChangePageButton = (newPage) => {
    setQueryObject({ ...queryObject, page: newPage });
  };

  const handleOrderByChange = (event) => {
    setQueryObject({ ...queryObject, orderBy: nullIfEmptyQuote(event.target.value) });
  };

  const handlePerPageChange = (event) => {
    const perPage = nullIfEmptyQuote(event.target.value);
    // only set if if our new perPage is different than previous. also switch to page 1 if we had a change
    if (perPage != queryObject) {
      setQueryObject({ ...queryObject, perPage, page: '1' });
    }
  };

  const handlePatentsChange = (event) => {
    const checked = event.target.checked;
    if (checked) {
      setQueryObject({ ...queryObject, patents: true });
    } else {
      setQueryObject({ ...queryObject, patents: null });
    }
  };

  const handleTypeChange = (type) => {
    console.log(type);
    setQueryObject({ ...queryObject, type });
  };

  return (
    <>
      <Head>
        <title>{`Patents page ${pageNum}`}</title>
      </Head>
      <select
        id="orderBy"
        value={queryObject.orderBy}
        onChange={handleOrderByChange}
        class="block w-full rounded-lg border border-gray-300 bg-gray-50 p-2.5 text-sm text-gray-900 focus:border-blue-500 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white dark:placeholder-gray-400 dark:focus:border-blue-500 dark:focus:ring-blue-500"
      >
        <option value="name_then_type">name then type</option>
        <option value="type_then_name">type then name</option>
        <option value="patent_expiration">patent expiration</option>
        <option value="harvest_time">harvest time</option>
        <option value="search_quality">search quality</option>
      </select>

      <select
        id="perPage"
        value={queryObject.perPage}
        onChange={handlePerPageChange}
        class="block w-full rounded-lg border border-gray-300 bg-gray-50 p-2.5 text-sm text-gray-900 focus:border-blue-500 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white dark:placeholder-gray-400 dark:focus:border-blue-500 dark:focus:ring-blue-500"
      >
        <option selected value="50">
          50 per page
        </option>
        <option value="200">200 per page</option>
        <option value="">unlimited</option>
      </select>

      <label>
        <input
          type="checkbox"
          checked={queryObject.patents == true}
          onClick={handlePatentsChange}
        />
        patented only
      </label>

      <Autocomplete
        disablePortal
        id="combo-box-demo"
        options={types}
        getOptionLabel={(option) => option.name}
        sx={{ width: 300 }}
        renderInput={(params) => <TextField {...params} label="Fruit Type" />}
        onChange={(event, option) => {
          handleTypeChange(option?.name);
        }}
      />

      {data?.page && (
        <>
          <h2>Page {data.page}</h2>
          <Button
            onClick={() => {
              handleChangePageButton(1);
            }}
            enabled={data.page > 1}
            label="first"
          />
          <Button
            onClick={() => {
              handleChangePageButton(parseInt(data.page) - 1);
            }}
            enabled={data.page > 1}
            label="previous"
          />
          {data.patentMidpointPage && (
            <Button
              onClick={() => {
                handleChangePageButton(data.patentMidpointPage);
              }}
              enabled={true}
              label="current"
            />
          )}
          <Button
            onClick={() => {
              handleChangePageButton(parseInt(data.page) + 1);
            }}
            enabled={data.page < parseInt(data.lastPage)}
            label="next"
          />
          <Button
            onClick={() => {
              handleChangePageButton(parseInt(data.lastPage));
            }}
            enabled={data.page < parseInt(data.lastPage)}
            label="last"
          />
        </>
      )}

      <ItemList data={data}></ItemList>
    </>
  );
}

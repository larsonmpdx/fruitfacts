
struct year_and_patent_number {
    year: i32,
    patent_number: i32,
}

// for varieties with no release year listed but a patent number given, guess at it based on their US patent number
pub fn USPP_number_to_release_year(uspp_number_input: i32) -> i32 {
    // https://www.uspto.gov/web/offices/ac/ido/oeip/taf/issuyear.htm
    const patent_numbers_to_year: [year_and_patent_number; 91] = [
        year_and_patent_number {
            year: 1931,
            patent_number: 1,
        },
        year_and_patent_number {
            year: 1932,
            patent_number: 6,
        },
        year_and_patent_number {
            year: 1933,
            patent_number: 52,
        },
        year_and_patent_number {
            year: 1934,
            patent_number: 85,
        },
        year_and_patent_number {
            year: 1935,
            patent_number: 117,
        },
        year_and_patent_number {
            year: 1936,
            patent_number: 162,
        },
        year_and_patent_number {
            year: 1937,
            patent_number: 211,
        },
        year_and_patent_number {
            year: 1938,
            patent_number: 266,
        },
        year_and_patent_number {
            year: 1939,
            patent_number: 307,
        },
        year_and_patent_number {
            year: 1940,
            patent_number: 352,
        },
        year_and_patent_number {
            year: 1941,
            patent_number: 437,
        },
        year_and_patent_number {
            year: 1942,
            patent_number: 499,
        },
        year_and_patent_number {
            year: 1943,
            patent_number: 564,
        },
        year_and_patent_number {
            year: 1944,
            patent_number: 611,
        },
        year_and_patent_number {
            year: 1945,
            patent_number: 649,
        },
        year_and_patent_number {
            year: 1946,
            patent_number: 666,
        },
        year_and_patent_number {
            year: 1947,
            patent_number: 722,
        },
        year_and_patent_number {
            year: 1948,
            patent_number: 774,
        },
        year_and_patent_number {
            year: 1949,
            patent_number: 818,
        },
        year_and_patent_number {
            year: 1950,
            patent_number: 911,
        },
        year_and_patent_number {
            year: 1951,
            patent_number: 1001,
        },
        year_and_patent_number {
            year: 1952,
            patent_number: 1059,
        },
        year_and_patent_number {
            year: 1953,
            patent_number: 1160,
        },
        year_and_patent_number {
            year: 1954,
            patent_number: 1238,
        },
        year_and_patent_number {
            year: 1955,
            patent_number: 1339,
        },
        year_and_patent_number {
            year: 1956,
            patent_number: 1442,
        },
        year_and_patent_number {
            year: 1957,
            patent_number: 1543,
        },
        year_and_patent_number {
            year: 1958,
            patent_number: 1672,
        },
        year_and_patent_number {
            year: 1959,
            patent_number: 1792,
        },
        year_and_patent_number {
            year: 1960,
            patent_number: 1893,
        },
        year_and_patent_number {
            year: 1961,
            patent_number: 2009,
        },
        year_and_patent_number {
            year: 1962,
            patent_number: 2117,
        },
        year_and_patent_number {
            year: 1963,
            patent_number: 2208,
        },
        year_and_patent_number {
            year: 1964,
            patent_number: 2337,
        },
        year_and_patent_number {
            year: 1965,
            patent_number: 2465,
        },
        year_and_patent_number {
            year: 1966,
            patent_number: 2585,
        },
        year_and_patent_number {
            year: 1967,
            patent_number: 2699,
        },
        year_and_patent_number {
            year: 1968,
            patent_number: 2784,
        },
        year_and_patent_number {
            year: 1969,
            patent_number: 2856,
        },
        year_and_patent_number {
            year: 1970,
            patent_number: 2959,
        },
        year_and_patent_number {
            year: 1971,
            patent_number: 3011,
        },
        year_and_patent_number {
            year: 1972,
            patent_number: 3063,
        },
        year_and_patent_number {
            year: 1973,
            patent_number: 3281,
        },
        year_and_patent_number {
            year: 1974,
            patent_number: 3413,
        },
        year_and_patent_number {
            year: 1975,
            patent_number: 3674,
        },
        year_and_patent_number {
            year: 1976,
            patent_number: 3824,
        },
        year_and_patent_number {
            year: 1977,
            patent_number: 4001,
        },
        year_and_patent_number {
            year: 1978,
            patent_number: 4174,
        },
        year_and_patent_number {
            year: 1979,
            patent_number: 4360,
        },
        year_and_patent_number {
            year: 1980,
            patent_number: 4491,
        },
        year_and_patent_number {
            year: 1981,
            patent_number: 4612,
        },
        year_and_patent_number {
            year: 1982,
            patent_number: 4796,
        },
        year_and_patent_number {
            year: 1983,
            patent_number: 4970,
        },
        year_and_patent_number {
            year: 1984,
            patent_number: 5168,
        },
        year_and_patent_number {
            year: 1985,
            patent_number: 5380,
        },
        year_and_patent_number {
            year: 1986,
            patent_number: 5622,
        },
        year_and_patent_number {
            year: 1987,
            patent_number: 5846,
        },
        year_and_patent_number {
            year: 1988,
            patent_number: 6075,
        },
        year_and_patent_number {
            year: 1989,
            patent_number: 6501,
        },
        year_and_patent_number {
            year: 1990,
            patent_number: 7089,
        },
        year_and_patent_number {
            year: 1991,
            patent_number: 7408,
        },
        year_and_patent_number {
            year: 1992,
            patent_number: 7761,
        },
        year_and_patent_number {
            year: 1993,
            patent_number: 8082,
        },
        year_and_patent_number {
            year: 1994,
            patent_number: 8527,
        },
        year_and_patent_number {
            year: 1995,
            patent_number: 9026,
        },
        year_and_patent_number {
            year: 1996,
            patent_number: 9413,
        },
        year_and_patent_number {
            year: 1997,
            patent_number: 9776,
        },
        year_and_patent_number {
            year: 1998,
            patent_number: 10172,
        },
        year_and_patent_number {
            year: 1999,
            patent_number: 10743,
        },
        year_and_patent_number {
            year: 2000,
            patent_number: 11169,
        },
        year_and_patent_number {
            year: 2001,
            patent_number: 11728,
        },
        year_and_patent_number {
            year: 2002,
            patent_number: 12314,
        },
        year_and_patent_number {
            year: 2003,
            patent_number: 13447,
        },
        year_and_patent_number {
            year: 2004,
            patent_number: 14441,
        },
        year_and_patent_number {
            year: 2005,
            patent_number: 15460,
        },
        year_and_patent_number {
            year: 2006,
            patent_number: 16176,
        },
        year_and_patent_number {
            year: 2007,
            patent_number: 17326,
        },
        year_and_patent_number {
            year: 2008,
            patent_number: 18373,
        },
        year_and_patent_number {
            year: 2009,
            patent_number: 19613,
        },
        year_and_patent_number {
            year: 2010,
            patent_number: 20622,
        },
        year_and_patent_number {
            year: 2011,
            patent_number: 21604,
        },
        year_and_patent_number {
            year: 2012,
            patent_number: 22428,
        },
        year_and_patent_number {
            year: 2013,
            patent_number: 23288,
        },
        year_and_patent_number {
            year: 2014,
            patent_number: 24135,
        },
        year_and_patent_number {
            year: 2015,
            patent_number: 25207,
        },
        year_and_patent_number {
            year: 2016,
            patent_number: 26285,
        },
        year_and_patent_number {
            year: 2017,
            patent_number: 27520,
        },
        year_and_patent_number {
            year: 2018,
            patent_number: 28831,
        },
        year_and_patent_number {
            year: 2019,
            patent_number: 30040,
        },
        year_and_patent_number {
            year: 2020,
            patent_number: 31315,
        },
        year_and_patent_number {
            year: 2021,
            patent_number: 32717,
        },
    ];

    let mut previous_year = patent_numbers_to_year[0].year;
    for element in patent_numbers_to_year {
        if element.patent_number > uspp_number_input {
            break;
        }

        previous_year = element.year;
    }

    previous_year
}
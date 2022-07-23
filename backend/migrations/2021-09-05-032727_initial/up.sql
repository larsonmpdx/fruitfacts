CREATE TABLE base_plants (
  id INTEGER PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  name_fts TEXT NOT NULL, -- for full text search, without special characters
  type TEXT NOT NULL,

  notoriety_score REAL,
  notoriety_score_explanation TEXT,
  number_of_references INTEGER NOT NULL,
  notoriety_highest_collection_score REAL,
  notoriety_highest_collection_score_id INTEGER,

  -- these fields don't go into collections because they're part of our ground truth
  aka TEXT, -- comma separated
  aka_fts TEXT, -- for full text search, without special characters. comma separated
  marketing_name TEXT, -- for any AKA entries that have (tm) or (r) in them, flag them and fill this column. they need special handing because of the confusion around variety name vs. marketing name
  description TEXT,
  uspp_number TEXT, -- text so we can store odd patent numbers - sometimes plants get non-plant patents
  uspp_expiration BigInt, -- unix seconds. bigint to get diesel to match this to i64 for the 2038 problem
  uspp_expiration_estimated INTEGER, -- bool, was the expiration year guessed-at based on the patent year?
  release_year INTEGER,
  release_year_note TEXT, -- in case the release year is guessed at from a patent number, put a note here
  released_by TEXT,
  release_collection_id INTEGER,
  
  ignore_unless_in_others INTEGER NOT NULL, -- bool, was this plant generated only by a low-information collection? if so remove it after import
  s_allele TEXT,

  -- calculated from all of the other data
  harvest_relative INTEGER, -- + or - days vs the chosen reference plant for this type
  harvest_relative_to TEXT, -- "Redhaven" for peaches for example
  harvest_relative_to_type TEXT, -- "Peach" for nectarines. should be same type for everything else
  harvest_relative_explanation TEXT, -- include weights etc.

  UNIQUE(uspp_number)
  UNIQUE(name_fts, type) --combo of these columns must be unique.  example: name "Co-op 32" type "Apple"
  UNIQUE(name, type)
);

-- fts: see https://www.sqlite.org/fts5.html
-- trigram needs sqlite 3.34.0+
-- open the amalgamated sqlite3.c to check version
-- SQL LIKE queries are an alternative to this, but probably slower
CREATE VIRTUAL TABLE fts_base_plants USING fts5(name_fts, aka_fts, content='base_plants', content_rowid='id', tokenize='trigram');

-- Triggers to keep the FTS index up to date
CREATE TRIGGER base_plants_ai AFTER INSERT ON base_plants BEGIN
  INSERT INTO fts_base_plants(rowid, name_fts, aka_fts) VALUES (new.id, new.name_fts, new.aka_fts);
END;
CREATE TRIGGER base_plants_ad AFTER DELETE ON base_plants BEGIN
  INSERT INTO fts_base_plants(fts_base_plants, rowid, name_fts, aka_fts) VALUES('delete', old.id, old.name_fts, old.aka_fts);
END;
CREATE TRIGGER base_plants_au AFTER UPDATE ON base_plants BEGIN
  INSERT INTO fts_base_plants(fts_base_plants, rowid, name_fts, aka_fts) VALUES('delete', old.id, old.name_fts, old.aka_fts);
  INSERT INTO fts_base_plants(rowid, name_fts, aka_fts) VALUES (new.id, new.name_fts, new.aka_fts);
END;


CREATE TABLE plant_types (
  id INTEGER PRIMARY KEY NOT NULL,
  group_name TEXT NOT NULL,
  name TEXT NOT NULL,
  latin_name TEXT,
  UNIQUE(name)
);

CREATE TABLE users (
  id INTEGER PRIMARY KEY NOT NULL,

  -- profile info
  name TEXT NOT NULL,
  location_name TEXT,
  latitude DOUBLE,
  longitude DOUBLE,

  UNIQUE(name)
);

-- in case a user has multiple oauth methods associated to their account
CREATE TABLE user_oauth_entries (
  id INTEGER PRIMARY KEY NOT NULL,

  user_id INTEGER NOT NULL,
  unique_id TEXT NOT NULL, -- google email ID, or other oauth ID which won't change. prepended with the provider's ID like "google:1234..."
  oauth_info TEXT, -- json which might be different for each oauth provider

  UNIQUE(unique_id)
);

CREATE TABLE user_sessions (
  id INTEGER PRIMARY KEY NOT NULL,

  user_id INTEGER NOT NULL,
  session_value TEXT NOT NULL,
  created BigInt NOT NULL, -- unix seconds. bigint to get diesel to match this to i64 for the 2038 problem

  UNIQUE(session_value)
);

CREATE TABLE collections (
  id INTEGER PRIMARY KEY NOT NULL,

  git_edit_time BigInt, -- unix seconds. bigint to get diesel to match this to i64 for the 2038 problem

  path TEXT NOT NULL, -- directory that we found this in, like "Oregon" or "Oregon/Willamette Valley"
  filename TEXT NOT NULL,

  notoriety_type TEXT NOT NULL,
  notoriety_score REAL NOT NULL,
  notoriety_score_explanation TEXT NOT NULL,
  harvest_time_devalue_factor REAL,
  ignore_for_nearby_searches INTEGER NOT NULL, -- bool, is this collection maybe a dictionary that has an arbitrary location set (like the US patent collection being set to washington, DC)? if so we don't want to find it in nearby locations

  title TEXT,
  author TEXT,
  description TEXT,
  url TEXT,
  published TEXT,
  reviewed TEXT,
  accessed TEXT,
  needs_help INTEGER NOT NULL, -- bool

  UNIQUE(path, filename)
);

CREATE TABLE locations (
  id INTEGER PRIMARY KEY NOT NULL,
  location_number INTEGER NOT NULL,
  collection_id INTEGER NOT NULL,

  location_name TEXT,
  latitude DOUBLE, -- todo these can probably be "not null"
  longitude DOUBLE,

  -- copied from the collection to save a lookup
  notoriety_score REAL NOT NULL,
  collection_path TEXT,
  collection_filename TEXT,
  collection_title TEXT,
  ignore_for_nearby_searches INTEGER NOT NULL,

  -- user location specific
  description TEXT
);

CREATE TABLE collection_items (
  id INTEGER PRIMARY KEY NOT NULL,
  
  collection_id INTEGER NOT NULL,
  location_id INTEGER, -- this can be unset for cases where there's a random list of varieties not attached to a location
  location_number INTEGER NOT NULL, -- 1,2,3,4. if set to zero then it's one of the non-location varieties. lets us simplify the front end and filter for "location 1" without enumerating them first
  user_id INTEGER, -- unset if these are from built-in collections

  path_and_filename TEXT, -- for website display/navigation instead of looking it up
  marketing_name TEXT, -- copied back from base plants for quicker display

  name TEXT NOT NULL,
  type TEXT NOT NULL,

  -- a shortcut to the base item's ID so we can simplify our searches
  -- this should be updated whenever the database is rebuilt or the name+type pair changes
  base_plant_id INTEGER,

  category TEXT, -- like "low chill"
  category_description TEXT,

  disease_resistance TEXT, -- json string like "{"FB":"moderate","PM":"high"}"
  chill TEXT,
  s_allele TEXT,

  -- the actual unique data from the imported guide
  description TEXT,
  harvest_text TEXT, -- to store the original text like "Sep 25" before parsing
  harvest_relative TEXT, --used for things like "redhaven+5"
  harvest_start INTEGER, --ordinal (day of the year)
  harvest_end INTEGER,

  -- pretty much only for figs with breba+main crop
  harvest_start_2 INTEGER,
  harvest_end_2 INTEGER,

  -- these are set after import either by parsing harvest_relative text
  -- or by using a delta from another variety with an already-calculated relative harvest
  calc_harvest_relative INTEGER,
  calc_harvest_relative_to TEXT, -- "Redhaven" for peaches for example
  calc_harvest_relative_to_type TEXT, -- "Peach" for nectarines. should be same type for everything else
  calc_harvest_relative_round DOUBLE, -- 0: directly parsed from harvest_relative text 1: set based on absolute harvest difference to a known variety 2+: successive rounds of this as more varieties get filled in
  calc_harvest_relative_explanation TEXT, -- which plant and value was referenced?

  UNIQUE(collection_id, location_id, name, type) --combo of these columns must be unique
);

CREATE TABLE facts (
  id INTEGER PRIMARY KEY NOT NULL,

  contributor TEXT NOT NULL,
  fact TEXT NOT NULL,
  reference TEXT NOT NULL
);

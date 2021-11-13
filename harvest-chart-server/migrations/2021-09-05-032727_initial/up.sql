CREATE TABLE base_plants (
  plant_id INTEGER PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  name_fts TEXT NOT NULL, -- for full text search, without special characters
  type TEXT NOT NULL,

  -- these fields don't go into collections because they're part of our ground truth
  aka TEXT, -- comma separated
  aka_fts TEXT, -- for full text search, without special characters. comma separated
  marketing_name TEXT, -- for any AKA entries that have (tm) or (r) in them, flag them and fill this column. they need special handing because of the confusion around variety name vs. marketing name
  description TEXT,
  uspp_number TEXT, -- text so we can store odd patent numbers - sometimes plants get non-plant patents
  uspp_expiration BigInt, -- unix seconds. bigint to get diesel to match this to i64 for the 2038 problem
  release_year INTEGER,
  release_year_note TEXT, -- in case the release year is guessed at from a patent number, put a note here
  released_by TEXT,
  release_collection_id INTEGER,

  UNIQUE(uspp_number)
  UNIQUE(name_fts, type) --combo of these columns must be unique.  example: name "Co-op 32" type "Apple"
  UNIQUE(name, type)
);

CREATE TABLE plant_types (
  plant_type_id INTEGER PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  latin_name TEXT,
  UNIQUE(name)
);

CREATE TABLE users (
  user_id INTEGER PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  UNIQUE(name)
);

CREATE TABLE collections (
  location_id INTEGER PRIMARY KEY NOT NULL,
  collection_id INTEGER NOT NULL,
  user_id INTEGER NOT NULL,

  path TEXT, -- directory that we found this in, like "Oregon" or "Oregon/Willamette Valley"
  filename TEXT,

  title TEXT,
  author TEXT,
  description TEXT,
  url TEXT,
  published TEXT,
  reviewed TEXT,
  accessed TEXT,

  location_name TEXT,
  latitude DOUBLE,
  longitude DOUBLE
);

CREATE TABLE collection_items (
  collection_item_id INTEGER PRIMARY KEY NOT NULL,
  
  collection_title TEXT,
  collection_id INTEGER NOT NULL,
  location_id INTEGER, -- this can be unset for cases where there's a random list of varieties not attached to a location

  -- name+type don't have to exist in base plants so this could be a wholly user-created plant
  name TEXT NOT NULL,
  type TEXT NOT NULL,

  category TEXT, -- like "low chill"
  category_description TEXT,

  disease_resistance TEXT, -- json string like "{"FB":"moderate","PM":"high"}"
  chill TEXT,

  -- the actual unique data from the imported guide
  description TEXT,
  harvest_text TEXT, -- to store the original text like "Sep 25" before parsing
  harvest_relative TEXT, --used for things like "redhaven+5"
  harvest_start INTEGER, --ordinal (day of the year)
  harvest_end INTEGER,
  harvest_start_is_midpoint INTEGER, -- bool: if this is a start-only harvest window, should the window be treated as a midpoint instead of a start when building a window around it?

  -- pretty much only for figs with breba+main crop
  harvest_start_2 INTEGER,
  harvest_end_2 INTEGER,
  harvest_start_2_is_midpoint INTEGER,

  UNIQUE(collection_id, location_id, type, name) --combo of these columns must be unique
);

CREATE TABLE base_plants (
  plant_id INTEGER PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  type TEXT NOT NULL,

  -- these fields don't go into collections because they're part of our ground truth
  aka TEXT, -- comma separated
  aka_fts TEXT, -- for full text search, like aka but with characters like dashes and spaces removed. comma separated
  description TEXT,
  uspp_number INTEGER,
  uspp_expiration TEXT,

  UNIQUE(name, type) --combo of these columns must be unique.  example: name "Pristine" type "Apple"
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

  location TEXT,
  latitude DOUBLE,
  longitude DOUBLE
);

CREATE TABLE collection_items (
  collection_item_id INTEGER PRIMARY KEY NOT NULL,
  location_name TEXT,
  collection_id INTEGER NOT NULL,

  -- name+type don't have to exist in base plants so this could be a wholly user-created plant
  name TEXT NOT NULL,
  type TEXT NOT NULL,

  category TEXT, -- like "low chill"
  category_description TEXT,

  -- the actual unique data from the imported guide
  description TEXT,
  harvest_relative TEXT, --used for things like "redhaven+5"
  harvest_text TEXT, -- to store the original text like "Sep 25" before parsing
  harvest_start INTEGER, --ordinal (day of the year)
  harvest_end INTEGER,

  -- pretty much only for figs with breba+main crop
  harvest_start_2 INTEGER,
  harvest_end_2 INTEGER,

  UNIQUE(location_name, collection_id, name, type) --combo of these columns must be unique
);

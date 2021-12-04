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
  release_year INTEGER,
  release_year_note TEXT, -- in case the release year is guessed at from a patent number, put a note here
  released_by TEXT,
  release_collection_id INTEGER,

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
  name TEXT NOT NULL,
  latin_name TEXT,
  UNIQUE(name)
);

CREATE TABLE users (
  id INTEGER PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  UNIQUE(name)
);

CREATE TABLE collections (
  id INTEGER PRIMARY KEY NOT NULL,
  user_id INTEGER NOT NULL,

  git_edit_time BigInt, -- unix seconds. bigint to get diesel to match this to i64 for the 2038 problem

  path TEXT, -- directory that we found this in, like "Oregon" or "Oregon/Willamette Valley"
  filename TEXT,

  notoriety_type TEXT NOT NULL,
  notoriety_score REAL,
  notoriety_score_explanation TEXT,

  title TEXT,
  author TEXT,
  description TEXT,
  url TEXT,
  published TEXT,
  reviewed TEXT,
  accessed TEXT,

  UNIQUE(path, filename)
);

CREATE TABLE locations (
  id INTEGER PRIMARY KEY NOT NULL,
  collection_id INTEGER NOT NULL,

  location_name TEXT,
  latitude DOUBLE,
  longitude DOUBLE
);

CREATE TABLE collection_items (
  id INTEGER PRIMARY KEY NOT NULL,
  
  collection_id INTEGER NOT NULL,
  location_id INTEGER, -- this can be unset for cases where there's a random list of varieties not attached to a location

  path_and_filename TEXT, -- for website display/navigation instead of looking it up
  marketing_name TEXT, -- copied back from base plants for quicker display

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

  UNIQUE(collection_id, location_id, name, type) --combo of these columns must be unique
);

CREATE TABLE base_plants (
  plant_id INTEGER PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  type TEXT NOT NULL,

  -- remaining columns are copied into collection_items
  description TEXT,
  patent TEXT,
  relative_harvest TEXT, --used for things like "redhaven+5"
  harvest_start INTEGER, --ordinal (day of the year)
  harvest_end INTEGER,

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
  collection_id INTEGER PRIMARY KEY NOT NULL,
  user_id INTEGER NOT NULL,
  name TEXT NOT NULL,

  -- for extension guides
  path TEXT, -- directory that we found this in, like "Oregon" or "Oregon/Willamette Valley"
  title TEXT,
  author TEXT,
  note TEXT,
  url TEXT,
  published TEXT,
  reviewed TEXT,
  accessed TEXT,

  location TEXT,
  latitude REAL,
  longitude REAL,
  UNIQUE(name)
);

CREATE TABLE collection_items (
  collection_item_id INTEGER PRIMARY KEY NOT NULL,
  collection_id INTEGER NOT NULL,

  note TEXT, -- for user notes

  -- name+type don't have to exist in base plants so this could be a wholly user-created plant
  name TEXT NOT NULL,
  type TEXT NOT NULL,

  -- remaining columns copied from base plants, these can be set to override or they can be inherited from the
  -- base plant with matching name+type
  description TEXT,
  patent TEXT,
  relative_harvest TEXT, --used for things like "redhaven+5"
  harvest_start INTEGER, --ordinal (day of the year)
  harvest_end INTEGER,

  FOREIGN KEY (collection_id)
      REFERENCES collections (collection_id),

  UNIQUE(collection_id, name, type) --combo of these columns must be unique
);

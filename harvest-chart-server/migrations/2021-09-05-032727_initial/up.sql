CREATE TABLE base_plants (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  type TEXT NOT NULL,

  -- remaining columns are copied into collection_items
  description TEXT,
  patent TEXT,
  relative_harvest TEXT, --used for things like "redhaven+5"
  harvest_start INTEGER, --ordinal (day of the year)
  harvest_end INTEGER,
  harvest_time_reference TEXT, -- which publication or organization gave the harvest time?

  UNIQUE(name, type) --combo of these columns must be unique.  example: name "Pristine" type "Apple"
);

CREATE TABLE plant_types (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  latin_name TEXT,
  UNIQUE(name)
);

CREATE TABLE users (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  UNIQUE(name)
);

CREATE TABLE collections (
  id INTEGER PRIMARY KEY,
  user_id INTEGER NOT NULL,
  name TEXT NOT NULL,
  UNIQUE(name)
);

CREATE TABLE collection_items (
  id INTEGER PRIMARY KEY,
  collection_id TEXT NOT NULL,

  notes TEXT, -- for user notes

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
  harvest_time_reference TEXT,

  UNIQUE(collection_id, name, type) --combo of these columns must be unique
);

CREATE TABLE base_plants (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  type TEXT NOT NULL,
  description TEXT,
  patent TEXT,
  relative_harvest TEXT, --used for things like "redhaven+5"
  harvest_start INTEGER, --integer type stores dates as unix times
  harvest_end INTEGER,
  UNIQUE(name, type) --combo of these columns must be unique.  example: name "Pristine" type "Apple"
)

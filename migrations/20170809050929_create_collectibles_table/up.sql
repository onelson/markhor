CREATE TABLE collectibles (
  id INTEGER PRIMARY KEY NOT NULL,
  short_name TEXT,
  name TEXT UNIQUE NOT NULL,
  description TEXT,
  appearance TEXT,
  category INTEGER NOT NULL,
  collected BOOLEAN NOT NULL,
  FOREIGN KEY(category) REFERENCES categories(id)
);

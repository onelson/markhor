CREATE TABLE collectibles (
  id INTEGER PRIMARY KEY NOT NULL,
  name TEXT UNIQUE NOT NULL,
  description TEXT,
  category INTEGER NOT NULL,
  collected BOOLEAN NOT NULL,
  FOREIGN KEY(category) REFERENCES categories(id)
);

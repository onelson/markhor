CREATE TABLE collectible_zone_membership (
  collectible_id INTEGER NOT NULL UNIQUE,
  zone_list TEXT NOT NULL,
  FOREIGN KEY(collectible_id) REFERENCES collectibles(id)
);

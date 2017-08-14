CREATE TABLE collectible_zone_membership (
  collectible_id INTEGER NOT NULL,
  zone_id INTEGER NOT NULL,
  FOREIGN KEY(collectible_id) REFERENCES collectibles(id),
  FOREIGN KEY(zone_id) REFERENCES zones(id),
  PRIMARY KEY (collectible_id, zone_id)
);

#[macro_use]
extern crate diesel;
use diesel::prelude::*;
use diesel::result::QueryResult;
use diesel::sqlite::SqliteConnection;

mod models;
mod schema;

pub type Conn = SqliteConnection;

#[allow(dead_code)]
pub fn get_conn(database_url: &str) -> Conn {
    Conn::establish(database_url).unwrap()
}

pub fn get_zones(conn: &Conn) -> QueryResult<Vec<models::Zone>> {
    use schema::zones::dsl::*;
    zones.load::<models::Zone>(conn)
}

pub fn get_categories(conn: &Conn) -> QueryResult<Vec<models::Category>> {
    use schema::categories::dsl::*;
    categories.order(name.asc()).load::<models::Category>(conn)
}

pub fn get_collectibles(conn: &Conn) -> QueryResult<Vec<models::Collectible>> {
    use schema::collectibles::dsl::*;
    collectibles
        .order(name.asc())
        .load::<models::Collectible>(conn)
}

pub fn get_memberships(conn: &Conn) -> QueryResult<Vec<models::CollectibleZoneMembership>> {
    use schema::collectible_zone_membership::dsl::*;
    collectible_zone_membership.load::<models::CollectibleZoneMembership>(conn)
}

pub fn update_collectible_collected(
    id_: i32,
    collected_: bool,
    conn: &Conn,
) -> QueryResult<models::Collectible> {
    use schema::collectibles::dsl::*;

    diesel::update(collectibles.find(id_))
        .set(collected.eq(collected_))
        .execute(conn)
        .expect("unable to update");

    collectibles
        .find(id_)
        .get_result::<models::Collectible>(conn)
}

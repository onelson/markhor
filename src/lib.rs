#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
use diesel::prelude::*;
use diesel::result::QueryResult;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::RunMigrationsError;

pub mod models;
mod schema;

pub type Conn = SqliteConnection;

#[allow(dead_code)]
pub fn get_conn(database_url: &str) -> Conn {
    Conn::establish(database_url).unwrap()
}

embed_migrations!("./migrations");

pub fn init_db(conn: &Conn) -> Result<(), RunMigrationsError> {
    embedded_migrations::run(conn)
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
    conn: &Conn,
    id_: i32,
    collected_: bool,
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

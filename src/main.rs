#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;

extern crate rocket_contrib;
extern crate serde;

extern crate serde_json;
#[macro_use]
extern crate serde_derive;


mod db;
mod models;

use std::io;
use std::path::{Path, PathBuf};
use std::env;
use rocket::response::NamedFile;


mod schema {
    infer_schema!("dotenv:DATABASE_URL");
}


mod api {
    use rocket_contrib::Json;
    use diesel::result::QueryResult;
    use diesel::prelude::*;
    use diesel;
    use db;
    use models;

    #[get("/zones")]
    pub fn get_zones(conn_: db::Conn) -> QueryResult<Json<Vec<models::Zone>>> {
        use schema::zones::dsl::*;
        let ref conn = *conn_;
        zones.load::<models::Zone>(conn).map(|xs| Json(xs))
    }


    #[get("/categories")]
    pub fn get_categories(conn_: db::Conn) -> QueryResult<Json<Vec<models::Category>>> {
        use schema::categories::dsl::*;
        let ref conn = *conn_;
        categories
            .order(name.asc())
            .load::<models::Category>(conn)
            .map(|xs| Json(xs))
    }

    #[get("/collectibles")]
    pub fn get_collectibles(conn_: db::Conn) -> QueryResult<Json<Vec<models::Collectible>>> {
        use schema::collectibles::dsl::*;
        let ref conn = *conn_;
        collectibles
            .order(name.asc())
            .load::<models::Collectible>(conn)
            .map(|xs| Json(xs))
    }

    #[get("/membership")]
    pub fn get_memberships(
        conn_: db::Conn,
    ) -> QueryResult<Json<Vec<models::CollectibleZoneMembership>>> {
        use schema::collectible_zone_membership::dsl::*;
        let ref conn = *conn_;
        collectible_zone_membership
            .load::<models::CollectibleZoneMembership>(conn)
            .map(|xs| Json(xs))
    }

    #[put("/collectibles/<id_>/collected/<collected_>")]
    pub fn update_collectible_collected(id_: i32, collected_: bool, conn_: db::Conn) -> QueryResult<Json<models::Collectible>> {
        use schema::collectibles::dsl::*;
        let ref conn = *conn_;

        diesel::update(collectibles.find(id_))
            .set(collected.eq(collected_))
            .execute(conn).expect("unable to update");

        collectibles.find(id_).get_result::<models::Collectible>(conn)
            .map(|x| Json(x))
    }
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("frontend/build/index.html")
}

#[get("/<file..>")]
fn spa_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("frontend/build/").join(file)).ok()
}

fn main() {
    dotenv::dotenv().ok();
    let _ = env_logger::init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    debug!("using db url: {}", database_url);
    rocket::ignite()
        .manage(db::init_pool(&database_url))
        .mount(
            "/api",
            routes![
                api::get_zones,
                api::get_categories,
                api::get_collectibles,
                api::get_memberships,
                api::update_collectible_collected
            ],
        )
        .mount("/", routes![spa_files, index])
        .launch();
}

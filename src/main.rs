#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
extern crate env_logger;
#[macro_use] extern crate log;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;

extern crate rocket_contrib;
extern crate serde;

extern crate serde_json;
#[macro_use]
extern crate serde_derive;


mod schema {
    infer_schema!("dotenv:DATABASE_URL");
}

mod db;
mod models;
use std::env;
use rocket_contrib::Json;
use diesel::result::QueryResult;
use diesel::prelude::*;



#[get("/zones")]
fn get_zones(conn_: db::Conn) -> QueryResult<Json<Vec<models::Zone>>> {
    use schema::zones::dsl::*;
    let ref conn = *conn_;
    zones
        .order(name.desc())
        .load::<models::Zone>(conn).map(|xs| Json(xs))
}


fn main() {
    dotenv::dotenv().ok();
    let _ = env_logger::init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    debug!("using db url: {}", database_url);
    rocket::ignite()
        .manage(db::init_pool(&database_url))
        .mount("/", routes![get_zones])
        .launch();
}

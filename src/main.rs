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

mod db;
mod models;
mod schema;
use std::env;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


//#[get("/tasks")]
//fn get_tasks(conn: DbConn) -> QueryResult<Json<Vec<Task>>> {
//    all_tasks.order(tasks::id.desc())
//        .load::<Task>(&conn)
//        .map(|tasks| Json(tasks))
//}


fn main() {
    dotenv::dotenv().ok();
    let _ = env_logger::init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    debug!("using db url: {}", database_url);
    rocket::ignite()
        .manage(db::init_pool(&database_url))
        .mount("/", routes![index])
        .launch();
}

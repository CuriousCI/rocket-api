#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

pub mod models;
pub mod schema;

use rocket_sync_db_pools::{database, diesel::PgConnection};

#[database("db")]
pub struct Db(PgConnection);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
	.attach(Db::fairing())
	.mount("/", routes![index])
}

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

pub mod models;
pub mod schema;
pub mod util;

use rocket_sync_db_pools::{database};
use diesel::dsl::*;
use diesel::prelude::*;	 

#[database("db")]
pub struct Db(util::PgConnection);

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

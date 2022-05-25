#[macro_use] extern crate rocket;
#[macro_use] extern  crate diesel;
// #[macro_use] extern crate rp1;

pub mod models;
pub mod schema;

use rocket_sync_db_pools::{database, diesel::PgConnection};
use diesel::prelude::*;

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
	.mount("/users", models::user::User::get_routes())
}

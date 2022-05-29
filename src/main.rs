#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;
pub mod util;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use diesel::{dsl::*, prelude::*};
use rocket::serde::{json::Json, uuid::Uuid as Id, Deserialize, Serialize};
use rocket_sync_db_pools::{database, diesel::PgConnection};
// use util::Guid;
use rocket::serde::uuid::Uuid as Guid;
use util::to_uuid;
use uuid::Uuid;

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

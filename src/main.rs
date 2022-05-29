#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;
pub mod util;

use chrono::NaiveDateTime;
use diesel::{dsl::*, prelude::*};
use models::*;
use rocket::serde::{json::Json, uuid::Uuid as Guid, Deserialize, Serialize};
use rocket_sync_db_pools::{database, diesel::PgConnection};
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
        .mount(
            "appointments",
            routes![
                appointment::post,
                appointment::get,
                appointment::get_id,
                appointment::put,
                appointment::delete
            ],
        )
        .mount(
            "feedback",
            routes![
                feedback::post,
                feedback::get,
                feedback::get_id,
                feedback::put,
                feedback::delete
            ],
        )
        .mount(
            "users",
            routes![
                // user::
            ],
        )
}

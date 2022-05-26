use uuid::Uuid;
use crate::*;
use chrono::NaiveDateTime;
use schema::appointments::dsl::*;
use rocket::serde::{json::Json, Serialize};
use rocket::http::Status;

#[derive(Queryable, Serialize)]
pub struct Appointment {
	pub appointment_id: Uuid,

	pub at: NaiveDateTime,

	pub user_id: Uuid,

	pub picture_consent: bool,
	pub analysis_consent: bool,
	pub data_consent: bool,

	pub analysis: String,
	pub picutre: String,

	pub created_at: NaiveDateTime,
	pub confirmed_at: NaiveDateTime,
	pub token: Uuid
}

#[post("/")]
pub async fn post(db: Db) {
}

#[get("/")]
pub async fn get(db:Db) -> Result<Json<Vec<Appointment>>, Status>{
		let result = db.run(move |db| {
			appointments.load(db)
		}).await;

		match result {
			Ok(value) => Json(value),
			Err => Status::InternalServerError
		}


}

#[get("/<id>")]
pub async fn get_id(id: Uuid, db: Db) {

}

#[put("/<id>")]
pub async fn put(id: Uuid, db: Db) {

}

#[delete("/<id>")]
pub async fn delete(id: Uuid, db: Db) {

}
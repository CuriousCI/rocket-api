use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Queryable)]
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
use uuid::Uuid;
use chrono::NaiveTime;

#[derive(Queryable)]

pub struct Feedback {
	pub feedback_id: Uuid,

	pub text: String,
	pub rating: i16,
	pub nps: i16,

	pub created_at : NaiveTime,

	pub user_id: Uuid
}
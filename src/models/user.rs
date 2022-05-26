use uuid::Uuid;
use chrono::NaiveDate;

#[derive(Queryable)]
pub struct User {
	pub user_id: Uuid,

	pub name: String,
	pub surname: String,
	pub fiscal_number: String,
	pub gender: String,
	pub category: String,

	pub email: String,
	pub phone_number: String,

	pub education: String,
	pub job: String,
	pub workplace: String,
	pub other_associations: String,

	pub birthday: NaiveDate,
	pub birthplace: String,
	pub birthplace_province: String,
	pub birthplace_istat_code: String,

	pub address: String,
	pub city: String,
	pub province: String,
	pub zip_code: String,
	pub istat_code: String
}


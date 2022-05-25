use uuid::Uuid;
use crate::Db;
// use crate::schema::users;

// #[derive(Queryable, AsChangeset)]
// #[diesel(table_name = users)]

#[rp1::crud(database = "Db", table = "users", auth="false")]
pub struct User {
	#[primary_key]
	pub user_id: Uuid,

	pub name: String,
	pub surname: String,
	pub fiscal_number: String,
	pub gender: String,
	pub category: String,

	#[validate(email)]
	pub email: String,
	#[validate(phone)]
	pub phone_number: String,

	pub education: String,
	pub job: String,
	pub workplace: String,
	pub other_associations: String,

	// pub birthday: NaiveDate,
	pub birthday: rp1::datetime::Date,
	pub birthplace: String,
	pub birthplace_province: String,
	pub birthplace_istat_code: String,

	pub address: String,
	pub city: String,
	pub province: String,
	pub zip_code: String,
	pub istat_code: String
}


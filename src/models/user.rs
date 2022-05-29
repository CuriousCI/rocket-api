use crate::*;
use schema::users::{self, dsl::*};

#[derive(Queryable, Identifiable, AsChangeset, Serialize, Deserialize)]
#[primary_key(user_id)]
pub struct User {
    pub user_id: Uuid,

    pub name: String,
    pub surname: String,
    pub fiscal_number: String,
    pub gender: String,
    pub category: String,

    pub email: String,
    pub phone_number: String,

    pub education: Option<String>,
    pub job: Option<String>,
    pub workplace: Option<String>,
    pub other_associations: Option<String>,

    pub birthday: NaiveDate,
    pub birthplace: String,
    pub birthplace_province: String,
    pub birthplace_istat_code: String,

    pub address: String,
    pub city: String,
    pub province: String,
    pub zip_code: String,
    pub istat_code: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub surname: String,
    pub fiscal_number: String,
    pub gender: String,
    pub category: String,

    pub email: String,
    pub phone_number: String,

    pub education: Option<String>,
    pub job: Option<String>,
    pub workplace: Option<String>,
    pub other_associations: Option<String>,

    pub birthday: NaiveDate,
    pub birthplace: String,
    pub birthplace_province: String,
    pub birthplace_istat_code: String,

    pub address: String,
    pub city: String,
    pub province: String,
    pub zip_code: String,
    pub istat_code: String,
}

#[post("/", data = "<user>")]
pub async fn post(user: Json<NewUser>, db: Db) {
    let user = user.into_inner();

    db.run(move |db| insert_into(users).values(&user).execute(db))
        .await
        .unwrap();
}

#[get("/")]
pub async fn get(db: Db) -> Json<Vec<User>> {
    Json(db.run(move |db| users.load(db)).await.unwrap())
}

#[get("/<id>")]
pub async fn get_id(id: Guid, db: Db) -> Json<User> {
    let id = to_uuid(id).unwrap();

    Json(
        db.run(move |db| users.filter(user_id.eq(id)).first(db))
            .await
            .unwrap(),
    )
}

#[put("/", data = "<user>")]
pub async fn put(user: Json<User>, db: Db) {
    let user = user.into_inner();

    db.run(move |db| update(users).set(&user).execute(db))
        .await
        .unwrap();
}

#[delete("/<id>")]
pub async fn delete(id: Guid, db: Db) {
    let id = to_uuid(id).unwrap();

    db.run(move |db| diesel::delete(users.filter(user_id.eq(id))).execute(db))
        .await
        .unwrap();
}

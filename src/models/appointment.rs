use crate::*;
use schema::appointments::{self, dsl::*};

#[derive(Queryable, Identifiable, Serialize)]
#[primary_key(appointment_id)]
pub struct Appointment {
    pub appointment_id: Uuid,

    pub at: NaiveDateTime,

    pub user_id: Option<Uuid>,

    pub picture_consent: Option<bool>,
    pub analysis_consent: Option<bool>,
    pub data_consent: bool,

    pub analysis: Option<String>,
    pub picture: Option<String>,

    pub created_at: Option<NaiveDateTime>,
    pub confirmed_at: Option<NaiveDateTime>,
    pub token: Option<Uuid>,
}

#[derive(Insertable, Deserialize)]
#[table_name = "appointments"]
pub struct NewAppointment {
    pub at: NaiveDateTime,

    pub user_id: Option<Uuid>,

    pub picture_consent: Option<bool>,
    pub analysis_consent: Option<bool>,
    pub data_consent: bool,
}

#[derive(AsChangeset, Deserialize)]
#[table_name = "appointments"]
pub struct UpdateAppointment {
    pub appointment_id: Uuid,

    pub at: NaiveDateTime,

    pub user_id: Option<Uuid>,

    pub picture_consent: Option<bool>,
    pub analysis_consent: Option<bool>,
    pub data_consent: bool,

    pub analysis: Option<String>,
    pub picture: Option<String>,
}

#[post("/", data = "<appointment>")]
pub async fn post(appointment: Json<NewAppointment>, db: Db) {
    let appointment = appointment.into_inner();

    db.run(move |db| insert_into(appointments).values(&appointment).execute(db))
        .await
        .unwrap();
}

#[get("/")]
pub async fn get(db: Db) -> Json<Vec<Appointment>> {
    Json(db.run(move |db| appointments.load(db)).await.unwrap())
}

#[get("/<id>")]
pub async fn get_id(id: Guid, db: Db) -> Json<Appointment> {
    let id = to_uuid(id).unwrap();

    Json(
        db.run(move |db| appointments.filter(appointment_id.eq(id)).first(db))
            .await
            .unwrap(),
    )
}

#[put("/<id>", data = "<appointment>")]
pub async fn put(id: Guid, appointment: Json<UpdateAppointment>, db: Db) {
    let id = to_uuid(id).unwrap();
    let appointment = appointment.into_inner();

    db.run(move |db| update(appointments).set(&appointment).execute(db))
        .await
        .unwrap();
}

#[delete("/<id>")]
pub async fn delete(id: Guid, db: Db) {
    let id = to_uuid(id).unwrap();

    db.run(move |db| diesel::delete(appointments.filter(appointment_id.eq(id))).execute(db))
        .await
        .unwrap();
}

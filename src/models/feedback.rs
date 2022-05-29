use crate::*;
use schema::feedback::{self, dsl::*};

#[derive(Queryable, Identifiable, Serialize)]
#[primary_key(feedback_id)]
#[table_name = "feedback"]
pub struct Feedback {
    pub feedback_id: Uuid,

    pub text: String,
    pub rating: i16,
    pub nps: i16,

    pub created_at: Option<NaiveDateTime>,

    pub user_id: Uuid,
}

#[derive(Insertable, Deserialize)]
#[table_name = "feedback"]
pub struct NewFeedback {
    pub text: String,
    pub rating: i16,
    pub nps: i16,

    pub user_id: Uuid,
}

#[derive(AsChangeset, Deserialize)]
#[table_name = "feedback"]
pub struct UpdateFeedback {
    pub feedback_id: Uuid,

    pub text: String,
    pub rating: i16,
    pub nps: i16,
}

#[post("/", data = "<feed>")]
pub async fn post(feed: Json<NewFeedback>, db: Db) {
    let feed = feed.into_inner();

    db.run(move |db| insert_into(feedback).values(&feed).execute(db))
        .await
        .unwrap();
}

#[get("/")]
pub async fn get(db: Db) -> Json<Vec<Feedback>> {
    Json(db.run(move |db| feedback.load(db)).await.unwrap())
}

#[get("/<id>")]
pub async fn get_id(id: Guid, db: Db) -> Json<Feedback> {
    let id = to_uuid(id).unwrap();

    Json(
        db.run(move |db| feedback.filter(feedback_id.eq(id)).first(db))
            .await
            .unwrap(),
    )
}

#[put("/<id>", data = "<feed>")]
pub async fn put(id: Guid, feed: Json<UpdateFeedback>, db: Db) {
    let id = to_uuid(id).unwrap();
    let feed = feed.into_inner();

    db.run(move |db| update(feedback).set(&feed).execute(db))
        .await
        .unwrap();
}

#[delete("/<id>")]
pub async fn delete(id: Guid, db: Db) {
    let id = to_uuid(id).unwrap();

    db.run(move |db| diesel::delete(feedback.filter(feedback_id.eq(id))).execute(db))
        .await
        .unwrap();
}

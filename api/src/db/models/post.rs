use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Deserialize, Serialize, Debug)]
#[diesel(table_name = crate::db::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Author))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published_at: Option<NaiveDateTime>,
    pub author_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Debug, AsChangeset)]
#[diesel(table_name = crate::db::schema::posts)]
#[serde(crate = "rocket::serde")]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct PublishPostBody {
    pub author_token: String,
}
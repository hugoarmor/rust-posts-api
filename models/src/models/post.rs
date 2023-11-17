use clap::Args;
use diesel::*;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[cfg_attr(feature = "diesel", derive(Queryable, Selectable))]
#[cfg_attr(feature = "diesel", diesel(table_name = crate::schema::posts))]
#[cfg_attr(feature = "diesel", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "diesel", diesel(belongs_to(Author)))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published_at: Option<NaiveDateTime>,
    pub author_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Serialize, Debug, Args)]
#[cfg_attr(feature = "diesel", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "diesel", diesel(table_name = crate::schema::posts))]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PublishPostBody {
    pub author_token: String,
}

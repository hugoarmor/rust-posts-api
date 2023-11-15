use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Deserialize, Serialize, Debug)]
#[diesel(table_name = crate::db::schema::authors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Author {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub token: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Debug, AsChangeset)]
#[diesel(table_name = crate::db::schema::authors)]
pub struct NewAuthorRequestBody {
    pub email: String,
    pub name: String,
}

#[derive(AsChangeset, Insertable)]
#[diesel(table_name = crate::db::schema::authors)]
pub struct NewAuthor {
    pub email: String,
    pub name: String,
    pub token: String,
}

use diesel::*;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[cfg_attr(feature = "diesel", derive(Queryable, Selectable))]
#[cfg_attr(feature = "diesel", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "diesel", diesel(table_name = crate::schema::authors))]
pub struct Author {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub token: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Serialize, Debug)]
#[cfg_attr(feature = "diesel", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "diesel", diesel(table_name = crate::schema::authors))]
pub struct NewAuthorRequestBody {
    pub email: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[cfg_attr(feature = "diesel", derive(AsChangeset, Insertable))]
#[cfg_attr(feature = "diesel", diesel(table_name = crate::schema::authors))]
pub struct NewAuthor {
    pub email: String,
    pub name: String,
    pub token: String,
}

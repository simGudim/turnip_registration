use crate::db::schema::users;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use diesel::sql_types::*;

#[derive(Debug, Queryable, Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String
}




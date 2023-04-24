#[allow(dead_code)]
use crate::db::schema::users;
// use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
// use diesel::sql_types::*;

#[derive(Debug, Queryable, Serialize, Deserialize, Insertable)]
#[diesel(primary_key(id))]
#[diesel(table_name = users)]
pub struct User {
    #[diesel(deserialize_as = i32)]
    pub user_id: Option<i32>,
    pub username: String
}




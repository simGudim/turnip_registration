#[allow(dead_code)]
use crate::db::{Db, PgPooledConnection};
use crate::utils;
use actix_web::{HttpResponse, Result, get};
use serde_json::json;


#[get("/users")]
pub async fn get_all_users(pool: PgPooledConnection) -> Result<HttpResponse> {
    let users = Db::get_all_users(pool).await;
    if !users.is_empty() {
        utils::log_success(json!({
            "number_of_users" : users.len()
        }));
        Ok(HttpResponse::Ok()
            .json(users))
    } else {
        Ok(HttpResponse::Ok()
            .body("The users table is empty!"))
    }
}
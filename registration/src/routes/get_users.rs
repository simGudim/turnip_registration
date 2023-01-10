use crate::db::{Db, PgPooledConnection};
use actix_web::{HttpResponse, Result, get};

#[get("/users")]
pub async fn get_all_users(pool: PgPooledConnection) -> Result<HttpResponse> {
    let users = Db::get_users(pool).await;
    if !users.is_empty() {
        Ok(HttpResponse::Ok()
            .json(users))
    } else {
        Ok(HttpResponse::Ok()
            .body("The users table is empty!"))
    }
}
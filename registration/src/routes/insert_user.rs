use crate::db::{Db, PgPooledConnection};
use actix_web::{HttpResponse, Result, post};


struct IncomingRequest {
    username: String
}

type Request = web::Json<IncomingRequest>;

#[post("/user")]
pub async fn insert_user(pool: PgPooledConnection, request: Request) -> Result<HttpResponse> {
    let users = Db::get_users(pool).await;
    if !users.is_empty() {
        Ok(HttpResponse::Ok()
            .json(users))
    } else {
        Ok(HttpResponse::Ok()
            .body("The users table is empty!"))
    }
}
use crate::db::{Db, PgPool};
use actix_web::{HttpResponse, Result, web, get};



#[get("/users")]
pub async fn get_all_users(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let mut conn = pool.get()
        .expect("couldn't get db connection from pool");
    let users = Db::get_users(&mut conn).await;
    if !users.is_empty() {
        Ok(HttpResponse::Ok()
            .json(users))
    } else {
        Ok(HttpResponse::Ok()
            .body("The users table is empty!"))
    }
}
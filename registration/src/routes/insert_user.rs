use crate::db::{Db, PgPooledConnection, models::User};
use actix_web::{HttpResponse, Result, post, web};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IncomingRequest {
    pub username: String
}

impl IncomingRequest {
    fn validate(&self) -> User {
        User {
            user_id : None,
            username : self.username.clone()
        }
    }
}

type Request = web::Json<IncomingRequest>;


#[post("/user")]
pub async fn insert_user(pool: PgPooledConnection, request: Request) -> Result<HttpResponse> {
    let user = request.validate();
    let users = Db::insert_user(pool, &user).await;
    if users{
        Ok(HttpResponse::Ok()
            .json(users))
    } else {
        Ok(HttpResponse::Ok()
            .body("The users table is empty!"))
    }
}
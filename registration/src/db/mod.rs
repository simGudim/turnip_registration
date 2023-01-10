pub mod models;
pub mod schema;


use models::User;
use actix_web::web;
use diesel::{
    Connection, 
    ExpressionMethods,
    PgConnection, 
    QueryDsl, 
    RunQueryDsl,
    self
};
use diesel::r2d2::{ 
    Pool, 
    PooledConnection, 
    ConnectionManager
};
use std::env;
use tracing;
// use failure::Error;
// use diesel::QueryResult;
// use chrono::Utc;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = web::Data<PgPool>;

#[derive(Clone)]
pub struct Db {
    pub pool: PgPool
}

impl Db {
    #[tracing::instrument]
    pub async fn establish_connection() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));

        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder().build(manager).unwrap();
        let db = Self {
            pool
        };
        db    
    }

    pub async fn get_user_by_username(user_name: &String, pool: web::Data<PgPool>) -> Option<User> {
        use self::schema::users::dsl::*;

        let mut conn = pool.get().expect("couldn't get db connection from pool");
        let mut items = users
            .filter(username.eq(user_name))
            .load::<models::User>(&mut conn)
            .expect("Error getting the user by username");
        items.pop()
    }

    pub async fn get_users(pool: web::Data<PgPool>) -> Vec<User> {
        use self::schema::users::dsl::*;
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        let items = users
            .order(id.asc())
            .load::<models::User>(&mut conn)
            .expect("Error loading devices");
        items
    }
}






#[allow(dead_code)]
pub mod models;
pub mod schema;

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
    ConnectionManager
};
use models::User;
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
        eprintln!("established connection to {}", &database_url);
        PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));

        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder().build(manager).unwrap();
        let db = Self {
            pool
        };
        db    
    }

    pub async fn get_user_by_username(user_name: &String, pool: PgPooledConnection) -> Option<User> {
        use self::schema::users::dsl::*;

        let mut conn = pool.get().expect("couldn't get db connection from pool");
        let mut items = users
            .filter(username.eq(user_name))
            .load::<models::User>(&mut conn)
            .expect("Error getting the user by username");
        items.pop()
    }

    pub async fn get_all_users(pool: PgPooledConnection) -> Vec<User> {
        use self::schema::users::dsl::*;
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        let items = users
            .order(user_id.asc())
            .load::<models::User>(&mut conn)
            .expect("Error loading devices");
        items
    }
    
    pub async fn insert_user(pool: PgPooledConnection, user: &User) -> bool {
        use self::schema::users::dsl::*;
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        let inserted_user :Vec<User>= diesel::insert_into(users)
            .values(user)
            .load(&mut conn)
            .expect("error inserting a user");
        tracing::debug!("{:?}", inserted_user);
        true
    }
}






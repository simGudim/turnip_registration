pub mod models;
pub mod schema;


use models::User;

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
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

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

    pub async fn get_user_by_username(user_name: &String, conn: &mut PgPooledConnection) -> Option<User> {
        use self::schema::users::dsl::*;
        let mut items = users
            .filter(username.eq(user_name))
            .load::<models::User>(conn)
            .expect("Error getting the user by username");
        items.pop()
    }

    pub async fn get_users(conn: &mut PgPooledConnection) -> Vec<User> {
        use self::schema::users::dsl::*;
        let items = users
            .order(id.asc())
            .load::<models::User>(conn)
            .expect("Error loading devices");
        items
    }
}






pub mod models;
pub mod schema;


use models::User;
use diesel::{
    Connection, 
    ExpressionMethods,
    PgConnection, 
    QueryDsl, 
    RunQueryDsl, 
    insert_into
};
use std::env;
use tracing::{info, instrument};
use failure::Error;
use diesel::r2d2::{ Pool, PooledConnection, ConnectionManager};
use diesel::QueryResult;
use diesel::sql_query;
use chrono::Utc;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct Db {
    pub pool: PgPool
}

static  MIN30: i64 = 1800000;

impl Db {
    #[instrument]
    pub async fn establish_connection() -> Self {
        info!("Creating db connection pool");
        let database_url = env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager).unwrap();
        let db = Self {
            pool
        };
        db    
    }
}






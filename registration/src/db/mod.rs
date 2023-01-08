pub mod models;
pub mod schema;


use models::{User, Device};
use diesel::{Connection, ExpressionMethods,PgConnection, 
    QueryDsl, RunQueryDsl, insert_into};
use std::env;
use tracing::{info, instrument};
use failure::Error;
use diesel::r2d2::{ Pool, PooledConnection, ConnectionManager};
use diesel::QueryResult;
use diesel::sql_query;
use chrono::Utc;
use std::collections::HashMap;

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

    pub fn get_all_events(conn: &PgPooledConnection) -> Result<Vec<models::EventStruct>, String> {
        let mut sqlselect = String::from(r#"
            SELECT * FROM eventlist
            ORDER BY created_at ASC
            LIMIT 500;
            "#);
        println!("Get event SQL: {}", sqlselect);
        let all_events = sql_query(&sqlselect).load::<models::EventStruct>(conn);

        match all_events {
            Ok(vect) => {
                Ok(vect)
            },
            Err(e) => {
                println!("Error when getting the events!{}", e);
                Err("Wrong username or password".to_string())
            },
        }
    }


   pub async fn get_user_by_username(user_name: &String, conn: &PgPooledConnection) -> Option<User> {
        use self::schema::users::dsl::*;
        let mut items = users
            .filter(name.eq(user_name))
            .load::<models::User>(conn)
            .expect("Error loading person");
        items.pop()
    }

    pub async fn get_user_by_email(email: &String, conn: &PgPooledConnection) -> Option<User> {
        use self::schema::users::dsl::*;
        let mut items = users
            .filter(emailaddr.eq(email))
            .load::<models::User>(conn)
            .expect("Error loading person");
        items.pop()
    }

    pub async fn get_users(conn: &PgPooledConnection) -> Vec<User> {
        use self::schema::users::dsl::*;
        let items = users
            .order(id.asc())
            .load::<models::User>(conn)
            .expect("Error loading devices");
        items
    }


    pub async fn add_user(user: &models::UserInsert, conn: PgPooledConnection) -> Option<models::User> {
        use self::schema::users::dsl::*;

        let mut added_user = insert_into(users)
            .values(user)
            // .returning(<models::User>)
            // .execute(&conn)
            .load::<models::User>(&conn)
            .expect("Error inserting person");
        added_user.pop()
    }

    pub async fn edit_user_info(
        conn: &PgPooledConnection, 
        user_id: i64,
        new_name: &String, 
        new_email: &String, 
        new_user_role: bool
    ) -> Option<models::User> {
        use self::schema::users::dsl::*;
        let user = diesel::update(users
            .filter(id.eq(user_id)))
            .set((name.eq(new_name), emailaddr.eq(new_email), user_role.eq(new_user_role)))
            .get_result(conn);
        user.ok()
    } 

    pub async fn get_devices(conn: &PgPooledConnection) -> Vec<Device> {
        use self::schema::devices::dsl::*;
        let items = devices
            // .filter(device_state.eq(true))
            .order(id.asc())
            .load::<models::Device>(conn)
            .expect("Error loading devices");
        items
    }

    pub async fn get_profiles(conn: &PgPooledConnection) -> Vec<models::Profile> {
        use self::schema::profiles::dsl::*;
        let items = profiles
            // .filter(device_state.eq(true))
            .order(id.asc())
            .load::<models::Profile>(conn)
            .expect("Error loading profiles");
        items
    }

    pub async fn change_profile(conn: &PgPooledConnection, panelsn: &String, sn: &String, new_profile: i32) -> Option<models::DeviceQuery> {
        use self::schema::devices::dsl::*;
        let device = diesel::update(devices.filter(serial.eq(sn)).filter(panelserial.eq(panelsn)))
            .set(profile.eq(new_profile))
            .get_result(conn);

        device.ok()
    } 

    pub async fn change_polltime(conn: &PgPooledConnection, profile: i32, new_polltime: i32) -> Option<models::Profile> {
        use self::schema::profiles::dsl::*;
        let profile = diesel::update(profiles.filter(code.eq(profile)))
            .set(polltime.eq(new_polltime))
            .get_result(conn);
        profile.ok()
    } 

    pub fn count_online_offline(conn: &PgPooledConnection) -> Option<HashMap<&str, i64>> {
        use diesel::sql_types::BigInt;
        #[derive(QueryableByName)]
        struct Count {
            #[sql_type = "BigInt"]
            count: i64,
        }
        let mut result = HashMap::new();
        let online: i64 = diesel::sql_query(
            r#"SELECT COUNT(*) FROM devices
            WHERE device_state = TRUE"#)
            .load::<Count>(conn)
            .expect("Query failed")
            .pop()
            .expect("No rows")
            .count;
        result.insert("online", online);

        let offline: i64 = diesel::sql_query(
                r#"SELECT COUNT(*) FROM devices
                WHERE device_state = FALSE"#)
                .load::<Count>(conn)
                .expect("Query failed")
                .pop()
                .expect("No rows")
                .count;
        result.insert("offline", offline);
        Some(result)
    }

    pub fn profile_breakdown(conn: &PgPooledConnection) -> Option<Vec<models::ProfileCounts>> {
        use diesel::sql_types::BigInt;
 
        let pr1: Vec<models::ProfileCounts> = diesel::sql_query(
            r#"SELECT profile, COUNT(*) as profile_counts FROM devices
            GROUP BY profile
            ORDER BY profile ASC"#)
            .load::<models::ProfileCounts>(conn)
            .expect("Query failed");
        Some(pr1)
    }
}









    // pub async fn add_user(user: &models::User, conn: PgPooledConnection) -> Result<usize, Error> {
    //     use self::schema::users::dsl::*;

    //     let row_inserted = insert_into(users)
    //         .values(user)
    //         .returning(schema::users::id)
    //         .execute(&conn)
    //         .expect("Error inserting person");
    //     Ok(row_inserted)
    // }

    // pub async fn get_user_by_username(user_name: &String, conn: &PgPooledConnection) -> Option<User> {
    //     use self::schema::users::dsl::*;
    //     let mut items = users
    //         .filter(username.eq(user_name))
    //         .load::<models::User>(conn)
    //         .expect("Error loading person");
    //     items.pop()
    // }

    // pub async fn get_user_by_id(id: &Uuid, conn: &PgPooledConnection) -> Option<User> {
    //     use self::schema::users::dsl::*;
    //     let mut items = users
    //         .filter(id.eq(id))
    //         .load::<models::User>(conn)
    //         .expect("Error loading person");
    //     items.pop()
    // }

    // pub async fn change_password(user_name: &String, new_password: &String, conn: &PgPooledConnection) -> Option<User> {
    //     use self::schema::users::dsl::*;
    //     let user = diesel::update(users.filter(username.eq(user_name)))
    //         .set(password_hash.eq(new_password))
    //         .get_result::<User>(conn)
    //         .expect(&format!("Unable to find username: {}", user_name));
    //     Some(user)
    // }
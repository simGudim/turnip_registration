mod conf;
mod routes;
mod db;

// extern crate crypto;
#[macro_use]
extern crate diesel;
// #[macro_use]
// extern crate validator_derive;

use crate::conf::Config;
use crate::db::Db;

use actix_web::{web, App,HttpServer};  
use actix_web::middleware::Logger;
use tracing;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env().expect("Server configuration not set");
    let db = Db::establish_connection().await;   
    tracing::info!(
        "Starting server at http://{}:{}", config.host, config.port
    );
    
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(db.pool.clone()))
            .service(routes::get_users::get_all_users)
            // .configure(f)
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await?;
    
    Ok(())
}

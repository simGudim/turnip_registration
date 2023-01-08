mod conf;
mod routes;
mod db;

extern crate crypto;
// #[macro_use]
// extern crate diesel;
// #[macro_use]
// extern crate validator_derive;

use crate::conf::Config;
use actix_web::{web, App, HttpResponse, HttpServer, Result, http};  
use actix_files as fs;
use actix_web::middleware::Logger;
use serde::{Deserialize, Serialize};
use tracing::{info};
use tracing_subscriber::EnvFilter;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env()
        .expect("Server configuration not set");
    let db = Db::establish_connection().await;   
    info!("Starting server at http://{}:{}", config.host, config.port);
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8000")
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            // .wrap(cors)
            .wrap(Logger::default())
            .app_data(web::Data::new(mongo.clone()))
            // .data(db.pool.clone())
            // .service(fs::Files::new("/static", ".").show_files_listing())
            .service(login::create_user)
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await?;
    
    Ok(())
}

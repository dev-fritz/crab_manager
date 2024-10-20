mod config;
mod handlers;
mod models;
mod repositories;
mod services;

use config::database::establish_connection;

use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer};
use dotenvy::dotenv;
use log::{info, error};

struct AppState {
    db_pool: sqlx::MySqlPool,
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    info!("Logging system initialized successfully!");

    let database_url = std::env::var("DATABASE_URL_SLAVE").expect("DATABASE_URL_SLAVE must be set");
    info!("Database URL: {}", database_url);

    let pool = match establish_connection(database_url).await {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to connect to database: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to connect to database"));
        }
    };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT","PATCH", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(AppState { db_pool: pool.clone() }))
            .configure(handlers::config)
            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}


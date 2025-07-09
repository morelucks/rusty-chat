use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use config::settings::AppConfig;
use database::connection::{create_pool, run_migrations};
use tracing::{info, error};
use tracing_subscriber;
use dotenv::dotenv;

pub mod config;
pub mod database;
pub mod models;
pub mod handlers;
pub mod utils;
pub mod routes;

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("rusty-chat")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = AppConfig::from_env().unwrap_or_else(|e| {
        error!("Failed to load configuration: {}", e);
        std::process::exit(1);
    });

    info!("Starting server with config: {:?}", config);
    
    let pool = create_pool(&config.database).await.unwrap_or_else(|e| {
        error!("Failed to create database pool: {}", e);
        std::process::exit(1);
    });

    info!("Database pool created successfully");
    
    run_migrations(&pool).await.unwrap_or_else(|e| {
        error!("Failed to run database migrations: {}", e);
        std::process::exit(1);
    });

    info!("Database migrations completed successfully");

    let server_host = config.server.host.clone();
    let server_port = config.server.port;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(config.clone()))
            .wrap(Logger::default())
            .service(
                web::scope("/api/v1")
                    .configure(routes::api::scoped_config)
            )
    })
    .bind((server_host, server_port))?
    .run()
    .await?;
    
    Ok(())
}

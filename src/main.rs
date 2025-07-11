use actix::Actor;
use actix_web::{middleware::Logger, web, App, HttpServer};
use config::settings::AppConfig;
use database::connection::{create_pool, run_migrations};
use dotenv::dotenv;
use tracing::{error, info};
use tracing_subscriber;
use rusty_chat::ws_server::ChatServer;
use crate::services::auth::AuthService;

pub mod config;
pub mod database;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod requests;
pub mod routes;
pub mod services;
pub mod utils;

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

    let chat_server = ChatServer::new().start();
    let auth_service = AuthService::new().unwrap_or_else(|e| {
        error!("Failed to create auth service: {}", e);
        std::process::exit(1);
    });
    let auth_service = std::sync::Arc::new(auth_service);

    //use: http://localhost:8080/api/v1/users to test
    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(chat_server.clone()))
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::from(auth_service.clone()))
            .wrap(Logger::default())
            .service(web::scope("/api/v1").configure(routes::api::scoped_config))
    })
    .bind((server_host, server_port))?
    .run()
    .await?;

    Ok(())
}

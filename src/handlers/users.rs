use actix_web::{web, HttpResponse, Result};
use tracing::error;
use crate::{
    utils::helpers::ApiResponse,
    models::user::User,
    database::connection::DbPool
};

pub async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse> {
    let users = User::find_all(&pool).await
        .map_err(|e| {
            error!("Failed to fetch users: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to fetch users")
        })?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(users)))
}
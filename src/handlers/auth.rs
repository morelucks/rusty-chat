use crate::{database::connection::DbPool, models::user::{CreateUser, User}, utils::helpers::ApiResponse};
use actix_web::{HttpResponse, Result, web};
use tracing::error;

// pub async fn login(pool: web::Data<DbPool>, user: web::Json<LoginUser>) -> Result<HttpResponse> {
//     let user = User::find_by_email(&pool, user.email).await.map_err(|e| {
//         error!("Failed to find user: {}", e);
//         actix_web::error::ErrorInternalServerError("Failed to find user")
//     })?;

//     Ok(HttpResponse::Ok().json(ApiResponse::success(user)))
// }

pub async fn register(pool: web::Data<DbPool>, user: web::Json<CreateUser>) -> Result<HttpResponse> {
    let new_user = User::create(&pool, user.into_inner()).await.map_err(|e| {
        error!("Failed to register user: {}", e);
        actix_web::error::ErrorInternalServerError("Failed to register user")
    })?;

    Ok(HttpResponse::Created().json(ApiResponse::success(new_user)))
}
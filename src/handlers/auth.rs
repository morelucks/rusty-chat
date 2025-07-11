use crate::{database::connection::DbPool, models::user::{CreateUser, OnlineStatus, User, UserRegistrationRequest}, utils::helpers::ApiResponse};
use actix_web::{HttpResponse, Result, web};
use tracing::error;

// pub async fn login(pool: web::Data<DbPool>, user: web::Json<LoginUser>) -> Result<HttpResponse> {
//     let user = User::find_by_email(&pool, user.email).await.map_err(|e| {
//         error!("Failed to find user: {}", e);
//         actix_web::error::ErrorInternalServerError("Failed to find user")
//     })?;

//     Ok(HttpResponse::Ok().json(ApiResponse::success(user)))  
// }

pub async fn register(pool: web::Data<DbPool>, user: web::Json<UserRegistrationRequest>) -> Result<HttpResponse> {
    let user_data = user.into_inner();
    
    if user_data.password != user_data.confirm_password {
        return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error("Password and confirm password do not match".to_string())));
    }

    let existing_email = User::find_by_email(&pool, user_data.email.clone()).await.map_err(|e| {
        error!("Failed to find user: {}", e);
        actix_web::error::ErrorInternalServerError("Failed to find user")
    })?;

    if existing_email.is_some() {
        return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error("Email already exists".to_string())));
    }

    let existing_username = User::find_by_username(&pool, user_data.username.clone()).await.map_err(|e| {
        error!("Failed to check username: {}", e);
        actix_web::error::ErrorInternalServerError("Failed to find user")
    })?;

    if existing_username.is_some() {
        return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error("Username already exist".to_string())));
    }

    // create user
    let new_user = User::create(&pool, CreateUser {
        full_name: user_data.full_name,
        username: user_data.username,
        email: user_data.email,
        password: user_data.password,
        status: OnlineStatus::Offline,
    }).await.map_err(|e| {
        error!("Failed to register user: {}", e);
        actix_web::error::ErrorInternalServerError("Failed to register user")
    })?;

    Ok(HttpResponse::Created().json(ApiResponse::success(new_user)))
}
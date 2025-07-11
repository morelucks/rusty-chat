use crate::{database::connection::DbPool, models::user::{CreateUser, LoginUser, OnlineStatus, User, UserRegistrationRequest}, utils::{helpers::ApiResponse, jwt}};
use actix_web::{HttpResponse, Result, web};
use tracing::error;

pub async fn login(pool: web::Data<DbPool>, user: web::Json<LoginUser>) -> Result<HttpResponse> {
    let user_exist = User::find_by_email(&pool, user.email.clone()).await.map_err(|e| {
        error!("Failed to find user: {}", e);
        actix_web::error::ErrorInternalServerError("Failed to find user")
    })?;

    if user_exist.is_none() {
        return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error("Email does exists".to_string())));
    }

    let user_exist = match user_exist {
        Some(u) => u,
        None => {
            return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error("Email does not exist".to_string())));
        }
    };

    if user.password.clone() != user_exist.password {
        return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error("Invalid login details".to_string())));
    }

    let token = jwt::create_jwt(&user_exist.id.to_string(), 60 * 24).map_err(|e| {
        error!("Failed to create JWT: {}", e);
        actix_web::error::ErrorInternalServerError("Failed to create token")
    })?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "token": token,
        "user": user
    })))

    
}

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
use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;
use tracing::{error, warn};
use crate::{
    utils::helpers::ApiResponse,
    models::room::{Room, CreateRoom},
    database::connection::DbPool
};

pub async fn create_room(
    pool: web::Data<DbPool>,
    room_data: web::Json<CreateRoom>,
) -> Result<HttpResponse> {
    let room = Room::create(&pool, room_data.into_inner()).await
        .map_err(|e| {
            error!("Failed to create room: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to create room")
        })?;

    Ok(HttpResponse::Created().json(ApiResponse::success(room)))
}

pub async fn get_all_rooms(pool: web::Data<DbPool>) -> Result<HttpResponse> {
    let rooms = Room::find_all(&pool).await
        .map_err(|e| {
            error!("Failed to fetch rooms: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to fetch rooms")
        })?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(rooms)))
}

pub async fn get_room_by_id(
    pool: web::Data<DbPool>,
    room_id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let room_id = room_id.into_inner();
    let room = Room::find_by_id(&pool, room_id).await
        .map_err(|e| {
            error!("Failed to fetch room {}: {}", room_id, e);
            actix_web::error::ErrorInternalServerError("Failed to fetch room")
        })?
        .ok_or_else(|| {
            warn!("Room not found: {}", room_id);
            actix_web::error::ErrorNotFound("Room not found")
        })?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(room)))
}
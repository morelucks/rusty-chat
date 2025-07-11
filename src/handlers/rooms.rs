use crate::{
    database::connection::DbPool,
    middleware::auth::AuthenticatedUser,
    models::room::{CreateRoom, Room},
    requests::room_requests::CreateRoomRequest,
    utils::helpers::ApiResponse,
};
use actix_web::{web, HttpResponse, Result};
use tracing::{error, warn};
use uuid::Uuid;

pub async fn create_room(
    pool: web::Data<DbPool>,
    room_data: web::Json<CreateRoomRequest>,
    user: AuthenticatedUser,
) -> Result<HttpResponse> {
    let create_data = CreateRoom {
        name: room_data.name.clone(),
        is_private: room_data.is_private,
        created_by: user.user_id,
    };

    let room = Room::create(&pool, create_data).await.map_err(|e| {
        error!("Failed to create room: {}", e);
        actix_web::error::ErrorInternalServerError("Failed to create room")
    })?;

    Ok(HttpResponse::Created().json(ApiResponse::success(room)))
}

pub async fn get_all_rooms(pool: web::Data<DbPool>) -> Result<HttpResponse> {
    let rooms = Room::find_all(&pool).await.map_err(|e| {
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
    let room = Room::find_by_id(&pool, room_id)
        .await
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

use crate::{
    database::connection::DbPool,
    handlers::message_broadcast::{BroadcastService, CreateBroadcastMessage, BroadcastMessageType},
    utils::helpers::ApiResponse,
};
use actix_web::{HttpResponse, Result, web};
use serde::{Deserialize, Serialize};
use tracing::error;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct BroadcastTextRequest {
    pub room_id: Uuid,
    pub sender_id: Uuid,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct BroadcastTypingRequest {
    pub room_id: Uuid,
    pub sender_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct BroadcastReadRequest {
    pub room_id: Uuid,
    pub sender_id: Uuid,
    pub message_id: String,
}

#[derive(Debug, Deserialize)]
pub struct JoinRoomRequest {
    pub room_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct LeaveRoomRequest {
    pub room_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct SystemMessageRequest {
    pub room_id: Uuid,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct BroadcastResponse {
    pub success: bool,
    pub message: String,
}

/// Broadcast a text message to all users in a room except the sender
pub async fn broadcast_text(
    pool: web::Data<DbPool>,
    request: web::Json<BroadcastTextRequest>,
) -> Result<HttpResponse> {
    let broadcast_service = BroadcastService::new(pool.get_ref().clone());
    
    match broadcast_service.broadcast_text_message(
        request.room_id,
        request.sender_id,
        request.content.clone(),
    ).await {
        Ok(_) => {
            let response = BroadcastResponse {
                success: true,
                message: "Message broadcasted successfully".to_string(),
            };
            Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
        }
        Err(e) => {
            error!("Failed to broadcast text message: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Failed to broadcast message".to_string(),
            )))
        }
    }
}

/// Broadcast typing indicator to all users in a room except the sender
pub async fn broadcast_typing(
    pool: web::Data<DbPool>,
    request: web::Json<BroadcastTypingRequest>,
) -> Result<HttpResponse> {
    let broadcast_service = BroadcastService::new(pool.get_ref().clone());
    
    match broadcast_service.broadcast_typing(request.room_id, request.sender_id).await {
        Ok(_) => {
            let response = BroadcastResponse {
                success: true,
                message: "Typing indicator broadcasted successfully".to_string(),
            };
            Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
        }
        Err(e) => {
            error!("Failed to broadcast typing indicator: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Failed to broadcast typing indicator".to_string(),
            )))
        }
    }
}

/// Broadcast read receipt to all users in a room except the sender
pub async fn broadcast_read_receipt(
    pool: web::Data<DbPool>,
    request: web::Json<BroadcastReadRequest>,
) -> Result<HttpResponse> {
    let broadcast_service = BroadcastService::new(pool.get_ref().clone());
    
    match broadcast_service.broadcast_read_receipt(
        request.room_id,
        request.sender_id,
        request.message_id.clone(),
    ).await {
        Ok(_) => {
            let response = BroadcastResponse {
                success: true,
                message: "Read receipt broadcasted successfully".to_string(),
            };
            Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
        }
        Err(e) => {
            error!("Failed to broadcast read receipt: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Failed to broadcast read receipt".to_string(),
            )))
        }
    }
}

/// Add user to room and broadcast join event
pub async fn join_room(
    pool: web::Data<DbPool>,
    request: web::Json<JoinRoomRequest>,
) -> Result<HttpResponse> {
    let broadcast_service = BroadcastService::new(pool.get_ref().clone());
    
    match broadcast_service.add_user_to_room(request.room_id, request.user_id).await {
        Ok(_) => {
            let response = BroadcastResponse {
                success: true,
                message: "User joined room successfully".to_string(),
            };
            Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
        }
        Err(e) => {
            error!("Failed to add user to room: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Failed to join room".to_string(),
            )))
        }
    }
}

/// Remove user from room and broadcast leave event
pub async fn leave_room(
    pool: web::Data<DbPool>,
    request: web::Json<LeaveRoomRequest>,
) -> Result<HttpResponse> {
    let broadcast_service = BroadcastService::new(pool.get_ref().clone());
    
    match broadcast_service.remove_user_from_room(request.room_id, request.user_id).await {
        Ok(_) => {
            let response = BroadcastResponse {
                success: true,
                message: "User left room successfully".to_string(),
            };
            Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
        }
        Err(e) => {
            error!("Failed to remove user from room: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Failed to leave room".to_string(),
            )))
        }
    }
}

/// Broadcast system message to all users in a room
pub async fn broadcast_system_message(
    pool: web::Data<DbPool>,
    request: web::Json<SystemMessageRequest>,
) -> Result<HttpResponse> {
    let broadcast_service = BroadcastService::new(pool.get_ref().clone());
    
    match broadcast_service.broadcast_system_message(request.room_id, request.content.clone()).await {
        Ok(_) => {
            let response = BroadcastResponse {
                success: true,
                message: "System message broadcasted successfully".to_string(),
            };
            Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
        }
        Err(e) => {
            error!("Failed to broadcast system message: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "Failed to broadcast system message".to_string(),
            )))
        }
    }
}

/// Get all users in a room
pub async fn get_room_users(
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let room_id = path.into_inner();
    let broadcast_service = BroadcastService::new(pool.get_ref().clone());
    let connection_manager = broadcast_service.get_connection_manager();
    
    let users = connection_manager.get_room_users(room_id).await;
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(users)))
}

/// Check if a user is in a room
pub async fn is_user_in_room(
    pool: web::Data<DbPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse> {
    let (room_id, user_id) = path.into_inner();
    let broadcast_service = BroadcastService::new(pool.get_ref().clone());
    let connection_manager = broadcast_service.get_connection_manager();
    
    let is_in_room = connection_manager.is_user_in_room(room_id, user_id).await;
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(is_in_room)))
} 
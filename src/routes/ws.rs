use crate::services::auth::AuthService;
use actix::Addr;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use rusty_chat::{ws_server::ChatServer, ws_session::ChatSession};

pub async fn ws_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<ChatServer>>,
    auth_service: web::Data<AuthService>,
) -> Result<HttpResponse, Error> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .or_else(|| {
            req.query_string().split('&').find_map(|kv| {
                let mut split = kv.split('=');
                if split.next()? == "token" {
                    split.next()
                } else {
                    None
                }
            })
        });

    let token = match token {
        Some(t) => t,
        None => return Ok(HttpResponse::Unauthorized().body("Missing token")),
    };

    // Validate JWT using AuthService
    let _claims = match auth_service.validate_token(token) {
        Ok(claims) => claims,
        Err(_) => return Ok(HttpResponse::Unauthorized().body("Invalid token")),
    };

    let room_id = req
        .query_string()
        .split('&')
        .find_map(|kv| {
            let mut split = kv.split('=');
            if split.next()? == "room_id" {
                split.next()
            } else {
                None
            }
        })
        .unwrap_or("default")
        .to_string();

    ws::start(
        ChatSession::new(room_id, srv.get_ref().clone()),
        &req,
        stream,
    )
}

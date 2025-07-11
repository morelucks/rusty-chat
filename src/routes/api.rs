use actix_web::{HttpResponse, web};

use crate::handlers;

pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users").service(
            web::resource("")
                .route(web::get().to(handlers::users::index))
                .route(web::head().to(HttpResponse::MethodNotAllowed)),
        ),
    ).service(
        web::scope("/rooms")
        .service(
            web::resource("")
            .route(web::post().to(handlers::rooms::create_room))
            .route(web::get().to(handlers::rooms::get_all_rooms))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
        )
        .service(
            web::resource("/{id}")
            .route(web::get().to(handlers::rooms::get_room_by_id))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),            
        )
    );
}

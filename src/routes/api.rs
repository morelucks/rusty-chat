use actix_web::{HttpResponse, web};

use crate::{handlers, middleware::auth::AuthMiddleware};

pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(
                web::resource("/register")
                    .route(web::post().to(handlers::auth::register))
            )
            .service(
                web::resource("/login")
                    .route(web::post().to(handlers::auth::login))
            )
    )
    .service(
        web::scope("/users").service(
            web::resource("")
                .route(web::get().to(handlers::users::index))
                .route(web::head().to(HttpResponse::MethodNotAllowed)),
        ),
    ).service(
        web::scope("/rooms")
        .service(
            web::resource("")
            .route(web::post().to(handlers::rooms::create_room).wrap(AuthMiddleware))
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

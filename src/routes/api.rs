use actix_web::{HttpResponse, web};

use crate::handlers;

pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users").service(
            web::resource("")
                .route(web::get().to(handlers::users::index))
                .route(web::head().to(HttpResponse::MethodNotAllowed)),
        ),
    );
}

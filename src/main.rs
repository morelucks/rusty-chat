use actix_web::{App, HttpResponse, HttpServer, Responder, get};

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("rusty-chat")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| App::new().service(home)).bind(("127.0.0.1", 8080))?;
    println!("Server is running at http://127.0.0.1:8080 ");
    server.run().await
}

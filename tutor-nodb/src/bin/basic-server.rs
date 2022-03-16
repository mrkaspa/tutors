use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let app = || App::new().configure(general_routes);

    println!("running server");
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Hello")
}

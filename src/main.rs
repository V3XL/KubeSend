mod utils;
mod handlers;
mod services;
mod models;
mod config;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use handlers::message_handler::send_email_endpoint;
use crate::config::Config;

async fn index(req: HttpRequest) -> impl Responder {
    format!("Hello _{}!_!", req.match_info().get("name").unwrap_or("World"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_file("config/config.yaml").expect("Failed to load config.yaml");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .service(web::resource("/").to(index))
            .route("/smtp/send", web::post().to(send_email_endpoint))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

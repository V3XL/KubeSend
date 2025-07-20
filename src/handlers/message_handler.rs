use crate::services::email_service::send_email;
use crate::models::email::SendEmailRequest;
use actix_web::{web, HttpResponse, Responder};
use crate::config::Config;

pub async fn send_email_endpoint(req: web::Json<SendEmailRequest>, config: web::Data<Config>) -> impl Responder {
    match send_email(req.into_inner(), &config.smtp).await {
        Ok(_) => HttpResponse::Ok().body("Email sent successfully!"),
        Err(e) => {
            eprintln!("Failed to send email: {}", e);
            HttpResponse::InternalServerError().body("Failed to send email.")
        }
    }
}
use actix_web::{HttpResponse, Responder};
use actix_web::http::header::ContentType;

pub async fn home() -> impl Responder {
  HttpResponse::Ok()
      .content_type(ContentType::html())
      .body(include_str!("home.html"))
}
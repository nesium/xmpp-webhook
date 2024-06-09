use crate::xmpp::XMPP;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse, Responder};
use jid::BareJid;
use std::str::FromStr;

pub async fn ping(xmpp: web::Data<XMPP>) -> impl Responder {
    xmpp.send_message(BareJid::from_str("marc@prose.org").unwrap(), "It works!")
        .unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body("Message sent.")
}

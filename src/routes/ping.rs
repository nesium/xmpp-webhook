use std::str::FromStr;

use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse, Responder};
use jid::BareJid;

use crate::xmpp::XMPPHandle;

pub async fn ping(xmpp: web::Data<XMPPHandle>) -> impl Responder {
    xmpp.send_message(BareJid::from_str("marc@prose.org").unwrap(), "It works!");

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body("Message sent.")
}

use std::str::FromStr;

use actix_web::{web, HttpResponse, Responder};
use github_webhook::payload_types::Schema;
use jid::BareJid;

use crate::xmpp::XMPPHandle;

pub async fn commit_hook<'a>(xmpp: web::Data<XMPPHandle>, body: web::Bytes) -> impl Responder {
    let payload = match serde_json::from_slice::<Schema>(&body) {
        Ok(payload) => payload,
        Err(err) => {
            return HttpResponse::BadRequest()
                .body(format!("Failed to deserialize payload: {}", err))
        }
    };

    let message = match payload {
        Schema::PushEvent(event) => {
            let commits_markdown: Vec<String> = event
                .commits
                .iter()
                .map(|commit| {
                    format!(
                        "- **Commit**: [{}]({})\n  **Author**: {} <{}>\n  **Message**: {}\n",
                        &commit.id[..7],
                        commit.url,
                        commit.author.name,
                        commit.author.email.unwrap_or("<no email>"),
                        commit.message
                    )
                })
                .collect();

            format!(
                "New commits pushed to repository **{}**:\n\n{}",
                event.repository.name,
                commits_markdown.join("\n")
            )
        }
        _ => return HttpResponse::Ok().body(format!("{:?}", payload)),
    };

    xmpp.send_message(BareJid::from_str("marc@prose.org").unwrap(), message);
    HttpResponse::Ok().body("message sent")
}

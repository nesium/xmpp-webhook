use crate::xmpp::XMPP;
use actix_web::{web, HttpResponse, Responder};
use jid::BareJid;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize)]
struct Commit {
    id: String,
    message: String,
    url: String,
    author: Author,
}

#[derive(Deserialize)]
struct Author {
    name: String,
    email: String,
}

#[derive(Deserialize)]
pub struct Payload {
    repository: Repository,
    commits: Vec<Commit>,
}

#[derive(Deserialize)]
struct Repository {
    name: String,
}

pub async fn commit_hook(xmpp: web::Data<XMPP>, payload: web::Json<Payload>) -> impl Responder {
    let commits_markdown: Vec<String> = payload
        .commits
        .iter()
        .map(|commit| {
            format!(
                "- **Commit**: [{}]({})\n  **Author**: {} <{}>\n  **Message**: {}\n",
                &commit.id[..7],
                commit.url,
                commit.author.name,
                commit.author.email,
                commit.message
            )
        })
        .collect();

    let message = format!(
        "New commits pushed to repository **{}**:\n\n{}",
        payload.repository.name,
        commits_markdown.join("\n")
    );

    xmpp.send_message(BareJid::from_str("marc@prose.org").unwrap(), message)
        .unwrap();

    HttpResponse::Ok().body("Webhook received")
}

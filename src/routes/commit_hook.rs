use std::str::FromStr;

use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder, ResponseError};
use github_webhook::payload_types::PushEvent;
use prose_xmpp::BareJid;

use crate::xmpp::XMPPHandle;

#[derive(thiserror::Error, Debug)]
pub enum WebhookError {
    #[error("Missing event type")]
    MissingEventType,
    #[error(transparent)]
    DeserializationError(#[from] serde_json::Error),
}

impl ResponseError for WebhookError {
    fn status_code(&self) -> StatusCode {
        match self {
            WebhookError::MissingEventType => StatusCode::BAD_REQUEST,
            WebhookError::DeserializationError(_) => StatusCode::BAD_REQUEST,
        }
    }
}

pub async fn commit_hook<'a>(
    req: HttpRequest,
    xmpp: web::Data<XMPPHandle>,
    body: web::Bytes,
) -> Result<impl Responder, WebhookError> {
    let Some(event_type) = req
        .headers()
        .get("X-GitHub-Event")
        .and_then(|val| val.to_str().ok())
    else {
        return Err(WebhookError::MissingEventType);
    };

    let message = match event_type {
        "push" => {
            let payload = serde_json::from_slice::<PushEvent>(&body)?;
            let commits_markdown: Vec<String> = payload
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
                payload.repository.name,
                commits_markdown.join("\n")
            )
        }
        _ => return Ok(HttpResponse::Ok().body("ok")),
    };

    xmpp.send_message(BareJid::from_str("marc@prose.org").unwrap(), message);
    Ok(HttpResponse::Ok().body("message sent"))
}

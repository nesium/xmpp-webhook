use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder, ResponseError};
use github_webhook::payload_types::{IssueCommentEvent, IssuesEvent, PushEvent};

use crate::webhook::{format, RepoMapping};
use crate::xmpp::{RoomId, XMPPHandle};

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

pub async fn webhook(
    req: HttpRequest,
    xmpp: web::Data<XMPPHandle>,
    body: web::Bytes,
    mapping: web::Data<RepoMapping>,
) -> Result<impl Responder, WebhookError> {
    let Some(event_type) = req
        .headers()
        .get("X-GitHub-Event")
        .and_then(|val| val.to_str().ok())
    else {
        return Err(WebhookError::MissingEventType);
    };

    let (repo, message) = match event_type {
        "push" => {
            let event = serde_json::from_slice::<PushEvent>(&body)?;
            (
                event.repository.full_name,
                format::format_push_event(&event),
            )
        }
        "issues" => {
            let event = serde_json::from_slice::<IssuesEvent>(&body)?;
            match event {
                IssuesEvent::Opened(event) => (
                    event.repository.full_name,
                    format::format_issue_opened(&event),
                ),
                IssuesEvent::Closed(event) => (
                    event.repository.full_name,
                    format::format_issue_closed(&event),
                ),
                IssuesEvent::Reopened(event) => (
                    event.repository.full_name,
                    format::format_issue_reopened(&event),
                ),
                _ => return Ok(HttpResponse::Ok().body("ok")),
            }
        }
        "issue_comment" => {
            let event = serde_json::from_slice::<IssueCommentEvent>(&body)?;
            match event {
                IssueCommentEvent::Created(event) => (
                    event.repository.full_name,
                    format::format_issue_comment_created(&event),
                ),
                _ => return Ok(HttpResponse::Ok().body("ok")),
            }
        }
        _ => return Ok(HttpResponse::Ok().body("ok")),
    };

    let Some(jid) = mapping.get(repo) else {
        return Ok(HttpResponse::Ok().body("unknown repo"));
    };

    xmpp.send_message(RoomId::Room(jid.clone()), message);
    Ok(HttpResponse::Ok().body("message sent"))
}

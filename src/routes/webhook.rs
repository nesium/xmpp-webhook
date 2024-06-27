use std::collections::HashMap;
use std::sync::Arc;

use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder, ResponseError};
use minijinja::{context, Environment};
use serde_json::Value;
use tracing::info;

use crate::services::xmpp_service::RoomId;
use crate::services::XMPPService;
use crate::webhook::{RepoMapping, WorkflowRunsStore};

#[derive(thiserror::Error, Debug)]
pub enum WebhookError {
    #[error("Missing event type")]
    MissingEventType,
    #[error("Missing repo")]
    MissingRepo,
    #[error(transparent)]
    DeserializationError(#[from] serde_json::Error),
    #[error(transparent)]
    RenderingError(#[from] minijinja::Error),
}

impl ResponseError for WebhookError {
    fn status_code(&self) -> StatusCode {
        match self {
            WebhookError::MissingEventType => StatusCode::BAD_REQUEST,
            WebhookError::MissingRepo => StatusCode::BAD_REQUEST,
            WebhookError::DeserializationError(_) => StatusCode::BAD_REQUEST,
            WebhookError::RenderingError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub async fn webhook(
    req: HttpRequest,
    xmpp: web::Data<Arc<dyn XMPPService>>,
    body: web::Bytes,
    mapping: web::Data<RepoMapping>,
    workflow_runs: web::Data<WorkflowRunsStore>,
    environment: web::Data<Environment<'_>>,
) -> Result<impl Responder, WebhookError> {
    let Some(event_type) = req
        .headers()
        .get("X-GitHub-Event")
        .and_then(|val| val.to_str().ok())
    else {
        return Err(WebhookError::MissingEventType);
    };

    let event = serde_json::from_slice::<HashMap<String, Value>>(&body)?;
    let repo = event
        .get("repository")
        .and_then(|repo| repo.get("full_name"))
        .and_then(|repo| repo.as_str())
        .ok_or(WebhookError::MissingRepo)?;

    let Some(jid) = mapping.get(&repo) else {
        return Ok(HttpResponse::Ok().body("unknown repo"));
    };

    match event_type {
        "workflow_run" => {
            // We want to send a message for each failed workflow run, for succeeded workflow runs
            // however we only want to send a message if we had a prior identical failed
            // workflow run.

            let workflow_id: u64 = event["workflow_run"]["workflow_id"]
                .as_u64()
                .unwrap_or_default();
            let head_branch = event["workflow_run"]["head_branch"]
                .as_str()
                .unwrap_or_default();

            match event["workflow_run"]["conclusion"]
                .as_str()
                .unwrap_or_default()
            {
                "success" => {
                    if !workflow_runs.workflow_succeeded(repo, workflow_id, head_branch) {
                        return Ok(HttpResponse::Ok().body("ok"));
                    }
                }
                "failure" => workflow_runs.workflow_failed(repo, workflow_id, head_branch),
                _ => {}
            }
        }
        _ => (),
    }

    // Documentation: https://docs.github.com/en/webhooks/webhook-events-and-payloads#issue_comment
    // Payload examples: https://github.com/octokit/webhooks/tree/main/payload-examples

    let template_name = event
        .get("action")
        .and_then(|action| action.as_str())
        .map(|action| format!("{event_type}__{action}"))
        .unwrap_or_else(|| event_type.to_string())
        + ".md";

    let Some(template) = environment.get_template(&template_name).ok() else {
        info!("No template for event {template_name}");
        return Ok(HttpResponse::Ok().body("ok"));
    };

    let message = template.render(context!(event => event))?;

    xmpp.send_message(RoomId::Room(jid.clone()), message);
    Ok(HttpResponse::Ok().body("message sent"))
}

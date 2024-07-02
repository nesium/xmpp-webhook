use crate::helpers::{spawn_app, SentMessage};
use anyhow::Result;
use insta::assert_snapshot;
use reqwest::{Body, StatusCode};

#[tokio::test]
async fn test_push() -> Result<()> {
    let (status, sent_messages) =
        receive_webhook("push", include_str!("fixtures/push.json")).await?;

    assert!(status.is_success());
    assert_snapshot!(sent_messages[0].message);

    Ok(())
}

#[tokio::test]
async fn test_force_push() -> Result<()> {
    let (status, sent_messages) =
        receive_webhook("push", include_str!("fixtures/push_forced.json")).await?;

    assert!(status.is_success());
    assert_snapshot!(sent_messages[0].message);

    Ok(())
}

#[tokio::test]
/// These can happen when pushing a new tag
async fn test_ignores_push_with_empty_commits() -> Result<()> {
    let (status, sent_messages) =
        receive_webhook("push", include_str!("fixtures/push_without_commits.json")).await?;

    assert!(status.is_success());
    assert!(sent_messages.is_empty());

    Ok(())
}

#[tokio::test]
async fn test_issue_opened() -> Result<()> {
    let (status, sent_messages) =
        receive_webhook("issues", include_str!("fixtures/issue_opened.json")).await?;

    assert!(status.is_success());
    assert_snapshot!(sent_messages[0].message);

    Ok(())
}

#[tokio::test]
async fn test_issue_closed() -> Result<()> {
    let (status, sent_messages) =
        receive_webhook("issues", include_str!("fixtures/issue_closed.json")).await?;

    assert!(status.is_success());
    assert_snapshot!(sent_messages[0].message);

    Ok(())
}

#[tokio::test]
async fn test_issue_comment_created() -> Result<()> {
    let (status, sent_messages) = receive_webhook(
        "issue_comment",
        include_str!("fixtures/issue_comment_created.json"),
    )
    .await?;

    assert!(status.is_success());
    assert_snapshot!(sent_messages[0].message);

    Ok(())
}

#[tokio::test]
async fn test_workflow_run_completed_with_failure() -> Result<()> {
    let (status, sent_messages) = receive_webhook(
        "workflow_run",
        include_str!("fixtures/workflow_run_completed_failure.json"),
    )
    .await?;

    assert!(status.is_success());
    assert_snapshot!(sent_messages[0].message);

    Ok(())
}

#[tokio::test]
async fn test_workflow_run_completed_with_success_does_not_send_message() -> Result<()> {
    let (status, sent_messages) = receive_webhook(
        "workflow_run",
        include_str!("fixtures/workflow_run_completed_success.json"),
    )
    .await?;

    assert!(status.is_success());
    assert!(sent_messages.is_empty());

    Ok(())
}

#[tokio::test]
async fn test_workflow_run_completed_with_cancellation_does_not_send_message() -> Result<()> {
    let (status, sent_messages) = receive_webhook(
        "workflow_run",
        include_str!("fixtures/workflow_run_completed_cancelled.json"),
    )
    .await?;

    assert!(status.is_success());
    assert!(sent_messages.is_empty());

    Ok(())
}

#[tokio::test]
async fn test_workflow_run_completed_with_success_sends_message_after_failure() -> Result<()> {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    client
        .post(&format!("{}/webhook", &app.address))
        .header("X-GitHub-Event", "workflow_run")
        .header("Content-Type", "application/json")
        .body(include_str!("fixtures/workflow_run_completed_failure.json"))
        .send()
        .await?;

    assert_snapshot!(app.xmpp.sent_messages()[0].message);

    app.xmpp.reset_sent_messages();

    client
        .post(&format!("{}/webhook", &app.address))
        .header("X-GitHub-Event", "workflow_run")
        .header("Content-Type", "application/json")
        .body(include_str!("fixtures/workflow_run_completed_success.json"))
        .send()
        .await?;

    assert_snapshot!(app.xmpp.sent_messages()[0].message);

    Ok(())
}

#[tokio::test]
async fn test_release_released() -> Result<()> {
    let (status, sent_messages) =
        receive_webhook("release", include_str!("fixtures/release_released.json")).await?;

    assert!(status.is_success());
    assert_snapshot!(sent_messages[0].message);

    Ok(())
}

#[tokio::test]
async fn test_release_prereleased() -> Result<()> {
    let (status, sent_messages) =
        receive_webhook("release", include_str!("fixtures/release_prereleased.json")).await?;

    assert!(status.is_success());
    assert_snapshot!(sent_messages[0].message);

    Ok(())
}

async fn receive_webhook(
    event_type: impl AsRef<str>,
    body: impl Into<Body>,
) -> Result<(StatusCode, Vec<SentMessage>)> {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{}/webhook", &app.address))
        .header("X-GitHub-Event", event_type.as_ref())
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await?;

    Ok((response.status(), app.xmpp.sent_messages()))
}

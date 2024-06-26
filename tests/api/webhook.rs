use anyhow::Result;
use insta::assert_snapshot;

use crate::helpers::spawn_app;

#[tokio::test]
async fn test_push() -> Result<()> {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{}/webhook", &app.address))
        .header("X-GitHub-Event", "push")
        .header("Content-Type", "application/json")
        .body(include_str!("fixtures/push.json"))
        .send()
        .await?;

    assert!(response.status().is_success());
    assert_snapshot!(app.xmpp.sent_messages()[0].message);

    Ok(())
}

#[tokio::test]
async fn test_issue_opened() -> Result<()> {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{}/webhook", &app.address))
        .header("X-GitHub-Event", "issues")
        .header("Content-Type", "application/json")
        .body(include_str!("fixtures/issue_opened.json"))
        .send()
        .await?;

    assert!(response.status().is_success());
    assert_snapshot!(app.xmpp.sent_messages()[0].message);

    Ok(())
}

#[tokio::test]
async fn test_issue_comment_created() -> Result<()> {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{}/webhook", &app.address))
        .header("X-GitHub-Event", "issue_comment")
        .header("Content-Type", "application/json")
        .body(include_str!("fixtures/issue_comment_created.json"))
        .send()
        .await?;

    assert!(response.status().is_success());
    assert_snapshot!(app.xmpp.sent_messages()[0].message);

    Ok(())
}

use insta::assert_snapshot;

use xmpp_webhook::webhook::format::{
    format_issue_comment_created, format_issue_opened, format_push_event,
};
use xmpp_webhook::webhook::types::{IssueCommentCreatedEvent, IssuesOpenedEvent, PushEvent};

#[test]
fn test_push() {
    let contents = include_str!("fixtures/push.json");
    assert_snapshot!(format_push_event(
        &serde_json::from_str::<PushEvent>(contents).unwrap()
    ));
}

#[test]
fn test_issue_opened() {
    let contents = include_str!("fixtures/issue_opened.json");
    assert_snapshot!(format_issue_opened(
        &serde_json::from_str::<IssuesOpenedEvent>(contents).unwrap()
    ));
}

#[test]
fn test_issue_comment_created() {
    let contents = include_str!("fixtures/issue_comment_created.json");
    assert_snapshot!(format_issue_comment_created(
        &serde_json::from_str::<IssueCommentCreatedEvent>(contents).unwrap()
    ));
}

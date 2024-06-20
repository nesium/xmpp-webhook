use github_webhook::payload_types::{IssuesOpenedEvent, PushEvent};
use insta::assert_snapshot;

use xmpp_webhook::webhook::format::{format_issue_opened, format_push_event};

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

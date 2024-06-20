use github_webhook::payload_types::{IssuesOpenedEvent, PushEvent};

pub fn format_push_event(event: &PushEvent) -> String {
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
        "New commits pushed to [{}]({})\n\n{}",
        event.repository.name,
        event.repository.html_url,
        commits_markdown.join("\n")
    )
}

pub fn format_issue_opened(event: &IssuesOpenedEvent) -> String {
    format!(
        r#"[{user}]({user_url}) has opened [issue #{issue_number}]({issue_url}) in [{repo}]({repo_url})

**Title**: {issue_title}
        "#,
        user = event.issue.issue.user.login,
        user_url = event.issue.issue.user.html_url,
        repo = event.repository.name,
        repo_url = event.repository.html_url,
        issue_number = event.issue.issue.number,
        issue_title = event.issue.issue.title,
        issue_url = event.issue.issue.html_url
    )
}

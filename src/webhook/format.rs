use github_webhook::payload_types::{
    IssueCommentCreatedEvent, IssuesClosedEvent, IssuesOpenedEvent, IssuesReopenedEvent, PushEvent,
};

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

pub fn format_issue_closed(event: &IssuesClosedEvent) -> String {
    format!(
        r#"[{user}]({user_url}) has closed [issue #{issue_number}]({issue_url}) in [{repo}]({repo_url})

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

pub fn format_issue_reopened(event: &IssuesReopenedEvent) -> String {
    format!(
        r#"[{user}]({user_url}) has reopened [issue #{issue_number}]({issue_url}) in [{repo}]({repo_url})

**Title**: {issue_title}
        "#,
        user = event.issue.user.login,
        user_url = event.issue.user.html_url,
        repo = event.repository.name,
        repo_url = event.repository.html_url,
        issue_number = event.issue.number,
        issue_title = event.issue.title,
        issue_url = event.issue.html_url
    )
}

pub fn format_issue_comment_created(event: &IssueCommentCreatedEvent) -> String {
    format!(
        r#"[{user}]({user_url}) commented on [issue #{issue_number}]({issue_url}) in [{repo}]({repo_url})

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

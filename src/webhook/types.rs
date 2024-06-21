use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Committer {
    pub name: String,
    pub email: Option<String>,
    pub date: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Commit {
    pub id: String,
    pub tree_id: String,
    pub distinct: bool,
    pub message: String,
    pub timestamp: String,
    pub url: String,
    pub author: Committer,
    pub committer: Committer,
    pub added: Vec<String>,
    pub modified: Vec<String>,
    pub removed: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub login: String,
    pub id: usize,
    pub name: Option<String>,
    pub email: Option<String>,
    pub html_url: String,
    #[serde(rename = "type")]
    pub type_: UserType,
}

#[derive(Debug, Deserialize)]
pub enum UserType {
    Bot,
    Organization,
    User,
}

#[derive(Debug, Deserialize)]
pub struct Label {
    pub id: usize,
    pub url: String,
    pub name: String,
    pub description: Option<String>,
    pub color: String,
    pub default: bool,
}

#[derive(Debug, Deserialize)]
pub struct Organization {
    pub login: String,
    pub id: usize,
    pub html_url: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Repository {
    pub id: usize,
    pub name: String,
    pub full_name: String,
    pub private: bool,
    pub owner: User,
    pub html_url: String,
    pub description: Option<String>,
    pub fork: bool,
    pub url: String,
}

pub type MilestoneState = PullRequestState;
#[derive(Debug, Deserialize)]
pub struct Milestone {
    pub url: String,
    pub html_url: String,
    pub id: usize,
    pub number: usize,
    pub title: String,
    pub description: Option<String>,
    pub creator: User,
    pub open_issues: usize,
    pub closed_issues: usize,
    pub state: MilestoneState,
    pub created_at: String,
    pub updated_at: String,
    pub due_on: Option<String>,
    pub closed_at: Option<String>,
}

pub type IssueState = PullRequestState;

#[derive(Debug, Deserialize)]
pub struct Issue {
    pub url: String,
    pub repository_url: String,
    pub html_url: String,
    pub id: usize,
    pub number: usize,
    pub title: String,
    pub user: User,
    pub labels: Option<Vec<Label>>,
    pub state: Option<IssueState>,
    pub locked: Option<bool>,
    pub assignee: Option<User>,
    pub assignees: Vec<User>,
    pub comments: usize,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub draft: Option<bool>,
    pub body: Option<String>,
    pub state_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct IssueComment {
    pub url: String,
    pub html_url: String,
    pub issue_url: String,
    pub id: usize,
    pub user: User,
    pub created_at: String,
    pub updated_at: String,
    pub body: String,
}

#[derive(Debug, Deserialize)]
pub struct PushEvent {
    #[serde(rename = "ref")]
    pub ref_: String,
    pub before: String,
    pub after: String,
    pub created: bool,
    pub deleted: bool,
    pub forced: bool,
    pub base_ref: Option<String>,
    pub commits: Vec<Commit>,
    pub head_commit: Option<Commit>,
    pub repository: Repository,
    pub pusher: Committer,
    pub sender: User,
    pub organization: Option<Organization>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "action")]
#[serde(rename_all = "snake_case")]
pub enum IssueCommentEvent {
    Created(IssueCommentCreatedEvent),
    Deleted(IssueCommentDeletedEvent),
    Edited(IssueCommentEditedEvent),
}

#[derive(Debug, Deserialize)]
#[serde(tag = "action")]
#[serde(rename_all = "snake_case")]
pub enum IssuesEvent {
    Assigned(IssuesAssignedEvent),
    Closed(IssuesClosedEvent),
    Deleted(IssuesDeletedEvent),
    Demilestoned(IssuesDemilestonedEvent),
    Edited(IssuesEditedEvent),
    Labeled(IssuesLabeledEvent),
    Locked(IssuesLockedEvent),
    Milestoned(IssuesMilestonedEvent),
    Opened(IssuesOpenedEvent),
    Pinned(IssuesPinnedEvent),
    Reopened(IssuesReopenedEvent),
    Transferred(IssuesTransferredEvent),
    Unassigned(IssuesUnassignedEvent),
    Unlabeled(IssuesUnlabeledEvent),
    Unlocked(IssuesUnlockedEvent),
    Unpinned(IssuesUnpinnedEvent),
}

#[derive(Debug, Deserialize)]
pub enum PullRequestState {
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "open")]
    Open,
}

pub type IssueCommentCreatedEventIssueState = PullRequestState;
#[derive(Debug, Deserialize)]
pub struct IssueCommentCreatedEventIssue {
    pub assignee: Option<User>,
    pub state: IssueCommentCreatedEventIssueState,
    pub locked: bool,
    pub labels: Vec<Label>,
    #[serde(flatten)]
    pub issue: Issue,
}
#[derive(Debug, Deserialize)]
pub struct IssueCommentCreatedEvent {
    pub issue: IssueCommentCreatedEventIssue,
    pub comment: IssueComment,
    pub repository: Repository,
    pub sender: User,
    pub organization: Option<Organization>,
}

pub type IssueCommentDeletedEventIssueState = PullRequestState;
#[derive(Debug, Deserialize)]
pub struct IssueCommentDeletedEventIssue {
    pub assignee: Option<User>,
    pub state: IssueCommentDeletedEventIssueState,
    pub locked: bool,
    pub labels: Vec<Label>,
    pub issue: Issue,
}
#[derive(Debug, Deserialize)]
pub struct IssueCommentDeletedEvent {
    pub issue: IssueCommentDeletedEventIssue,
    pub comment: IssueComment,
    pub repository: Repository,
    pub sender: User,
    pub organization: Option<Organization>,
}

pub type IssueCommentEditedEventIssueState = PullRequestState;
#[derive(Debug, Deserialize)]
pub struct IssueCommentEditedEventIssue {
    pub assignee: Option<User>,
    pub state: IssueCommentEditedEventIssueState,
    pub locked: bool,
    pub labels: Vec<Label>,
    #[serde(flatten)]
    pub issue: Issue,
}

#[derive(Debug, Deserialize)]
pub struct IssueCommentEditedEvent {
    pub issue: IssueCommentEditedEventIssue,
    pub comment: IssueComment,
    pub repository: Repository,
    pub sender: User,
    pub organization: Option<Organization>,
}

#[derive(Debug, Deserialize)]
pub struct IssuesAssignedEvent {
    pub issue: Issue,
    pub assignee: Option<User>,
    pub repository: Repository,
    pub sender: User,
    pub organization: Option<Organization>,
}
#[derive(Debug, Deserialize)]
pub struct IssuesClosedEventIssue {
    pub closed_at: String,
    #[serde(flatten)]
    pub issue: Issue,
}
#[derive(Debug, Deserialize)]
pub struct IssuesClosedEvent {
    pub issue: IssuesClosedEventIssue,
    pub repository: Repository,
    pub sender: User,
    pub organization: Option<Organization>,
}
#[derive(Debug, Deserialize)]
pub struct IssuesDeletedEvent {
    pub issue: Issue,
    pub repository: Repository,
    pub sender: User,
    pub organization: Option<Organization>,
}
#[derive(Debug, Deserialize)]
pub struct IssuesDemilestonedEventIssue {
    #[serde(flatten)]
    pub issue: Issue,
}
#[derive(Debug, Deserialize)]
pub struct IssuesDemilestonedEvent {
    pub issue: IssuesDemilestonedEventIssue,
    pub milestone: Milestone,
    pub repository: Repository,
    pub sender: User,
    pub organization: Option<Organization>,
}
#[derive(Debug, Deserialize)]
pub struct IssuesEditedEventChangesBody {
    pub from: String,
}
#[derive(Debug, Deserialize)]
pub struct IssuesEditedEventChangesTitle {
    pub from: String,
}
#[derive(Debug, Deserialize)]
pub struct IssuesEditedEventChanges {
    pub body: Option<IssuesEditedEventChangesBody>,
    pub title: Option<IssuesEditedEventChangesTitle>,
}
#[derive(Debug, Deserialize)]
pub struct IssuesEditedEvent {
    pub issue: Issue,
    pub label: Option<Label>,
    pub changes: IssuesEditedEventChanges,
    pub repository: Repository,
    pub sender: User,
    pub organization: Option<Organization>,
}
#[derive(Debug, Deserialize)]
pub struct IssuesLabeledEvent {
    pub issue: Issue,
    pub label: Option<Label>,
    pub repository: Repository,
    pub sender: User,
    pub organization: Option<Organization>,
}

#[derive(Debug, Deserialize)]
pub enum PullRequestActiveLockReason {
    #[serde(rename = "off-topic")]
    OffTopic,
    #[serde(rename = "resolved")]
    Resolved,
    #[serde(rename = "spam")]
    Spam,
    #[serde(rename = "too heated")]
    TooHeated,
}

pub type IssuesLockedEventIssueActiveLockReason = PullRequestActiveLockReason;
#[derive(Debug, Deserialize)]
pub struct IssuesLockedEventIssue {
    pub active_lock_reason: Option<IssuesLockedEventIssueActiveLockReason>,
    #[serde(flatten)]
    pub issue: Issue,
}
#[derive(Debug, Deserialize)]
pub struct IssuesLockedEvent {
    pub issue: IssuesLockedEventIssue,
    pub repository: Repository,
    pub sender: User,
    pub organization: Option<Organization>,
}
#[derive(Debug, Deserialize)]
pub struct IssuesMilestonedEventIssue {
    pub milestone: Milestone,
    #[serde(flatten)]
    pub issue: Issue,
}
#[derive(Debug, Deserialize)]
pub struct IssuesMilestonedEvent {
    pub issue: IssuesMilestonedEventIssue,
    pub milestone: Milestone,
    pub repository: Repository,
    pub sender: User,
    pub organization: Option<Organization>,
}
#[derive(Debug, Deserialize)]
pub struct IssuesOpenedEventChanges {
    pub old_issue: Issue,
    pub old_repository: Repository,
}
#[derive(Debug, Deserialize)]
pub struct IssuesOpenedEventIssue {
    pub closed_at: (),
    #[serde(flatten)]
    pub issue: Issue,
}
#[derive(Debug, Deserialize)]
pub struct IssuesOpenedEvent {
    pub changes: Option<IssuesOpenedEventChanges>,
    pub issue: IssuesOpenedEventIssue,
    pub repository: Repository,
    pub sender: User,
    pub organization: Option<Organization>,
}
#[derive(Debug, Deserialize)]
pub struct IssuesPinnedEvent {
    pub issue: Issue,
    pub repository: Repository,
    pub sender: User,
    pub organization: Option<Organization>,
}
pub type IssuesReopenedEventIssue = Issue;
#[derive(Debug, Deserialize)]
pub struct IssuesReopenedEvent {
    pub issue: IssuesReopenedEventIssue,
    pub repository: Repository,
    pub sender: User,
    pub organization: Option<Organization>,
}
#[derive(Debug, Deserialize)]
pub struct IssuesTransferredEventChanges {
    pub new_issue: Issue,
    pub new_repository: Repository,
}
#[derive(Debug, Deserialize)]
pub struct IssuesTransferredEvent {
    pub changes: IssuesTransferredEventChanges,
    pub issue: Issue,
    pub repository: Repository,
    pub sender: User,
    pub organization: Option<Organization>,
}
#[derive(Debug, Deserialize)]
pub struct IssuesUnassignedEvent {
    pub issue: Issue,
    pub assignee: Option<User>,
    pub repository: Repository,
    pub sender: User,
    pub organization: Option<Organization>,
}
#[derive(Debug, Deserialize)]
pub struct IssuesUnlabeledEvent {
    pub issue: Issue,
    pub label: Option<Label>,
    pub repository: Repository,
    pub sender: User,
    pub organization: Option<Organization>,
}
#[derive(Debug, Deserialize)]
pub struct IssuesUnlockedEventIssue {
    #[serde(flatten)]
    pub issue: Issue,
}
#[derive(Debug, Deserialize)]
pub struct IssuesUnlockedEvent {
    pub issue: IssuesUnlockedEventIssue,
    pub repository: Repository,
    pub sender: User,
    pub organization: Option<Organization>,
}
#[derive(Debug, Deserialize)]
pub struct IssuesUnpinnedEvent {
    pub issue: Issue,
    pub repository: Repository,
    pub sender: User,
    pub organization: Option<Organization>,
}

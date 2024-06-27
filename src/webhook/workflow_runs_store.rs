use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// A store for managing workflow runs associated with repositories.
#[derive(Debug, Clone)]
pub struct WorkflowRunsStore(Arc<Mutex<HashMap<String, Vec<WorkflowRun>>>>);

impl WorkflowRunsStore {
    pub fn new() -> Self {
        Self(Default::default())
    }

    /// Records a failed workflow run for the specified repository. If the run is already recorded,
    /// it does not add a duplicate.
    ///
    /// # Arguments
    /// * `repo` - The repository name.
    /// * `workflow_id` - A unique identifier for the workflow run.
    /// * `head_branch` - The branch name at the head during the workflow run.
    pub fn workflow_failed(
        &self,
        repo: impl Into<String>,
        workflow_id: u64,
        head_branch: impl Into<String>,
    ) {
        let run = WorkflowRun {
            workflow_id,
            head_branch: head_branch.into(),
        };
        let mut map = self.0.lock().unwrap();
        let runs = map.entry(repo.into()).or_default();

        if runs.contains(&run) {
            return;
        }

        runs.push(run);
    }

    /// Removes workflow runs from the store that match the specified `workflow_id` and `head_branch`.
    /// Returns `true` if any runs were removed, otherwise `false`.
    ///
    /// # Arguments
    /// * `repo` - The repository name.
    /// * `workflow_id` - The unique identifier for the workflow to be removed.
    /// * `head_branch` - The branch name at the head during the workflow run.
    pub fn workflow_succeeded(
        &self,
        repo: impl AsRef<str>,
        workflow_id: u64,
        head_branch: impl AsRef<str>,
    ) -> bool {
        let mut map = self.0.lock().unwrap();
        let Some(runs) = map.get_mut(repo.as_ref()) else {
            return false;
        };

        let runs_len = runs.len();
        let head_branch = head_branch.as_ref();
        runs.retain(|run| run.workflow_id != workflow_id || &run.head_branch != head_branch);

        runs.len() != runs_len
    }
}

#[derive(Debug, PartialEq)]
struct WorkflowRun {
    workflow_id: u64,
    head_branch: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_store_is_empty() {
        let store = WorkflowRunsStore::new();
        let map = store.0.lock().unwrap();
        assert!(map.is_empty());
    }

    #[test]
    fn test_workflow_failed_adds_run() {
        let store = WorkflowRunsStore::new();
        store.workflow_failed("repo1", 1, "main");
        let map = store.0.lock().unwrap();
        assert_eq!(map.get("repo1").unwrap().len(), 1);
    }

    #[test]
    fn test_workflow_failed_does_not_add_duplicate() {
        let store = WorkflowRunsStore::new();
        store.workflow_failed("repo1", 1, "main");
        store.workflow_failed("repo1", 1, "main"); // Attempt to add duplicate
        let map = store.0.lock().unwrap();
        assert_eq!(map.get("repo1").unwrap().len(), 1);
    }

    #[test]
    fn test_workflow_succeeded_removes_correct_run() {
        let store = WorkflowRunsStore::new();
        store.workflow_failed("repo1", 1, "main");
        store.workflow_failed("repo1", 2, "dev");
        let removed = store.workflow_succeeded("repo1", 1, "main");
        let map = store.0.lock().unwrap();
        assert!(removed);
        assert_eq!(map.get("repo1").unwrap().len(), 1);
        assert_eq!(map.get("repo1").unwrap()[0].workflow_id, 2);
    }

    #[test]
    fn test_workflow_succeeded_returns_false_if_no_match() {
        let store = WorkflowRunsStore::new();
        store.workflow_failed("repo1", 1, "main");
        let removed = store.workflow_succeeded("repo1", 999, "main");
        assert!(!removed);
    }
}

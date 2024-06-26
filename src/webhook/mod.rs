use std::collections::HashMap;
use std::sync::Arc;

use prose_xmpp::BareJid;

use crate::config::RepoSettings;

#[derive(Debug, Clone)]
pub struct RepoMapping(Arc<HashMap<String, BareJid>>);

impl RepoMapping {
    pub fn new(mapping: Vec<RepoSettings>) -> Self {
        Self(Arc::new(
            mapping.into_iter().map(|m| (m.repo, m.room)).collect(),
        ))
    }

    pub fn get(&self, room: &str) -> Option<&BareJid> {
        self.0.get(room)
    }
}

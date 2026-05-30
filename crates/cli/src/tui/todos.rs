use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// A single todo item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: u64,
    pub text: String,
    pub done: bool,
}

/// Persisted todo data for one session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoData {
    pub items: Vec<TodoItem>,
    pub next_id: u64,
}

impl TodoData {
    pub fn empty() -> Self {
        Self {
            items: Vec::new(),
            next_id: 1,
        }
    }

    pub fn count_total(&self) -> usize {
        self.items.len()
    }

    pub fn count_done(&self) -> usize {
        self.items.iter().filter(|i| i.done).count()
    }

    pub fn all_done(&self) -> bool {
        !self.items.is_empty() && self.items.iter().all(|i| i.done)
    }
}

/// Build the session-scoped todos file path.
pub fn todos_path(workspace_root: &Path, session_id: &str) -> PathBuf {
    workspace_root
        .join(".nca")
        .join("sessions")
        .join(format!("{session_id}.todos.json"))
}

/// Load todos for a given session. Returns empty if file missing or corrupt.
pub fn load_todos(workspace_root: &Path, session_id: &str) -> TodoData {
    let path = todos_path(workspace_root, session_id);
    fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_else(TodoData::empty)
}

/// Save todos for a given session.
pub fn save_todos(workspace_root: &Path, session_id: &str, data: &TodoData) {
    let path = todos_path(workspace_root, session_id);
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string_pretty(data) {
        let _ = fs::write(&path, json);
    }
}

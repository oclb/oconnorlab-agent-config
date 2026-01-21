#![allow(dead_code)]

use serde::{Deserialize, Serialize};

/// Request for git pull command
#[derive(Debug, Serialize, Deserialize)]
pub struct GitPullRequest {
    /// Directory containing the git repository
    pub path: String,
    /// Remote name (default: origin)
    #[serde(default = "default_remote")]
    pub remote: String,
    /// Branch name (default: current branch)
    pub branch: Option<String>,
}

fn default_remote() -> String {
    "origin".to_string()
}

/// Response from git pull command
#[derive(Debug, Serialize, Deserialize)]
pub struct GitPullResponse {
    pub path: String,
    pub output: String,
    pub already_up_to_date: bool,
    pub files_changed: usize,
    pub duration_ms: u64,
}

/// Parse git pull output to extract info
pub fn parse_git_pull_output(output: &str) -> (bool, usize) {
    let already_up_to_date =
        output.contains("Already up to date") || output.contains("Already up-to-date");

    // Count files changed from output like "2 files changed, 10 insertions(+)"
    let files_changed = output
        .lines()
        .find(|line| line.contains("files changed") || line.contains("file changed"))
        .and_then(|line| line.split_whitespace().next().and_then(|n| n.parse().ok()))
        .unwrap_or(0);

    (already_up_to_date, files_changed)
}

/// Request for git status command
#[derive(Debug, Serialize, Deserialize)]
pub struct GitStatusRequest {
    /// Directory containing the git repository
    pub path: String,
}

/// Response from git status command
#[derive(Debug, Serialize, Deserialize)]
pub struct GitStatusResponse {
    pub path: String,
    pub branch: String,
    pub clean: bool,
    pub ahead: usize,
    pub behind: usize,
    pub modified: Vec<String>,
    pub untracked: Vec<String>,
    pub duration_ms: u64,
}

/// Parse git status --porcelain output
pub fn parse_git_status_output(output: &str) -> (Vec<String>, Vec<String>) {
    let mut modified = Vec::new();
    let mut untracked = Vec::new();

    for line in output.lines() {
        if line.len() < 3 {
            continue;
        }
        let status = &line[0..2];
        let file = line[3..].to_string();

        if status == "??" {
            untracked.push(file);
        } else {
            modified.push(file);
        }
    }

    (modified, untracked)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_git_pull_up_to_date() {
        let output = "Already up to date.";
        let (up_to_date, files) = parse_git_pull_output(output);
        assert!(up_to_date);
        assert_eq!(files, 0);
    }

    #[test]
    fn test_parse_git_pull_with_changes() {
        let output = r#"Updating abc123..def456
Fast-forward
 file1.py | 10 +++++++---
 file2.py |  5 +++++
 2 files changed, 12 insertions(+), 3 deletions(-)"#;
        let (up_to_date, files) = parse_git_pull_output(output);
        assert!(!up_to_date);
        assert_eq!(files, 2);
    }

    #[test]
    fn test_parse_git_status() {
        let output = r#" M src/main.rs
?? new_file.txt
MM both_modified.rs"#;
        let (modified, untracked) = parse_git_status_output(output);
        assert_eq!(modified.len(), 2);
        assert_eq!(untracked.len(), 1);
        assert!(untracked.contains(&"new_file.txt".to_string()));
    }
}

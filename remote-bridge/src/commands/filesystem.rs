use serde::{Deserialize, Serialize};

/// Flags for ls command - type-safe, no arbitrary flags
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LsFlag {
    Long,        // -l
    All,         // -a (include hidden)
    Human,       // -h (human-readable sizes)
    Recursive,   // -R
    SortByTime,  // -t
    SortBySize,  // -S
}

impl LsFlag {
    pub fn to_arg(&self) -> &'static str {
        match self {
            LsFlag::Long => "-l",
            LsFlag::All => "-a",
            LsFlag::Human => "-h",
            LsFlag::Recursive => "-R",
            LsFlag::SortByTime => "-t",
            LsFlag::SortBySize => "-S",
        }
    }
}

/// Request for ls command
#[derive(Debug, Serialize, Deserialize)]
pub struct LsRequest {
    pub path: String,
    #[serde(default)]
    pub flags: Vec<LsFlag>,
}

/// Response from ls command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LsResponse {
    pub entries: Vec<DirEntry>,
    pub path: String,
    pub duration_ms: u64,
}

/// Directory entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirEntry {
    pub name: String,
    pub entry_type: EntryType,
    pub size: Option<u64>,
    pub modified: Option<String>,
    pub permissions: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryType {
    File,
    Directory,
    Symlink,
    Other,
}

/// Parse ls output into structured entries
pub fn parse_ls_output(output: &str, long_format: bool) -> Vec<DirEntry> {
    let mut entries = Vec::new();

    for line in output.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("total") {
            continue;
        }

        if long_format {
            // Parse long format: -rw-r--r-- 1 user group 1234 Jan 1 12:00 filename
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 9 {
                let perms = parts[0];
                let entry_type = match perms.chars().next() {
                    Some('d') => EntryType::Directory,
                    Some('l') => EntryType::Symlink,
                    Some('-') => EntryType::File,
                    _ => EntryType::Other,
                };

                let size = parts[4].parse().ok();
                let name = parts[8..].join(" ");

                entries.push(DirEntry {
                    name,
                    entry_type,
                    size,
                    modified: Some(format!("{} {} {}", parts[5], parts[6], parts[7])),
                    permissions: Some(perms.to_string()),
                });
            }
        } else {
            // Simple format: just filename
            entries.push(DirEntry {
                name: line.to_string(),
                entry_type: EntryType::Other, // Unknown without -l
                size: None,
                modified: None,
                permissions: None,
            });
        }
    }

    entries
}

/// Flags for cat command
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CatFlag {
    NumberLines, // -n
}

/// Request for cat command
#[derive(Debug, Serialize, Deserialize)]
pub struct CatRequest {
    pub path: String,
    #[serde(default)]
    pub flags: Vec<CatFlag>,
    pub head: Option<usize>,   // Only return first N lines
    pub tail: Option<usize>,   // Only return last N lines
    pub offset: Option<usize>, // Start from line N
    pub limit: Option<usize>,  // Number of lines to return
}

/// Response from cat command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatResponse {
    pub content: String,
    pub path: String,
    pub total_lines: usize,
    pub truncated: bool,
    pub duration_ms: u64,
}

/// Flags for grep command
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GrepFlag {
    IgnoreCase,       // -i
    Recursive,        // -r
    LineNumbers,      // -n
    FilesWithMatches, // -l
    Count,            // -c
    InvertMatch,      // -v
    WholeWord,        // -w
}

impl GrepFlag {
    pub fn to_arg(&self) -> &'static str {
        match self {
            GrepFlag::IgnoreCase => "-i",
            GrepFlag::Recursive => "-r",
            GrepFlag::LineNumbers => "-n",
            GrepFlag::FilesWithMatches => "-l",
            GrepFlag::Count => "-c",
            GrepFlag::InvertMatch => "-v",
            GrepFlag::WholeWord => "-w",
        }
    }
}

/// Request for grep command
#[derive(Debug, Serialize, Deserialize)]
pub struct GrepRequest {
    pub pattern: String,
    pub paths: Vec<String>,
    #[serde(default)]
    pub flags: Vec<GrepFlag>,
}

/// Response from grep command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrepResponse {
    pub matches: Vec<GrepMatch>,
    pub total_matches: usize,
    pub files_searched: usize,
    pub duration_ms: u64,
}

/// A single grep match
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrepMatch {
    pub file: String,
    pub line_number: Option<usize>,
    pub content: String,
}

/// Parse grep output into structured matches
pub fn parse_grep_output(output: &str, with_line_numbers: bool) -> Vec<GrepMatch> {
    let mut matches = Vec::new();

    for line in output.lines() {
        if line.is_empty() {
            continue;
        }

        if with_line_numbers {
            // Format: filename:linenum:content
            let parts: Vec<&str> = line.splitn(3, ':').collect();
            if parts.len() >= 3 {
                matches.push(GrepMatch {
                    file: parts[0].to_string(),
                    line_number: parts[1].parse().ok(),
                    content: parts[2].to_string(),
                });
            } else if parts.len() == 2 {
                // No line number
                matches.push(GrepMatch {
                    file: parts[0].to_string(),
                    line_number: None,
                    content: parts[1].to_string(),
                });
            }
        } else {
            // Format: filename:content or just content
            if let Some((file, content)) = line.split_once(':') {
                matches.push(GrepMatch {
                    file: file.to_string(),
                    line_number: None,
                    content: content.to_string(),
                });
            } else {
                matches.push(GrepMatch {
                    file: String::new(),
                    line_number: None,
                    content: line.to_string(),
                });
            }
        }
    }

    matches
}

/// Maximum file size for download (1 MB)
pub const MAX_DOWNLOAD_SIZE: u64 = 1024 * 1024;

/// Request for download command
#[derive(Debug, Serialize, Deserialize)]
pub struct DownloadRequest {
    pub path: String,
}

/// Response from download command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadResponse {
    pub path: String,
    pub content: String,       // base64-encoded
    pub size_bytes: u64,
    pub duration_ms: u64,
}

/// Error when file is too large for download
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadTooLargeError {
    pub path: String,
    pub size_bytes: u64,
    pub max_bytes: u64,
    pub scp_command: String,
}

/// Flags for find command
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindType {
    File,       // -type f
    Directory,  // -type d
    Symlink,    // -type l
}

impl FindType {
    pub fn to_arg(&self) -> &'static str {
        match self {
            FindType::File => "f",
            FindType::Directory => "d",
            FindType::Symlink => "l",
        }
    }
}

/// Request for find command
#[derive(Debug, Serialize, Deserialize)]
pub struct FindRequest {
    pub path: String,
    /// Name pattern (e.g., "*.py")
    pub name: Option<String>,
    /// File type filter
    pub file_type: Option<FindType>,
    /// Max depth to search
    pub max_depth: Option<u32>,
    /// Max results to return
    #[serde(default = "default_find_limit")]
    pub limit: usize,
}

fn default_find_limit() -> usize {
    1000
}

/// Response from find command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindResponse {
    pub files: Vec<String>,
    pub total_found: usize,
    pub truncated: bool,
    pub duration_ms: u64,
}

/// Request for wc command
#[derive(Debug, Serialize, Deserialize)]
pub struct WcRequest {
    pub path: String,
    /// Count lines only (-l)
    #[serde(default)]
    pub lines_only: bool,
    /// Count words only (-w)
    #[serde(default)]
    pub words_only: bool,
    /// Count bytes only (-c)
    #[serde(default)]
    pub bytes_only: bool,
}

/// Response from wc command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WcResponse {
    pub path: String,
    pub lines: Option<u64>,
    pub words: Option<u64>,
    pub bytes: Option<u64>,
    pub duration_ms: u64,
}

/// Request for head command
#[derive(Debug, Serialize, Deserialize)]
pub struct HeadRequest {
    pub path: String,
    /// Number of lines (default 10)
    #[serde(default = "default_head_lines")]
    pub lines: usize,
}

fn default_head_lines() -> usize {
    10
}

/// Response from head command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadResponse {
    pub path: String,
    pub content: String,
    pub lines_returned: usize,
    pub duration_ms: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ls_long() {
        let output = r#"total 8
-rw-r--r-- 1 user group 1234 Jan 15 12:00 file.txt
drwxr-xr-x 2 user group 4096 Jan 14 10:30 subdir
"#;

        let entries = parse_ls_output(output, true);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].name, "file.txt");
        assert!(matches!(entries[0].entry_type, EntryType::File));
        assert_eq!(entries[0].size, Some(1234));

        assert_eq!(entries[1].name, "subdir");
        assert!(matches!(entries[1].entry_type, EntryType::Directory));
    }

    #[test]
    fn test_parse_grep_with_line_numbers() {
        let output = r#"file.py:10:def main():
file.py:25:    return result
"#;

        let matches = parse_grep_output(output, true);
        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].file, "file.py");
        assert_eq!(matches[0].line_number, Some(10));
        assert_eq!(matches[0].content, "def main():");
    }
}

use std::time::Duration;

/// A simple struct that represents a file.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct File {
    pub created_at: Duration,
    pub extension: String,
    pub modified_at: Duration,
    pub name: String,
    pub path: String,
}

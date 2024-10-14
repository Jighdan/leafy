use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Directory {
    pub path: String,
    pub children: Option<Vec<Directory>>,
    pub file_extension: Option<String>,
    pub item_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryContent {
    pub path: String,
    pub file_extension: Option<String>,
    pub item_type: String,
    pub content: Option<String>,
    pub media_content: Option<String>,
    pub mime_type: Option<String>,
    pub size: u64,
    pub last_modified: String,
}
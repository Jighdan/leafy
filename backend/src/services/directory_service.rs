use std::fs;
use std::path::{Path, PathBuf};
use rocket::http::Status;
use base64::{Engine as _, engine::general_purpose};
use crate::models::directory::{Directory, DirectoryContent};

pub struct DirectoryService;

impl DirectoryService {
    pub fn new() -> Self {
        DirectoryService
    }

    pub async fn list(&self) -> Result<Vec<Directory>, Status> {
        let path = PathBuf::from("./content");
        Ok(self.read_directory(&path))
    }

    pub async fn get(&self, path: &Path) -> Result<DirectoryContent, Status> {
        let full_path = PathBuf::from("./content").join(path);
    
        if !full_path.exists() {
            return Err(Status::NotFound);
        }

        let relative_path = full_path.strip_prefix("./content/").unwrap_or(&full_path);
        let file_name = relative_path.to_string_lossy().into_owned();
        let metadata = fs::metadata(&full_path).map_err(|_| Status::InternalServerError)?;
        let size = metadata.len();
        let last_modified = metadata.modified()
            .map(|time| time.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs().to_string())
            .unwrap_or_else(|_| "Unknown".to_string());

        let (item_type, file_extension, mime_type) = if metadata.is_dir() {
            ("directory".to_string(), None, None)
        } else {
            let extension = full_path.extension().and_then(|e| e.to_str()).map(|s| s.to_string());
            let item_type = self.determine_item_type(extension.as_deref().unwrap_or(""));
            let mime = if item_type != "other" {
                Some(mime_guess::from_path(&full_path).first_or_octet_stream().essence_str().to_string())
            } else {
                None
            };
            (item_type, extension, mime)
        };

        let mut file_content = DirectoryContent {
            path: file_name,
            file_extension,
            item_type: item_type.clone(),
            content: None,
            media_content: None,
            mime_type,
            size,
            last_modified,
        };

        if !metadata.is_dir() {
            match item_type.as_str() {
                "markdown" => {
                    let content = fs::read_to_string(&full_path).map_err(|_| Status::InternalServerError)?;
                    file_content.content = Some(content);
                },
                "media" => {
                    let content = fs::read(&full_path).map_err(|_| Status::InternalServerError)?;
                    let base64_content = general_purpose::STANDARD.encode(content);
                    file_content.media_content = Some(base64_content);
                },
                _ => {
                    // For other types, we're already including metadata (size and last_modified)
                    // No need to add content
                }
            }
        }

        Ok(file_content)
    }

    fn read_directory(&self, path: &PathBuf) -> Vec<Directory> {
        let mut files = Vec::new();
        let ignored_files = vec![".git", ".gitlab-ci.yml"];

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.filter_map(Result::ok) {
                let file_path = entry.path();
                let relative_path = file_path.strip_prefix("./content/").unwrap_or(&file_path);
                let file_name = relative_path.to_string_lossy().into_owned();

                if ignored_files.contains(&file_name.as_str()) {
                    continue;
                }

                let mut file_info = Directory {
                    path: file_name,
                    children: None,
                    file_extension: None,
                    item_type: "directory".to_string(),
                };

                if file_path.is_dir() {
                    file_info.children = Some(self.read_directory(&file_path));
                } else if let Some(extension) = file_path.extension() {
                    file_info.file_extension = Some(extension.to_string_lossy().into_owned());
                    file_info.item_type = self.determine_item_type(&file_info.file_extension.as_ref().unwrap());
                }

                files.push(file_info);
            }
        }

        files
    }

    fn determine_item_type(&self, extension: &str) -> String {
        match extension.to_lowercase().as_str() {
            "md" => "markdown".to_string(),
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "mp4" | "avi" | "mov" => "media".to_string(),
            "" => "directory".to_string(),
            _ => "other".to_string(),
        }
    }
}
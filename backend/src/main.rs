use std::fs;
use std::path::PathBuf;
use warp::Filter;
use serde::Serialize;
use base64::{Engine as _, engine::general_purpose};

#[derive(Serialize)]
struct FileInfo {
    path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Vec<FileInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_extension: Option<String>,
    item_type: String,
}

#[derive(Serialize)]
struct FileContent {
    path: String,
    file_extension: Option<String>,
    item_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    media_content: Option<String>,
    size: u64,
    last_modified: String,
}

#[tokio::main]
async fn main() {
    let root = warp::path::end().map(|| "Welcome to the Leafy Backend!");

    let directory_list = warp::path("directory")
        .and(warp::path::end())
        .and_then(list_directory);

    let file_content = warp::path("directory")
        .and(warp::path::tail())
        .and_then(get_file_content);

    let routes = root
        .or(directory_list)
        .or(file_content);

    println!("Server started at http://localhost:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn list_directory() -> Result<impl warp::Reply, warp::Rejection> {
    let path = PathBuf::from("./content");
    let file_info = read_directory(&path);

    Ok(warp::reply::json(&file_info))
}

async fn get_file_content(path: warp::path::Tail) -> Result<impl warp::Reply, warp::Rejection> {
    let full_path = PathBuf::from("./content").join(path.as_str());
    
    if !full_path.exists() {
        return Err(warp::reject::not_found());
    }

    let relative_path = full_path.strip_prefix("./content/").unwrap_or(&full_path);
    let file_name = relative_path.to_string_lossy().into_owned();
    let metadata = fs::metadata(&full_path).map_err(|_| warp::reject::not_found())?;
    let size = metadata.len();
    let last_modified = metadata.modified()
        .map(|time| time.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs().to_string())
        .unwrap_or_else(|_| "Unknown".to_string());

    let (item_type, file_extension, mime_type) = if metadata.is_dir() {
        ("directory".to_string(), None, None)
    } else {
        let extension = full_path.extension().and_then(|e| e.to_str()).map(|s| s.to_string());
        let item_type = determine_item_type(extension.as_deref().unwrap_or(""));
        let mime = if item_type != "other" {
            Some(mime_guess::from_path(&full_path).first_or_octet_stream().essence_str().to_string())
        } else {
            None
        };
        (item_type, extension, mime)
    };

    let mut file_content = FileContent {
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
                let content = fs::read_to_string(&full_path).map_err(|_| warp::reject::not_found())?;
                file_content.content = Some(content);
            },
            "media" => {
                let content = fs::read(&full_path).map_err(|_| warp::reject::not_found())?;
                let base64_content = general_purpose::STANDARD.encode(content);
                file_content.media_content = Some(base64_content);
            },
            _ => {
                // For other types, we're already including metadata (size and last_modified)
                // No need to add content
            }
        }
    }

    Ok(warp::reply::json(&file_content))
}

fn read_directory(path: &PathBuf) -> Vec<FileInfo> {
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

            let mut file_info = FileInfo {
                path: file_name,
                children: None,
                file_extension: None,
                item_type: "directory".to_string(),
            };

            if file_path.is_dir() {
                file_info.children = Some(read_directory(&file_path));
            } else if let Some(extension) = file_path.extension() {
                file_info.file_extension = Some(extension.to_string_lossy().into_owned());
                file_info.item_type = determine_item_type(&file_info.file_extension.as_ref().unwrap());
            }

            files.push(file_info);
        }
    }

    files
}

fn determine_item_type(extension: &str) -> String {
    match extension.to_lowercase().as_str() {
        "md" => "markdown".to_string(),
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "mp4" | "avi" | "mov" => "media".to_string(),
        "" => "directory".to_string(),
        _ => "other".to_string(),
    }
}
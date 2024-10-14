use rocket::serde::json::Json;
use rocket::State;
use rocket::http::Status;
use std::path::PathBuf;
use crate::models::directory::{Directory, DirectoryContent};
use crate::services::directory_service::DirectoryService;

#[get("/")]
pub fn index() -> &'static str {
    "Welcome to the Leafy Backend!"
}

#[get("/directory")]
pub async fn list_directory(directory_service: &State<DirectoryService>) -> Result<Json<Vec<Directory>>, Status> {
    directory_service.list().await.map(Json)
}

#[get("/directory/<path..>")]
pub async fn get_file_content(path: PathBuf, directory_service: &State<DirectoryService>) -> Result<Json<DirectoryContent>, Status> {
    directory_service.get(&path).await.map(Json)
}

pub fn routes() -> Vec<rocket::Route> {
    routes![index, list_directory, get_file_content]
}
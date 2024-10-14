#[macro_use] extern crate rocket;

mod routes;
mod models;
mod services;

use services::directory_service::DirectoryService;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes::directory::routes())
        .manage(DirectoryService::new())
}
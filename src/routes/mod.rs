pub mod users;

use crate::response::{GenericResponse, ResponseMessage};
use rocket::{get, http::Status, serde::json::Json};

#[get("/healthchecker")]
pub async fn health_checker_handler() -> Result<Json<GenericResponse>, Status> {
    const MESSAGE: &str = "Build Simple CRUD API with Rust and Rocket";

    let response_json = GenericResponse {
        status: ResponseMessage::Success,
        message: MESSAGE.to_string(),
    };
    Ok(Json(response_json))
}

#[get("/")]
pub async fn welcome() -> Result<Json<GenericResponse>, Status> {
    const MESSAGE: &str = "Welcome to Epic mail Rust";

    let response_json = GenericResponse {
        status: ResponseMessage::Success,
        message: MESSAGE.to_string(),
    };
    Ok(Json(response_json))
}

use crate::response::{GenericResponse, ResponseMessage};
use rocket::{http::Status, serde::json::Json};

#[post("/signup")]
async fn signup() -> Result<Json<GenericResponse>, Status> {
    const MESSAGE: &str = "Build Simple CRUD API with Rust and Rocket";

    let response_json = GenericResponse {
        status: ResponseMessage::Success,
        message: MESSAGE.to_string(),
    };
    Ok(Json(response_json))
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![signup]
}

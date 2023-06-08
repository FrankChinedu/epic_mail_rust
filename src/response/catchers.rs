use super::*;
use rocket::{http::Status, serde::json::Json};

#[catch(404)]
pub fn general_not_found() -> Result<Json<GenericResponse>, Status> {
    Ok(Json(GenericResponse {
        message: "Route not Found".to_string(),
        status: "failure".to_owned(),
    }))
}

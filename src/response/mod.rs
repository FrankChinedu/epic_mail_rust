use serde::Serialize;

pub mod catchers;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

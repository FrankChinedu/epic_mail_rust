use serde::Serialize;

pub mod catchers;

#[derive(Serialize)]
pub enum ResponseMessage {
    Success,
    Failure,
}

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: ResponseMessage,
    pub message: String,
}

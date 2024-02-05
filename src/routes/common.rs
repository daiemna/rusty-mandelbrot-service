use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use derive_more::Display;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct ErrResponse {
    error: String,
}

#[derive(Debug, Display, derive_more::Error)]
pub enum ServiceError {
    #[display(fmt = "File Not found!")]
    FileNotFound,
    #[display(fmt = "Unable to create file!")]
    FileNotCreated,
    #[display(fmt = "Invalid file Name!")]
    InvalidFileName,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            ServiceError::FileNotFound => StatusCode::NOT_FOUND,
            ServiceError::InvalidFileName => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

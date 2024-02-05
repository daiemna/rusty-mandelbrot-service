use crate::config::Config;
use actix_files as fs;
use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::{get, web, HttpRequest};
use std::env::current_dir;
use std::path::PathBuf;

use super::common::ServiceError;

#[get("/files/{filename:.*}")]
pub async fn index(
    req: HttpRequest,
    data: web::Data<Config>,
) -> Result<fs::NamedFile, ServiceError> {
    if let Ok(mut path) = current_dir() {
        // path.pop();
        path.push::<&str>(data.render_dir.as_ref());
        let filename: PathBuf = match req.match_info().query("filename").parse::<PathBuf>() {
            Ok(name) => {
                if name.starts_with("..") {
                    return Err(ServiceError::InvalidFileName);
                }
                // dbg!(name.as_path());
                name
            }
            Err(_) => return Err(ServiceError::InvalidFileName),
        };
        path.push(filename);
        // dbg!(path.as_path());
        if let Ok(file) = fs::NamedFile::open(path) {
            return Ok(file.set_content_disposition(ContentDisposition {
                disposition: DispositionType::Attachment,
                parameters: vec![],
            }));
        }
    }
    return Err(ServiceError::FileNotFound);
}

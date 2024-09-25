use crate::error::Error;
use crate::response::ResponseErr;
use actix_web::HttpResponse;
use mime_guess::from_path;
use rust_embed::Embed;
/// Embed the static files into the binary
#[derive(Embed)]
#[folder = "resource/"]
struct Asset;

pub fn handle_web_file(path: &str) -> HttpResponse {
    match Asset::get(format!("ui/{}", path).as_ref()) {
        Some(content) => HttpResponse::Ok()
            .content_type(from_path(path).first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().json(ResponseErr {
            success: false,
            err_code: "404".to_string(),
            err_message: Some("Not Found".to_string()),
        }),
    }
}

pub fn load_config() -> Result<String, Error> {
    match Asset::get("application.yml") {
        Some(content) => {
            let content = String::from_utf8(content.data.into_owned()).unwrap();
            Ok(content)
        }
        None => Err(Error::UnprocessableEntity("load config file fail".into())),
    }
}
pub fn load_secret(path: &str) -> Result<String, Error> {
    match Asset::get(format!("pem/{}", path).as_ref()) {
        Some(content) => {
            let content = String::from_utf8(content.data.into_owned()).unwrap();
            Ok(content)
        }
        None => Err(Error::UnprocessableEntity("load secret file fail".into())),
    }
}

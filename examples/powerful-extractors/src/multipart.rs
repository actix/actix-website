use actix_multipart::form::{MultipartForm, tempfile::TempFile, text::Text};
use actix_web::Responder;

// <multipart>
#[derive(MultipartForm)]
pub struct Upload {
    title: Text<String>,
    #[multipart(limit = "10 MiB")]
    file: TempFile,
}

#[rustfmt::skip]
pub async fn upload(
    MultipartForm(form): MultipartForm<Upload>,
) -> impl Responder {
    format!("Received {} ({} bytes)", form.title.0, form.file.size)
}
// </multipart>

use actix_web::{Responder, http::header::Accept, web};

// <headers>
// Accept: text/csv;q=0.5, application/json
#[rustfmt::skip]
pub async fn preferred_format(
    web::Header(accept): web::Header<Accept>,
) -> impl Responder {
    if accept.preference() == mime::TEXT_CSV {
        "kind,count\nclick,42\n"
    } else {
        r#"[["click",42]]"#
    }
}
// </headers>

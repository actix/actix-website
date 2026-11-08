use actix_web::{Result, error::ErrorUnauthorized};
use actix_web_httpauth::extractors::bearer::BearerAuth;

// <auth>
pub async fn profile(auth: BearerAuth) -> Result<String> {
    let username = authenticate(auth.token()).await?;

    Ok(format!("Welcome, {username}!"))
}
// </auth>

async fn authenticate(token: &str) -> Result<&'static str> {
    // Demo lookup. Replace with your authentication service.
    if token == "demo-token" {
        Ok("Alice")
    } else {
        Err(ErrorUnauthorized("Invalid token"))
    }
}

#[cfg(test)]
mod tests {
    use actix_web::{App, http::StatusCode, test, web};

    #[actix_web::test]
    async fn bearer_auth_accepts_valid_and_rejects_missing_or_invalid_tokens() {
        let app =
            test::init_service(App::new().route("/profile", web::get().to(super::profile)))
                .await;

        for (token, status) in [
            (Some("Bearer demo-token"), StatusCode::OK),
            (Some("Bearer invalid"), StatusCode::UNAUTHORIZED),
            (None, StatusCode::UNAUTHORIZED),
        ] {
            let mut request = test::TestRequest::get().uri("/profile");

            if let Some(token) = token {
                request = request.insert_header(("authorization", token));
            }

            let response = test::call_service(&app, request.to_request()).await;

            assert_eq!(response.status(), status);

            if status == StatusCode::OK {
                assert_eq!(test::read_body(response).await, "Welcome, Alice!");
            }
        }
    }
}

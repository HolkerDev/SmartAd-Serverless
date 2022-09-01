use http::Response;
use lambda_http::{Body, Error, IntoResponse, Request};

pub async fn handle_sign_up(event: Request) -> Result<impl IntoResponse, Error> {
    Ok(Response::builder()
        .body(Body::from("Noice".to_string()))
        .unwrap())
}

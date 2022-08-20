use lambda_http::{IntoResponse, Request, RequestExt, Response};
use lambda_http::http::StatusCode;
use serde_json::json;
use log::{info, warn};
use crate::models::{ConfirmationCode, UserSignUp};

type E = Box<dyn std::error::Error + Sync + Send + 'static>;

pub async fn handle_sign_up(
    event: Request,
) -> Result<impl IntoResponse, E> {
    let user_sign_up: UserSignUp = match event.payload() {
        Ok(Some(user_sign_up)) => user_sign_up,
        Err(err) => {
            warn!("Failed to parse sign up user from request body: {}", err);
            return Ok(response(
                StatusCode::BAD_REQUEST,
                json!({"message": "Failed to parse request body"}).to_string(),
            ));
        }
        Ok(None) => {
            warn!("Missing user sign up in request body");
            return Ok(response(
                StatusCode::BAD_REQUEST,
                json!({"message": "Wrong body structure"}).to_string(),
            ));
        }
    };

    //TODO: Save user

    //TODO: Generate confirmation code

    Ok(response(StatusCode::OK, json!(ConfirmationCode{ code: String::from("123456")}).to_string()))
}

/// HTTP Response with a JSON payload
fn response(status_code: StatusCode, body: String) -> Response<String> {
    Response::builder()
        .status(status_code)
        .header("Content-Type", "application/json")
        .body(body)
        .unwrap()
}
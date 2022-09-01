use crate::models::ConfirmRegistration;
use http::{Response, StatusCode};
use lambda_http::{Body, Error, IntoResponse, Request, RequestExt};
use log::warn;
use serde_json::json;

use super::utils::response;

pub async fn handle_sign_up(event: Request) -> Result<impl IntoResponse, Error> {
    let _: ConfirmRegistration = match event.payload() {
        Ok(Some(confirmation_request)) => confirmation_request,
        Err(err) => {
            warn!("Error during parsing body: {}", err);
            return Ok(response(
                StatusCode::BAD_REQUEST,
                json!({"message" : "Failed to parse request body"}).to_string(),
            ));
        }
        Ok(None) => {
            return Ok(response(
                StatusCode::BAD_REQUEST,
                json!({"message": "Empty body"}).to_string(),
            ))
        }
    };
    Ok(Response::builder()
        .body(Body::from("Noice".to_string()))
        .unwrap())
}

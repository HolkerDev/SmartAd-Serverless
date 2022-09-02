use crate::models::ConfirmRegistration;
use aws_sdk_dynamodb::model::AttributeValue;
use http::{Response, StatusCode};
use lambda_http::{Body, Error, IntoResponse, Request, RequestExt};
use log::warn;
use serde_json::json;

use super::utils::{init_db, response};

pub async fn handle_sign_up(event: Request) -> Result<impl IntoResponse, Error> {
    let confirmation_request: ConfirmRegistration = match event.payload() {
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

    let client = init_db().await;

    let _ = client
        .get_item()
        .table_name("smartad")
        .key(
            "pk",
            AttributeValue::S(format!(
                "confirmation_code#{}#{}",
                confirmation_request.email, confirmation_request.code
            )),
        )
        .key("sk", AttributeValue::S("none".into()))
        .send()
        .await;

    Ok(Response::builder()
        .body(Body::from("Noice".to_string()))
        .unwrap())
}

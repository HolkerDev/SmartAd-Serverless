use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::model::AttributeValue;
use bcrypt::{DEFAULT_COST, hash};
use lambda_http::{Body, Error, IntoResponse, Request, RequestExt, Response};
use lambda_http::http::StatusCode;
use serde_json::json;

use crate::models::{ConfirmationCode, UserSignUp};

pub async fn handle_sign_up(event: Request) -> Result<impl IntoResponse, Error> {
    let user_sign_up: UserSignUp = match event.payload() {
        Ok(Some(user_sign_up)) => user_sign_up,
        Err(_err) => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(
                    json!({"message": "Failed to parse request body"}).to_string(),
                ))
                .unwrap());
        }
        Ok(None) => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(
                    json!({"message": "Wrong body structure"}).to_string(),
                ))
                .unwrap());
        }
    };

    let client = init_db().await;


    //TODO: Check if user email unique

    let _result = client.put_item()
        .table_name("smartad")
        .item("pk", AttributeValue::S(format!("user#{}", user_sign_up.email)))
        .item("sk", AttributeValue::S("none".into()))
        .item("email", AttributeValue::S(user_sign_up.email))
        .item("password", AttributeValue::S(hash(user_sign_up.password, DEFAULT_COST).unwrap()))
        .send()
        .await;

    //TODO: Generate confirmation code and save it

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(
            json!(ConfirmationCode {
                code: "123456".into()
            }).to_string(),
        ))
        .unwrap())
}

async fn init_db() -> Client {
    let shared_config = aws_config::load_from_env().await;
    Client::new(&shared_config)
}

#[cfg(test)]
mod tests {
    use lambda_http::http::StatusCode;
    use lambda_http::IntoResponse;

    use crate::handlers::sign_up_handler::handle_sign_up;
    use crate::models::UserSignUp;

    use super::super::utils::*;

    #[tokio::test]
    async fn should_return_ok_when_form_is_ok() {
        let req = request_new(
            serde_json::to_string(&UserSignUp {
                email: "something".into(),
                password: "pass".into(),
            })
                .unwrap(),
        );
        let res = handle_sign_up(req).await.unwrap().into_response().await;
        assert_eq!(res.status(), StatusCode::OK);
    }
}

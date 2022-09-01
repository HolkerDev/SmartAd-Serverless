use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use bcrypt::{hash, DEFAULT_COST};
use lambda_http::http::StatusCode;
use lambda_http::{Body, Error, IntoResponse, Request, RequestExt, Response};
use rand::rngs::ThreadRng;
use rand::Rng;
use serde_json::json;

use crate::models::{ConfirmationCode, UserSignUp};

pub async fn handle_sign_up(event: Request) -> Result<impl IntoResponse, Error> {
    let user_sign_up: UserSignUp = match event.payload() {
        Ok(Some(user_sign_up)) => user_sign_up,
        Err(_err) => {
            return Ok(response(
                StatusCode::BAD_REQUEST,
                json!({"message": "Failed to parse request body"}).to_string(),
            ));
        }
        Ok(None) => {
            return Ok(response(
                StatusCode::BAD_REQUEST,
                json!({"message": "Wrong body structure"}).to_string(),
            ));
        }
    };

    let client = init_db().await;

    let existing_user = client
        .get_item()
        .table_name("smartad")
        .key(
            "pk",
            AttributeValue::S(format!("user#{}", user_sign_up.email.clone())),
        )
        .key("sk", AttributeValue::S("none".into()))
        .send()
        .await?;

    if existing_user.item().is_some() {
        return Ok(response(
            StatusCode::CONFLICT,
            json!({"message": "This email is already taken"}).to_string(),
        ));
    }

    let _ = client
        .put_item()
        .table_name("smartad")
        .item(
            "pk",
            AttributeValue::S(format!("user#{}", user_sign_up.email)),
        )
        .item("sk", AttributeValue::S("none".into()))
        .item("email", AttributeValue::S(user_sign_up.email.clone()))
        .item(
            "password",
            AttributeValue::S(hash(user_sign_up.password, DEFAULT_COST).unwrap()),
        )
        .send()
        .await;

    let code = generate_confirmation_code();

    let _ = client
        .put_item()
        .table_name("smartad")
        .item(
            "pk",
            AttributeValue::S(format!("confirmation_code#{}#{}", user_sign_up.email, code)),
        )
        .item("sk", AttributeValue::S("none".into()))
        .send()
        .await;

    Ok(response(
        StatusCode::CREATED,
        json!(ConfirmationCode { code }).to_string(),
    ))
}

async fn init_db() -> Client {
    let shared_config = aws_config::load_from_env().await;
    Client::new(&shared_config)
}

fn response(status_code: StatusCode, json_string: String) -> Response<Body> {
    Response::builder()
        .status(status_code)
        .body(Body::from(json_string))
        .unwrap()
}

fn generate_confirmation_code() -> String {
    let random = rand::thread_rng();
    let mut code = "".to_string();
    for _ in 0..6 {
        code.push_str(&generate_random_code_number(random.clone()))
    }
    code
}

fn generate_random_code_number(mut random: ThreadRng) -> String {
    random.gen_range(0..10).to_string()
}

#[cfg(test)]
mod tests {
    use lambda_http::http::StatusCode;
    use lambda_http::IntoResponse;

    use crate::handlers::sign_up_handler::generate_confirmation_code;
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
        assert_eq!(res.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn should_generate_six_numbers_code() {
        let code = generate_confirmation_code();
        assert_eq!(code.len(), 6);
    }
}

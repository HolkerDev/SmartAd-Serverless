use lambda_http::{Body, Error, IntoResponse, Request, RequestExt, Response};
use lambda_http::http::StatusCode;
use serde_json::json;
use crate::models::{ConfirmationCode, UserSignUp};

// type E = Box<dyn std::error::Error + Sync + Send + 'static>;

pub async fn handle_sign_up(event: Request) -> Result<impl IntoResponse, Error> {
    let _user_sign_up: UserSignUp = match event.payload() {
        Ok(Some(user_sign_up)) => user_sign_up,
        Err(_err) => {
            return Ok(Response::builder().status(StatusCode::BAD_REQUEST).body(Body::from(json!({"message": "Failed to parse request body"}).to_string())).unwrap());
        }
        Ok(None) => {
            return Ok(Response::builder().status(StatusCode::BAD_REQUEST).body(Body::from(json!({"message": "Wrong body structure"}).to_string())).unwrap());
        }
    };

    //TODO: Save user

    //TODO: Generate confirmation code

    Ok(Response::builder().status(StatusCode::OK).body(Body::from(json!(ConfirmationCode {code: String::from("123456")}).to_string())).unwrap())
}

#[cfg(test)]
mod tests {
    use lambda_http::{IntoResponse};
    use lambda_http::http::StatusCode;

    use super::super::utils::*;
    use crate::handlers::sign_up_handler::{handle_sign_up};
    use crate::models::UserSignUp;

    #[tokio::test]
    async fn test_something() {
        let req = request_new(
            serde_json::to_string(&UserSignUp {
                email: String::from("something"),
                password: String::from("pass"),
            })
            .unwrap(),
        );
        let res = handle_sign_up(req).await.unwrap().into_response().await;
        assert_eq!(res.status(), StatusCode::OK);
    }
}

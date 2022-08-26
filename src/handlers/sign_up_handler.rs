use crate::models::{ConfirmationCode, UserSignUp};
use lambda_http::http::StatusCode;
use lambda_http::{Body, Error, IntoResponse, Request, RequestExt, Response};
use serde_json::json;

// type E = Box<dyn std::error::Error + Sync + Send + 'static>;

pub async fn handle_sign_up(event: Request) -> Result<impl IntoResponse, Error> {
    let _user_sign_up: UserSignUp = match event.payload() {
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

    //TODO: Save user

    //TODO: Generate confirmation code

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(
            json!(ConfirmationCode {
                code: "123456".into()
            })
            .to_string(),
        ))
        .unwrap())
}

#[cfg(test)]
mod tests {
    use lambda_http::http::StatusCode;
    use lambda_http::IntoResponse;

    use super::super::utils::*;
    use crate::handlers::sign_up_handler::handle_sign_up;
    use crate::models::UserSignUp;

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

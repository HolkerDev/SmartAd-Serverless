use aws_lambda_events::event::apigw::{
    ApiGatewayProxyRequest as AwsRequest, ApiGatewayProxyResponse as AwsResponse,
};
use lambda_http::http::StatusCode;
use log::warn;
use serde_json::json;

use crate::models::{ConfirmationCode, UserSignUp};

use super::utils::response;

type E = Box<dyn std::error::Error + Sync + Send + 'static>;

pub async fn handle_sign_up(
    event: AwsRequest,
) -> Result<AwsResponse, E> {
    let user_sign_up: UserSignUp = match serde_json::from_str(event.body.unwrap().as_str()) {
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


#[cfg(test)]
mod tests {
    use crate::handlers::sign_up_handler::handle_sign_up;
    use crate::models::UserSignUp;
    use super::super::utils::*;

    #[tokio::test]
    async fn test_something() {
        let mut req = request();
        req.body = Some(serde_json::to_string(&UserSignUp { email: String::from("something"), password: String::from("pass") }).unwrap());
        let res = handle_sign_up(req.clone()).await;
        assert_eq!(res.unwrap().status_code, 200);
    }
}
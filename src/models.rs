use aws_sdk_dynamodb::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserSignUp {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct ConfirmRegistration {
    pub email: String,
    pub code: String,
}

#[derive(Deserialize, Serialize)]
pub struct ConfirmationCode {
    pub code: String,
}

impl ConfirmationCode {
    pub fn new(code: String) -> ConfirmationCode {
        ConfirmationCode { code }
    }
}

pub async fn get_db_client() -> Client {
    let config = aws_config::load_from_env().await;
    aws_sdk_dynamodb::Client::new(&config)
}

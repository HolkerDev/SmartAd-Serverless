use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserSignUp {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct ConfirmationCode {
    pub code: String,
}

trait ConfirmationCodeConstructor {
    fn new(code: String) -> ConfirmationCode;
}

impl ConfirmationCodeConstructor for ConfirmationCode {
    fn new(code: String) -> ConfirmationCode {
        ConfirmationCode {
            code
        }
    }
}
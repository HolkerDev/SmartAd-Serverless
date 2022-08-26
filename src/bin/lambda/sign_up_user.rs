use lambda_http::{service_fn, Error};
use smartad::handlers::sign_up_handler::handle_sign_up;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_http::run(service_fn(handle_sign_up)).await?;
    Ok(())
}
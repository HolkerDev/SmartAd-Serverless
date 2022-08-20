use lambda_http::{Request, service_fn};

use smartad::handlers::sign_up_handler::handle_sign_up;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    lambda_http::run(service_fn(|event: Request| handle_sign_up(event))).await?;
    Ok(())
}
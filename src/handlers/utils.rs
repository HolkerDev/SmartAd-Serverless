use aws_lambda_events::encodings::Body;
use aws_lambda_events::event::apigw::ApiGatewayProxyResponse as AwsResponse;
use lambda_http::http::{HeaderMap, StatusCode};

pub trait StatusCodeExt {
    fn as_i64(&self) -> i64;
}

impl StatusCodeExt for StatusCode {
    fn as_i64(&self) -> i64 {
        self.as_str().parse::<i64>().unwrap()
    }
}

pub fn response(status: StatusCode, json_body: String) -> AwsResponse {
    AwsResponse {
        status_code: status.as_i64(),
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Text(json_body)),
        is_base64_encoded: Some(false),
    }
}
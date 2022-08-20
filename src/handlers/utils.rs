use aws_lambda_events::encodings::Body;
use aws_lambda_events::event::apigw::{ApiGatewayProxyResponse as AwsResponse, ApiGatewayProxyRequest as AwsRequest};
use lambda_http::http::{HeaderMap, StatusCode};

pub trait StatusCodeExt {
    fn as_i64(&self) -> i64;
}

impl StatusCodeExt for StatusCode {
    fn as_i64(&self) -> i64 {
        self.as_str().parse::<i64>().unwrap()
    }
}

pub fn request() -> AwsRequest {
    AwsRequest {
        resource: None,
        path: None,
        http_method: Default::default(),
        headers: Default::default(),
        multi_value_headers: Default::default(),
        query_string_parameters: Default::default(),
        multi_value_query_string_parameters: Default::default(),
        path_parameters: Default::default(),
        stage_variables: Default::default(),
        request_context: Default::default(),
        body: None,
        is_base64_encoded: None,
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
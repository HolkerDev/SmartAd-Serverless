use http::request::Request;
use http::Response;
use lambda_http::http::StatusCode;
use lambda_http::Body;

pub trait StatusCodeExt {
    fn as_i64(&self) -> i64;
}

impl StatusCodeExt for StatusCode {
    fn as_i64(&self) -> i64 {
        self.as_str().parse::<i64>().unwrap()
    }
}

pub fn request_new(json_body: String) -> Request<Body> {
    println!("Json payload: {}", json_body);
    Request::builder()
        .header(
            String::from("content-type"),
            String::from("application/json"),
        )
        .body(Body::from(json_body))
        .unwrap()
}

pub fn response(status_code: StatusCode, json_string: String) -> Response<Body> {
    Response::builder()
        .status(status_code)
        .body(Body::from(json_string))
        .unwrap()
}

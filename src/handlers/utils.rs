use lambda_http::http::StatusCode;

pub trait StatusCodeExt {
    fn as_i64(&self) -> i64;
}

impl StatusCodeExt for StatusCode {
    fn as_i64(&self) -> i64 {
        self.as_str().parse::<i64>().unwrap()
    }
}
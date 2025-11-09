#[derive(Debug)]
pub enum CurlErrors {
    BadMethod,
    ConnectionFailed,
    HeaderError,
    JsonError,
}

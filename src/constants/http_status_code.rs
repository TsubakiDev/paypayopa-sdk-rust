//! HTTP status code constants used by PayPay OPA response handling.

/// HTTP status code thresholds used by the SDK.
pub struct HttpStatusCode;

impl HttpStatusCode {
    /// The HTTP 200 OK success status.
    pub const OK: u16 = 200;
    /// The first HTTP 3xx redirection status code.
    pub const REDIRECT: u16 = 300;
}

//! Error code constants returned by PayPay OPA.

/// PayPay OPA response error code values.
pub struct ErrorCode;

impl ErrorCode {
    /// Error code for invalid request payloads or parameters.
    pub const BAD_REQUEST_ERROR: &'static str = "BAD_REQUEST_ERROR";
    /// Error code for gateway-level failures.
    pub const GATEWAY_ERROR: &'static str = "GATEWAY_ERROR";
    /// Error code for server-side failures.
    pub const SERVER_ERROR: &'static str = "SERVER_ERROR";
    /// Error code for failed authentication or authorization.
    pub const UNAUTHORIZED: &'static str = "UNAUTHORIZED";
}

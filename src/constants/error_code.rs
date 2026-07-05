pub struct ErrorCode;

impl ErrorCode {
    pub const BAD_REQUEST_ERROR: &'static str = "BAD_REQUEST_ERROR";
    pub const GATEWAY_ERROR: &'static str = "GATEWAY_ERROR";
    pub const SERVER_ERROR: &'static str = "SERVER_ERROR";
    pub const UNAUTHORIZED: &'static str = "UNAUTHORIZED";
}

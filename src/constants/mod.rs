//! Constants used by the PayPay OPA API client.

/// API identifier constants used for PayPay OPA request signing.
pub mod api_list;
/// Error code constants returned by PayPay OPA responses.
pub mod error_code;
/// HTTP status code thresholds used by the SDK.
pub mod http_status_code;
/// Base URLs and resource paths for PayPay OPA endpoints.
pub mod url;

pub use api_list::ApiNames;
pub use error_code::ErrorCode;
pub use http_status_code::HttpStatusCode;
pub use url::Url;

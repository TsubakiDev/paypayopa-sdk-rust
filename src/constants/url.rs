//! Base URLs and path constants for PayPay OPA endpoints.

/// PayPay OPA base URLs and resource paths.
pub struct Url;

impl Url {
    /// Sandbox API gateway base URL.
    pub const SANDBOX_BASE_URL: &'static str = "https://apigw.sandbox.paypay.ne.jp";
    /// Production API gateway base URL.
    pub const PRODUCTION_BASE_URL: &'static str = "https://apigw.paypay.ne.jp";
    /// Performance test API gateway base URL.
    pub const PERF_BASE_URL: &'static str = "https://perf-apigw.paypay.ne.jp";
    /// Developer resolve URL used by PayPay account linking flows.
    pub const RESOLVE: &'static str = "https://developer.paypay.ne.jp/develop/resolve";
    /// Dynamic QR code resource path.
    pub const CODE: &'static str = "/v2/codes";
    /// Payment resource path.
    pub const PAYMENT: &'static str = "/v2/payments";
    /// Account-link QR session resource path.
    pub const ACCOUNT_LINK: &'static str = "/v1/qr/sessions";
    /// Pending payment request order resource path.
    pub const PENDING_PAYMENT: &'static str = "/v1/requestOrder";
    /// User authorization resource path.
    pub const USER_AUTH: &'static str = "/v2/user/authorizations";
    /// Cashback resource path.
    pub const GIVE_CASHBACK: &'static str = "/v2/cashback";
    /// Cashback reversal resource path.
    pub const REVERSAL_CASHBACK: &'static str = "/v2/cashback_reversal";
    /// Refund resource path.
    pub const REFUNDS: &'static str = "/v2/refunds";
}

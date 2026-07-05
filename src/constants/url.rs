pub struct Url;

impl Url {
    pub const SANDBOX_BASE_URL: &'static str = "https://apigw.sandbox.paypay.ne.jp";
    pub const PRODUCTION_BASE_URL: &'static str = "https://apigw.paypay.ne.jp";
    pub const PERF_BASE_URL: &'static str = "https://perf-apigw.paypay.ne.jp";
    pub const RESOLVE: &'static str = "https://developer.paypay.ne.jp/develop/resolve";
    pub const CODE: &'static str = "/v2/codes";
    pub const PAYMENT: &'static str = "/v2/payments";
    pub const ACCOUNT_LINK: &'static str = "/v1/qr/sessions";
    pub const PENDING_PAYMENT: &'static str = "/v1/requestOrder";
    pub const USER_AUTH: &'static str = "/v2/user/authorizations";
    pub const GIVE_CASHBACK: &'static str = "/v2/cashback";
    pub const REVERSAL_CASHBACK: &'static str = "/v2/cashback_reversal";
    pub const REFUNDS: &'static str = "/v2/refunds";
}

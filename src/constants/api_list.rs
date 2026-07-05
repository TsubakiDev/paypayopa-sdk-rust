//! PayPay OPA API identifiers used in request metadata.

/// API names sent with requests for PayPay OPA tracking and validation.
pub struct ApiNames;

impl ApiNames {
    /// API identifier for canceling a payment.
    pub const CANCEL_PAYMENT: &'static str = "v2_cancelPayment";
    /// API identifier for capturing an authorized payment.
    pub const CAPTURE_PAYMENT: &'static str = "v2_captureAuthorizedOrder";
    /// API identifier for creating a payment.
    pub const CREATE_PAYMENT: &'static str = "v2_createPayment";
    /// API identifier for creating a dynamic QR code.
    pub const CREATE_QRCODE: &'static str = "v2_createDynamicQRCode";
    /// API identifier for deleting a dynamic QR code.
    pub const DELETE_QRCODE: &'static str = "v2_deleteDynamicQRCode";
    /// API identifier for retrieving payment details.
    pub const GET_PAYMENT: &'static str = "v2_getPaymentDetail";
    /// API identifier for retrieving QR payment details.
    pub const GET_QR_PAYMENT: &'static str = "v2_getQRPaymentDetails";
    /// API identifier for retrieving refund details.
    pub const GET_REFUND: &'static str = "v2_getRefundDetails";
    /// API identifier for creating a payment refund.
    pub const REFUND_PAYMENT: &'static str = "v2_createRefundPayment";
    /// API identifier for reverting an authorized order.
    pub const REVERT_AUTHORIZE: &'static str = "v2_revertAuthorizedOrder";
    /// API identifier for creating a preauthorized payment.
    pub const PREAUTHORIZE_PAYMENT: &'static str = "v2_createOrderAndAuthorize";
    /// API identifier for creating a subscription payment.
    pub const CREATE_CONTINUOUS_PAYMENT: &'static str = "v1_createSubscriptionPayment";
    /// API identifier for creating a request order.
    pub const CREATE_REQUEST_ORDER: &'static str = "v1_createRequestOrder";
    /// API identifier for retrieving a request order.
    pub const GET_REQUEST_ORDER: &'static str = "v1_getRequestOrder";
    /// API identifier for canceling a request order.
    pub const CANCEL_REQUEST_ORDER: &'static str = "v1_cancelRequestOrder";
    /// API identifier for refunding a request order.
    pub const REFUND_REQUEST_ORDER: &'static str = "v2_createRefundPayment";
    /// API identifier for retrieving a secure user profile.
    pub const GET_SECURE_USER_PROFILE: &'static str = "v2_getSecureUserProfile";
    /// API identifier for checking wallet balance.
    pub const CHECK_BALANCE: &'static str = "v2_checkWalletBalance";
    /// API identifier for retrieving user authorization status.
    pub const GET_USER_AUTH_STATUS: &'static str = "v2_userAuthStatus";
    /// API identifier for unlinking a user authorization.
    pub const UNLINK_USER: &'static str = "v2_unlinkUser";
    /// API identifier for creating an account-link QR session.
    pub const CREATE_QR_SESSION: &'static str = "v1_qrSession";
    /// API identifier for creating a cashback request.
    pub const CREATE_CASHBACK_REQUEST: &'static str = "v2_createCashBackRequest";
    /// API identifier for retrieving cashback details.
    pub const GET_CASHBACK_DETAILS: &'static str = "v2_getCashbackDetails";
    /// API identifier for creating a cashback reversal request.
    pub const CREATE_REVERSE_CASHBACK_REQUEST: &'static str = "v2_createReverseCashBackRequest";
    /// API identifier for retrieving reversed cashback details.
    pub const GET_REVERESED_CASHBACK_DETAILS: &'static str = "v2_getReversedCashBackDetails";
}

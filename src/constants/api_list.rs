pub struct ApiNames;

impl ApiNames {
    pub const CANCEL_PAYMENT: &'static str = "v2_cancelPayment";
    pub const CAPTURE_PAYMENT: &'static str = "v2_captureAuthorizedOrder";
    pub const CREATE_PAYMENT: &'static str = "v2_createPayment";
    pub const CREATE_QRCODE: &'static str = "v2_createDynamicQRCode";
    pub const DELETE_QRCODE: &'static str = "v2_deleteDynamicQRCode";
    pub const GET_PAYMENT: &'static str = "v2_getPaymentDetail";
    pub const GET_QR_PAYMENT: &'static str = "v2_getQRPaymentDetails";
    pub const GET_REFUND: &'static str = "v2_getRefundDetails";
    pub const REFUND_PAYMENT: &'static str = "v2_createRefundPayment";
    pub const REVERT_AUTHORIZE: &'static str = "v2_revertAuthorizedOrder";
    pub const PREAUTHORIZE_PAYMENT: &'static str = "v2_createOrderAndAuthorize";
    pub const CREATE_CONTINUOUS_PAYMENT: &'static str = "v1_createSubscriptionPayment";
    pub const CREATE_REQUEST_ORDER: &'static str = "v1_createRequestOrder";
    pub const GET_REQUEST_ORDER: &'static str = "v1_getRequestOrder";
    pub const CANCEL_REQUEST_ORDER: &'static str = "v1_cancelRequestOrder";
    pub const REFUND_REQUEST_ORDER: &'static str = "v2_createRefundPayment";
    pub const GET_SECURE_USER_PROFILE: &'static str = "v2_getSecureUserProfile";
    pub const CHECK_BALANCE: &'static str = "v2_checkWalletBalance";
    pub const GET_USER_AUTH_STATUS: &'static str = "v2_userAuthStatus";
    pub const UNLINK_USER: &'static str = "v2_unlinkUser";
    pub const CREATE_QR_SESSION: &'static str = "v1_qrSession";
    pub const CREATE_CASHBACK_REQUEST: &'static str = "v2_createCashBackRequest";
    pub const GET_CASHBACK_DETAILS: &'static str = "v2_getCashbackDetails";
    pub const CREATE_REVERSE_CASHBACK_REQUEST: &'static str = "v2_createReverseCashBackRequest";
    pub const GET_REVERESED_CASHBACK_DETAILS: &'static str = "v2_getReversedCashBackDetails";
}

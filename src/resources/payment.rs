use std::sync::Arc;

use crate::client::ClientInner;
use crate::constants::{ApiNames, Url};
use crate::error::Result;
use crate::resources::{
    add_requested_at, optional_data, require_amount, require_field, required_id,
    IntoOptionalString, IntoOptionalValue,
};
use crate::Value;

#[derive(Clone)]
pub struct Payment {
    inner: Arc<ClientInner>,
}

impl Payment {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    pub fn create<D: IntoOptionalValue>(&self, data: D) -> Result<Value> {
        let mut data = optional_data(data);
        add_requested_at(&mut data)?;
        require_field(&data, "merchantPaymentId")?;
        require_amount(&data)?;
        self.inner
            .post(Url::PAYMENT, Some(&data), ApiNames::CREATE_PAYMENT)
    }

    pub fn get_payment_details<S: IntoOptionalString>(&self, id: S) -> Result<Value> {
        let id = required_id(id, "merchantPaymentId")?;
        let url = format!("{}/{}", Url::PAYMENT, id);
        self.inner.get(&url, None, ApiNames::GET_PAYMENT)
    }

    pub fn cancel_payment<S: IntoOptionalString>(&self, id: S) -> Result<Value> {
        let id = required_id(id, "merchantPaymentId")?;
        let url = format!("{}/{}", Url::PAYMENT, id);
        self.inner.delete(&url, None, ApiNames::CANCEL_PAYMENT)
    }

    pub fn refund_payment<D: IntoOptionalValue>(&self, data: D) -> Result<Value> {
        let mut data = optional_data(data);
        add_requested_at(&mut data)?;
        require_field(&data, "merchantRefundId")?;
        require_field(&data, "paymentId")?;
        require_amount(&data)?;
        self.inner
            .post("/v2/refunds/", Some(&data), ApiNames::REFUND_PAYMENT)
    }

    pub fn refund_details<S: IntoOptionalString>(&self, id: S) -> Result<Value> {
        let id = required_id(id, "merchantRefundId")?;
        let url = format!("{}/{}", Url::REFUNDS, id);
        self.inner.get(&url, None, ApiNames::GET_REFUND)
    }

    pub fn capture_payment<D: IntoOptionalValue>(&self, data: D) -> Result<Value> {
        let mut data = optional_data(data);
        add_requested_at(&mut data)?;
        require_field(&data, "merchantPaymentId")?;
        require_field(&data, "merchantCaptureId")?;
        require_field(&data, "orderDescription")?;
        require_amount(&data)?;
        self.inner.post(
            "/v2/payments/capture",
            Some(&data),
            ApiNames::CAPTURE_PAYMENT,
        )
    }

    pub fn create_continuous_payment<D: IntoOptionalValue>(&self, data: D) -> Result<Value> {
        let mut data = optional_data(data);
        add_requested_at(&mut data)?;
        require_field(&data, "merchantPaymentId")?;
        require_field(&data, "userAuthorizationId")?;
        require_amount(&data)?;
        self.inner.post(
            "/v1/subscription/payments",
            Some(&data),
            ApiNames::CREATE_CONTINUOUS_PAYMENT,
        )
    }

    pub fn revert_payment<D: IntoOptionalValue>(&self, data: D) -> Result<Value> {
        let mut data = optional_data(data);
        add_requested_at(&mut data)?;
        require_field(&data, "merchantRevertId")?;
        require_field(&data, "paymentId")?;
        self.inner.post(
            "/v2/payments/preauthorize/revert",
            Some(&data),
            ApiNames::REVERT_AUTHORIZE,
        )
    }
}

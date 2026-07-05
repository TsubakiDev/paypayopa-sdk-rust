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
pub struct Pending {
    inner: Arc<ClientInner>,
}

impl Pending {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    pub fn create_pending_payment<D: IntoOptionalValue>(&self, data: D) -> Result<Value> {
        let mut data = optional_data(data);
        add_requested_at(&mut data)?;
        require_field(&data, "merchantPaymentId")?;
        require_field(&data, "userAuthorizationId")?;
        require_amount(&data)?;
        self.inner.post(
            Url::PENDING_PAYMENT,
            Some(&data),
            ApiNames::CREATE_REQUEST_ORDER,
        )
    }

    pub fn get_payment_details<S: IntoOptionalString>(&self, id: S) -> Result<Value> {
        let id = required_id(id, "merchantPaymentId")?;
        let url = format!("{}/{}", Url::PENDING_PAYMENT, id);
        self.inner.get(&url, None, ApiNames::GET_REQUEST_ORDER)
    }

    pub fn cancel_payment<S: IntoOptionalString>(&self, id: S) -> Result<Value> {
        let id = required_id(id, "merchantPaymentId")?;
        let url = format!("{}/{}", Url::PENDING_PAYMENT, id);
        self.inner
            .delete(&url, None, ApiNames::CANCEL_REQUEST_ORDER)
    }

    pub fn refund_payment<D: IntoOptionalValue>(&self, data: D) -> Result<Value> {
        let mut data = optional_data(data);
        require_field(&data, "merchantRefundId")?;
        add_requested_at(&mut data)?;
        require_field(&data, "paymentId")?;
        require_amount(&data)?;
        self.inner
            .post(Url::REFUNDS, Some(&data), ApiNames::REFUND_REQUEST_ORDER)
    }

    pub fn refund_details<S: IntoOptionalString>(&self, id: S) -> Result<Value> {
        let id = required_id(id, "merchantRefundId")?;
        let url = format!("{}/{}", Url::REFUNDS, id);
        self.inner.get(&url, None, ApiNames::GET_REFUND)
    }
}

//! Dynamic QR code resource client.

use std::sync::Arc;

use crate::client::ClientInner;
use crate::constants::{ApiNames, Url};
use crate::error::Result;
use crate::resources::{
    add_requested_at, empty_object, optional_data, require_amount, require_field,
    require_order_items, required_id, IntoOptionalString, IntoOptionalValue,
};
use crate::Value;

/// Client for PayPay dynamic QR code APIs.
#[derive(Clone)]
pub struct Code {
    inner: Arc<ClientInner>,
}

impl Code {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    /// Creates a dynamic QR code for a payment.
    ///
    /// The request body must include `merchantPaymentId` and a valid `amount`.
    /// When `orderItems` are present, each item is validated before sending.
    pub fn create_qr_code<D: IntoOptionalValue>(&self, data: D) -> Result<Value> {
        let mut data = optional_data(data);
        add_requested_at(&mut data)?;
        require_field(&data, "merchantPaymentId")?;
        require_amount(&data)?;
        require_order_items(&data)?;
        self.inner
            .post(Url::CODE, Some(&data), ApiNames::CREATE_QRCODE)
    }

    /// Retrieves payment details for a QR payment by merchant payment ID.
    pub fn get_payment_details<S: IntoOptionalString>(&self, id: S) -> Result<Value> {
        let id = required_id(id, "merchantPaymentId")?;
        let url = format!("{}/payments/{}", Url::CODE, id);
        self.inner.get(&url, None, ApiNames::GET_QR_PAYMENT)
    }

    /// Deletes a dynamic QR code by code ID.
    pub fn delete_qr_code<S: IntoOptionalString>(&self, id: S) -> Result<Value> {
        let id = required_id(id, "codeId")?;
        let url = format!("{}/{}", Url::CODE, id);
        let data = empty_object();
        self.inner
            .delete(&url, Some(&data), ApiNames::DELETE_QRCODE)
    }
}

//! Preauthorization resource client.

use std::sync::Arc;

use crate::client::ClientInner;
use crate::constants::{ApiNames, Url};
use crate::error::Result;
use crate::resources::{
    add_requested_at, optional_data, require_amount, require_field,
    require_integer_field_if_present, IntoOptionalValue,
};
use crate::Value;

/// Client for PayPay preauthorization APIs.
#[derive(Clone)]
pub struct Preauth {
    inner: Arc<ClientInner>,
}

impl Preauth {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    /// Creates a preauthorized payment.
    ///
    /// The request body must include `merchantPaymentId`, `userAuthorizationId`,
    /// and a valid `amount`. When `expiresAt` is present, it must be an integer
    /// epoch timestamp.
    pub fn pre_authorize_create<D: IntoOptionalValue>(&self, data: D) -> Result<Value> {
        let mut data = optional_data(data);
        add_requested_at(&mut data)?;
        require_field(&data, "merchantPaymentId")?;
        require_field(&data, "userAuthorizationId")?;
        require_integer_field_if_present(&data, "expiresAt")?;
        require_amount(&data)?;
        let url = format!("{}/preauthorize", Url::PAYMENT);
        self.inner
            .post(&url, Some(&data), ApiNames::PREAUTHORIZE_PAYMENT)
    }
}

//! Account linking resource client.

use std::sync::Arc;

use crate::client::ClientInner;
use crate::constants::{ApiNames, Url};
use crate::error::Result;
use crate::resources::{optional_data, require_field, IntoOptionalValue};
use crate::Value;

/// Client for PayPay account linking QR session APIs.
#[derive(Clone)]
pub struct Account {
    inner: Arc<ClientInner>,
}

impl Account {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    /// Creates an account-linking QR session.
    ///
    /// The request body must include `scopes`, `nonce`, `redirectUrl`, and
    /// `referenceId`.
    pub fn create_qr_session<D: IntoOptionalValue>(&self, data: D) -> Result<Value> {
        let data = optional_data(data);
        require_field(&data, "scopes")?;
        require_field(&data, "nonce")?;
        require_field(&data, "redirectUrl")?;
        require_field(&data, "referenceId")?;
        self.inner
            .post(Url::ACCOUNT_LINK, Some(&data), ApiNames::CREATE_QR_SESSION)
    }
}

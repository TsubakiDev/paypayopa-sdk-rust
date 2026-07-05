//! User authorization resource client.

use std::sync::Arc;

use serde_json::json;

use crate::client::ClientInner;
use crate::constants::{ApiNames, Url};
use crate::error::Result;
use crate::resources::{empty_object, required_id, required_query_id, IntoOptionalString};
use crate::Value;

/// Client for PayPay user authorization APIs.
#[derive(Clone)]
pub struct User {
    inner: Arc<ClientInner>,
}

impl User {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    /// Retrieves the status of a user authorization by user authorization ID.
    pub fn get_authorization_status<S: IntoOptionalString>(&self, id: S) -> Result<Value> {
        let id = required_query_id(id, "userAuthorizationId")?;
        let params = json!({
            "userAuthorizationId": id,
        });
        self.inner.get(
            Url::USER_AUTH,
            Some(&params),
            ApiNames::GET_USER_AUTH_STATUS,
        )
    }

    /// Unlinks a user authorization by code ID.
    ///
    /// This method preserves the historical misspelling in the public API. Use
    /// [`User::unlink_user_authorization`] in new code.
    pub fn unlink_user_athorization<S: IntoOptionalString>(&self, id: S) -> Result<Value> {
        let id = required_id(id, "codeId")?;
        let url = format!("{}/{}", Url::USER_AUTH, id);
        let data = empty_object();
        self.inner.delete(&url, Some(&data), ApiNames::UNLINK_USER)
    }

    /// Unlinks a user authorization by code ID.
    pub fn unlink_user_authorization<S: IntoOptionalString>(&self, id: S) -> Result<Value> {
        self.unlink_user_athorization(id)
    }
}

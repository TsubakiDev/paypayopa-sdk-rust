use std::sync::Arc;

use crate::client::ClientInner;
use crate::constants::{ApiNames, Url};
use crate::error::Result;
use crate::resources::{
    optional_data, require_amount_shape, require_field, required_id, IntoOptionalString,
    IntoOptionalValue,
};
use crate::Value;

#[derive(Clone)]
pub struct Cashback {
    inner: Arc<ClientInner>,
}

impl Cashback {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    pub fn give_cashback<D: IntoOptionalValue>(&self, data: D) -> Result<Value> {
        let data = optional_data(data);
        require_field(&data, "merchantCashbackId")?;
        require_field(&data, "userAuthorizationId")?;
        require_amount_shape(&data, "amount amount", "amount currency")?;
        require_field(&data, "requestedAt")?;
        require_field(&data, "walletType")?;
        self.inner.post(
            Url::GIVE_CASHBACK,
            Some(&data),
            ApiNames::CREATE_CASHBACK_REQUEST,
        )
    }

    pub fn check_cashback_detail<S: IntoOptionalString>(
        &self,
        merchant_cashback_id: S,
    ) -> Result<Value> {
        let merchant_cashback_id = required_id(merchant_cashback_id, "merchantCashbackId")?;
        let url = format!("{}/{}", Url::GIVE_CASHBACK, merchant_cashback_id);
        self.inner.get(&url, None, ApiNames::GET_CASHBACK_DETAILS)
    }

    pub fn reverse_cashback<D: IntoOptionalValue>(&self, data: D) -> Result<Value> {
        let data = optional_data(data);
        require_field(&data, "merchantCashbackReversalId")?;
        require_field(&data, "merchantCashbackId")?;
        require_amount_shape(&data, "amount amount", "amount currency")?;
        require_field(&data, "requestedAt")?;
        self.inner.post(
            Url::REVERSAL_CASHBACK,
            Some(&data),
            ApiNames::CREATE_REVERSE_CASHBACK_REQUEST,
        )
    }

    pub fn check_cashback_reversal_detail<R, C>(
        &self,
        merchant_cashback_reversal_id: R,
        merchant_cashback_id: C,
    ) -> Result<Value>
    where
        R: IntoOptionalString,
        C: IntoOptionalString,
    {
        let merchant_cashback_reversal_id =
            required_id(merchant_cashback_reversal_id, "merchantCashbackReversalId")?;
        let merchant_cashback_id = required_id(merchant_cashback_id, "merchantCashbackId")?;
        let url = format!(
            "{}/{}/{}",
            Url::REVERSAL_CASHBACK,
            merchant_cashback_reversal_id,
            merchant_cashback_id
        );
        self.inner
            .get(&url, None, ApiNames::GET_REVERESED_CASHBACK_DETAILS)
    }
}

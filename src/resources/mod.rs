//! Resource clients and request helpers for PayPay OPA API groups.

mod account;
mod cashback;
mod code;
mod payment;
mod pending;
mod preauth;
mod user;

use chrono::Utc;
use serde_json::{Map, Number, Value};

use crate::error::{Error, Result};

pub use account::Account;
pub use cashback::Cashback;
pub use code::Code;
pub use payment::Payment;
pub use pending::Pending;
pub use preauth::Preauth;
pub use user::User;

/// Converts JSON-like input into an optional request body.
///
/// Resource methods use this trait so callers can pass owned [`Value`],
/// borrowed [`Value`], or `Option<Value>` without manual conversion.
pub trait IntoOptionalValue {
    /// Converts the input into an optional JSON request body.
    fn into_optional_value(self) -> Option<Value>;
}

impl IntoOptionalValue for Value {
    fn into_optional_value(self) -> Option<Value> {
        Some(self)
    }
}

impl IntoOptionalValue for Option<Value> {
    fn into_optional_value(self) -> Option<Value> {
        self
    }
}

impl IntoOptionalValue for &Value {
    fn into_optional_value(self) -> Option<Value> {
        Some(self.clone())
    }
}

/// Converts string-like input into an optional identifier.
///
/// Resource methods use this trait so callers can pass owned strings,
/// borrowed strings, or optional strings for PayPay identifiers.
pub trait IntoOptionalString {
    /// Converts the input into an optional identifier string.
    fn into_optional_string(self) -> Option<String>;
}

impl IntoOptionalString for String {
    fn into_optional_string(self) -> Option<String> {
        Some(self)
    }
}

impl IntoOptionalString for &str {
    fn into_optional_string(self) -> Option<String> {
        Some(self.to_owned())
    }
}

impl IntoOptionalString for &String {
    fn into_optional_string(self) -> Option<String> {
        Some(self.clone())
    }
}

impl IntoOptionalString for Option<String> {
    fn into_optional_string(self) -> Option<String> {
        self
    }
}

impl IntoOptionalString for Option<&str> {
    fn into_optional_string(self) -> Option<String> {
        self.map(ToOwned::to_owned)
    }
}

impl IntoOptionalString for Option<&String> {
    fn into_optional_string(self) -> Option<String> {
        self.cloned()
    }
}

pub(crate) fn optional_data<D: IntoOptionalValue>(data: D) -> Value {
    data.into_optional_value().unwrap_or_else(empty_object)
}

pub(crate) fn empty_object() -> Value {
    Value::Object(Map::new())
}

pub(crate) fn required_id<S: IntoOptionalString>(id: S, field: &str) -> Result<String> {
    id.into_optional_string()
        .ok_or_else(|| missing_request_params(field))
}

pub(crate) fn required_query_id<S: IntoOptionalString>(id: S, field: &str) -> Result<String> {
    id.into_optional_string()
        .ok_or_else(|| Error::validation(format!("MISSING QUERY PARAM for {field}")))
}

pub(crate) fn add_requested_at(data: &mut Value) -> Result<()> {
    let map = object_mut(data)?;
    if !map.contains_key("requestedAt") {
        map.insert(
            "requestedAt".to_owned(),
            Value::Number(Number::from(Utc::now().timestamp())),
        );
    }
    Ok(())
}

pub(crate) fn require_field(data: &Value, field: &str) -> Result<()> {
    let map = object(data)?;
    if map.contains_key(field) {
        Ok(())
    } else {
        Err(missing_request_params(field))
    }
}

pub(crate) fn require_amount(data: &Value) -> Result<()> {
    require_amount_with_messages(data, "amount", "currency")
}

pub(crate) fn require_amount_with_messages(
    data: &Value,
    missing_amount: &str,
    missing_currency: &str,
) -> Result<()> {
    let data = object(data)?;
    let amount = data
        .get("amount")
        .and_then(Value::as_object)
        .ok_or_else(|| missing_request_params(missing_amount))?;

    let amount_value = amount
        .get("amount")
        .ok_or_else(|| missing_request_params(missing_amount))?;
    if !(amount_value.is_i64() || amount_value.is_u64()) {
        return Err(Error::validation("Amount should be of type integer"));
    }

    if !amount.contains_key("currency") {
        return Err(missing_request_params(missing_currency));
    }

    Ok(())
}

pub(crate) fn require_amount_shape(
    data: &Value,
    missing_amount: &str,
    missing_currency: &str,
) -> Result<()> {
    let data = object(data)?;
    let amount = data
        .get("amount")
        .and_then(Value::as_object)
        .ok_or_else(|| missing_request_params(missing_amount))?;

    if !amount.contains_key("amount") {
        return Err(missing_request_params(missing_amount));
    }

    if !amount.contains_key("currency") {
        return Err(missing_request_params(missing_currency));
    }

    Ok(())
}

pub(crate) fn require_order_items(data: &Value) -> Result<()> {
    let Some(order_items) = object(data)?.get("orderItems") else {
        return Ok(());
    };

    let Some(order_items) = order_items.as_array() else {
        return Err(missing_request_params("orderItem Name"));
    };

    for item in order_items {
        let item = item
            .as_object()
            .ok_or_else(|| missing_request_params("orderItem Name"))?;
        if !item.contains_key("name") {
            return Err(missing_request_params("orderItem Name"));
        }
        if !item.contains_key("quantity") {
            return Err(missing_request_params("orderItem quantity"));
        }

        let unit_price = item
            .get("unitPrice")
            .and_then(Value::as_object)
            .ok_or_else(|| missing_request_params("orderItem.amount.unitPrice"))?;
        if !unit_price.contains_key("amount") {
            return Err(missing_request_params("orderItem.amount.unitPrice"));
        }
        if !unit_price.contains_key("currency") {
            return Err(missing_request_params("orderItem.amount.currency"));
        }
    }

    Ok(())
}

pub(crate) fn require_integer_field_if_present(data: &Value, field: &str) -> Result<()> {
    let Some(value) = object(data)?.get(field) else {
        return Ok(());
    };

    if value.is_i64() || value.is_u64() {
        Ok(())
    } else {
        Err(Error::validation(format!(
            "{field} should be of type integer (EPOCH)"
        )))
    }
}

pub(crate) fn missing_request_params(field: &str) -> Error {
    Error::validation(format!("MISSING REQUEST PARAMS for {field}"))
}

fn object(data: &Value) -> Result<&Map<String, Value>> {
    data.as_object()
        .ok_or_else(|| Error::validation("request data must be a JSON object"))
}

fn object_mut(data: &mut Value) -> Result<&mut Map<String, Value>> {
    data.as_object_mut()
        .ok_or_else(|| Error::validation("request data must be a JSON object"))
}

#![warn(missing_docs)]

//! Rust port of the PayPay OPA Python SDK.
//!
//! The public surface mirrors the Python package closely: a [`Client`] owns
//! resource handles named `code`, `payment`, `preauth`, `pending`, `cashback`,
//! `account`, and `user`.

/// HTTP client, authentication, and request signing support.
pub mod client;
/// PayPay OPA API names, endpoint paths, and status constants.
pub mod constants;
/// Error and result types returned by the SDK.
pub mod error;
/// Resource clients for PayPay OPA API groups.
pub mod resources;

pub use client::{Auth, Client, ClientOptions};
pub use error::{Error, Result, ServerError, SignatureVerificationError};
pub use resources::{
    Account, Cashback, Code, IntoOptionalString, IntoOptionalValue, Payment, Pending, Preauth, User,
};
pub use serde_json::{json, Value};

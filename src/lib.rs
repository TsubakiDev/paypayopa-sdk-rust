//! Rust port of the PayPay OPA Python SDK.
//!
//! The public surface mirrors the Python package closely: a [`Client`] owns
//! resource handles named `code`, `payment`, `preauth`, `pending`, `cashback`,
//! `account`, and `user`.

pub mod client;
pub mod constants;
pub mod error;
pub mod resources;

pub use client::{Auth, Client, ClientOptions};
pub use error::{Error, Result, ServerError, SignatureVerificationError};
pub use resources::{
    Account, Cashback, Code, IntoOptionalString, IntoOptionalValue, Payment, Pending, Preauth, User,
};
pub use serde_json::{json, Value};

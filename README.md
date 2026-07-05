# PayPay OPA SDK - Rust

Rust stable port of the PayPay OPA Python SDK.

The crate keeps the same PayPay API coverage as the Python package while using Rust-style naming and error handling:

- resources are available as `client.code`, `client.payment`, `client.preauth`, `client.pending`, `client.cashback`, `client.account`, and `client.user`
- request payloads use `serde_json::Value`, so the SDK stays close to the original dynamic JSON API
- methods return `paypayopa::Result<serde_json::Value>`
- HTTP calls are synchronous through `reqwest::blocking`, matching the Python SDK's `requests` behavior

## Install

```toml
[dependencies]
paypayopa-sdk-rust = "<version>"
```

## Usage

```rust
use paypayopa::{json, Client, ClientOptions, Result};

fn main() -> Result<()> {
    let client = Client::new(("API_KEY", "API_SECRET"));

    client.set_assume_merchant("MERCHANT_ID");

    let request = json!({
        "merchantPaymentId": "merchant-payment-id",
        "codeType": "ORDER_QR",
        "redirectUrl": "https://example.com",
        "redirectType": "WEB_LINK",
        "orderDescription": "Example order",
        "amount": {
            "amount": 1,
            "currency": "JPY"
        }
    });

    let response = client.code.create_qr_code(request)?;
    println!("{response}");

    Ok(())
}
```

Production mode:

```rust
use paypayopa::{Client, ClientOptions};

let client = Client::with_options(
    ("API_KEY", "API_SECRET"),
    ClientOptions::production(),
);
```

Custom base URL, useful for tests:

```rust
use paypayopa::{Client, ClientOptions};

let client = Client::with_options(
    ("key_id", "key_secret"),
    ClientOptions::with_base_url("http://127.0.0.1:8080"),
);
```

## Resource Methods

```text
client.code.create_qr_code(data)
client.code.get_payment_details(merchant_payment_id)
client.code.delete_qr_code(code_id)

client.payment.create(data)
client.payment.get_payment_details(merchant_payment_id)
client.payment.cancel_payment(merchant_payment_id)
client.payment.refund_payment(data)
client.payment.refund_details(merchant_refund_id)
client.payment.capture_payment(data)
client.payment.create_continuous_payment(data)
client.payment.revert_payment(data)

client.preauth.pre_authorize_create(data)

client.pending.create_pending_payment(data)
client.pending.get_payment_details(merchant_payment_id)
client.pending.cancel_payment(merchant_payment_id)
client.pending.refund_payment(data)
client.pending.refund_details(merchant_refund_id)

client.cashback.give_cashback(data)
client.cashback.check_cashback_detail(merchant_cashback_id)
client.cashback.reverse_cashback(data)
client.cashback.check_cashback_reversal_detail(merchant_cashback_reversal_id, merchant_cashback_id)

client.account.create_qr_session(data)

client.user.get_authorization_status(user_authorization_id)
client.user.unlink_user_authorization(user_authorization_id)
client.user.unlink_user_athorization(user_authorization_id) // Python-compatible spelling
```

## Verification

```sh
cargo fmt
cargo test
```

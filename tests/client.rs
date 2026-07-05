use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::Duration;

use paypayopa::{json, Client, ClientOptions, Error, Value};

fn spawn_json_server(status: u16, body: &'static str) -> (String, Receiver<String>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind test server");
    let address = listener.local_addr().expect("test server address");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("accept request");
        stream
            .set_read_timeout(Some(Duration::from_secs(2)))
            .expect("set read timeout");

        let mut request = Vec::new();
        let mut buffer = [0_u8; 1024];
        loop {
            let read = stream.read(&mut buffer).expect("read request");
            if read == 0 {
                break;
            }
            request.extend_from_slice(&buffer[..read]);
            if request_complete(&request) {
                break;
            }
        }

        let response = format!(
            "HTTP/1.1 {status} OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
            body.len()
        );
        stream
            .write_all(response.as_bytes())
            .expect("write response");
        tx.send(String::from_utf8(request).expect("utf8 request"))
            .expect("send request");
    });

    (format!("http://{}", address), rx)
}

fn request_complete(request: &[u8]) -> bool {
    let Some(header_end) = find_bytes(request, b"\r\n\r\n") else {
        return false;
    };
    let headers = String::from_utf8_lossy(&request[..header_end]);
    let content_length = headers
        .lines()
        .find_map(|line| line.strip_prefix("content-length: "))
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(0);
    request.len() >= header_end + 4 + content_length
}

fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

fn request_body(request: &str) -> Value {
    let (_, body) = request.split_once("\r\n\r\n").expect("request body");
    serde_json::from_str(body).expect("json body")
}

#[test]
fn auth_header_matches_python_sdk_algorithm() {
    let header = Client::auth_header_with_nonce_timestamp(
        "key",
        "secret",
        "POST",
        "/v2/codes",
        "application/json;charset=UTF-8",
        Some(r#"{"a":1}"#),
        "nonce",
        "12345",
    );

    assert_eq!(
        header,
        "hmac OPA-Auth:key:Te98rbwV62D+T6MRB4AIc32VPE5a+tUrai/wFCAGgx0=:nonce:12345:20yInMqy99Jxgt2roxoSvw=="
    );
}

#[test]
fn create_qr_code_sends_paypay_request_shape() {
    let (base_url, rx) = spawn_json_server(200, r#"{"ok":true}"#);
    let client = Client::with_options(
        ("key_id", "key_secret"),
        ClientOptions::with_base_url(base_url),
    );
    client.set_assume_merchant("merchant-id");

    let response = client
        .code
        .create_qr_code(json!({
            "merchantPaymentId": "merchant-payment-id",
            "codeType": "ORDER_QR",
            "requestedAt": 1,
            "amount": {
                "amount": 1,
                "currency": "JPY"
            }
        }))
        .expect("create qr code response");

    assert_eq!(response, json!({"ok": true}));

    let request = rx.recv_timeout(Duration::from_secs(2)).expect("request");
    let lower = request.to_ascii_lowercase();
    assert!(request.starts_with("POST /v2/codes HTTP/1.1"));
    assert!(lower.contains("authorization: hmac opa-auth:key_id:"));
    assert!(lower.contains("content-type: application/json;charset=utf-8"));
    assert!(lower.contains("x-assume-merchant: merchant-id"));
    assert_eq!(
        request_body(&request),
        json!({
            "merchantPaymentId": "merchant-payment-id",
            "codeType": "ORDER_QR",
            "requestedAt": 1,
            "amount": {
                "amount": 1,
                "currency": "JPY"
            }
        })
    );
}

#[test]
fn get_authorization_status_uses_query_parameter() {
    let (base_url, rx) = spawn_json_server(200, r#"{"status":"ok"}"#);
    let client = Client::with_options(
        ("key_id", "key_secret"),
        ClientOptions::with_base_url(base_url),
    );

    let response = client
        .user
        .get_authorization_status("fakeid")
        .expect("auth status response");

    assert_eq!(response, json!({"status": "ok"}));

    let request = rx.recv_timeout(Duration::from_secs(2)).expect("request");
    assert!(request.starts_with("GET /v2/user/authorizations?userAuthorizationId=fakeid HTTP/1.1"));
}

#[test]
fn payment_validation_uses_result_errors() {
    let client = Client::new(("key_id", "key_secret"));

    let error = client
        .payment
        .create(json!({
            "merchantPaymentId": "merchant-payment-id",
            "amount": {
                "amount": "1",
                "currency": "JPY"
            }
        }))
        .expect_err("invalid amount should fail");

    match error {
        Error::Validation(message) => assert_eq!(message, "Amount should be of type integer"),
        other => panic!("unexpected error: {other}"),
    }
}

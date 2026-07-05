use std::sync::{Arc, RwLock};

use base64::{engine::general_purpose::STANDARD, Engine as _};
use chrono::{Duration, Utc};
use hmac::{Hmac, Mac};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use md5::{Digest, Md5};
use reqwest::blocking::Client as HttpClient;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::Sha256;
use uuid::Uuid;

use crate::constants::{HttpStatusCode, Url};
use crate::error::Result;
use crate::resources::{Account, Cashback, Code, Payment, Pending, Preauth, User};

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Auth {
    pub api_key: String,
    pub api_secret: String,
}

impl Auth {
    pub fn new(api_key: impl Into<String>, api_secret: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            api_secret: api_secret.into(),
        }
    }
}

#[derive(Clone)]
pub struct ClientOptions {
    pub production_mode: bool,
    pub perf_mode: bool,
    pub base_url: Option<String>,
    pub http_client: Option<HttpClient>,
}

impl Default for ClientOptions {
    fn default() -> Self {
        Self {
            production_mode: false,
            perf_mode: false,
            base_url: None,
            http_client: None,
        }
    }
}

impl ClientOptions {
    pub fn sandbox() -> Self {
        Self::default()
    }

    pub fn production() -> Self {
        Self {
            production_mode: true,
            ..Self::default()
        }
    }

    pub fn perf() -> Self {
        Self {
            perf_mode: true,
            ..Self::default()
        }
    }

    pub fn with_base_url(base_url: impl Into<String>) -> Self {
        Self {
            base_url: Some(base_url.into()),
            ..Self::default()
        }
    }
}

#[derive(Clone)]
pub struct Client {
    inner: Arc<ClientInner>,
    pub code: Code,
    pub payment: Payment,
    pub preauth: Preauth,
    pub pending: Pending,
    pub cashback: Cashback,
    pub account: Account,
    pub user: User,
}

pub(crate) struct ClientInner {
    http_client: HttpClient,
    auth: Auth,
    production_mode: bool,
    perf_mode: bool,
    base_url: String,
    assume_merchant: RwLock<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct JwtClaims {
    #[serde(skip_serializing_if = "Option::is_none")]
    iss: Option<String>,
    exp: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nonce: Option<String>,
    #[serde(rename = "redirectUrl", skip_serializing_if = "Option::is_none")]
    redirect_url: Option<String>,
    #[serde(rename = "referenceId", skip_serializing_if = "Option::is_none")]
    reference_id: Option<String>,
    #[serde(rename = "deviceId", skip_serializing_if = "Option::is_none")]
    device_id: Option<String>,
    #[serde(rename = "phoneNumber", skip_serializing_if = "Option::is_none")]
    phone_number: Option<String>,
    #[serde(
        rename = "userAuthorizationId",
        skip_serializing_if = "Option::is_none"
    )]
    user_authorization_id: Option<String>,
}

impl Client {
    pub fn new<K, S>(auth: (K, S)) -> Self
    where
        K: Into<String>,
        S: Into<String>,
    {
        Self::with_options(auth, ClientOptions::default())
    }

    pub fn with_options<K, S>(auth: (K, S), options: ClientOptions) -> Self
    where
        K: Into<String>,
        S: Into<String>,
    {
        let base_url = Self::set_base_url(&options);
        let inner = Arc::new(ClientInner {
            http_client: options.http_client.unwrap_or_default(),
            auth: Auth::new(auth.0, auth.1),
            production_mode: options.production_mode,
            perf_mode: options.perf_mode,
            base_url,
            assume_merchant: RwLock::new(String::new()),
        });

        Self {
            code: Code::new(inner.clone()),
            payment: Payment::new(inner.clone()),
            preauth: Preauth::new(inner.clone()),
            pending: Pending::new(inner.clone()),
            cashback: Cashback::new(inner.clone()),
            account: Account::new(inner.clone()),
            user: User::new(inner.clone()),
            inner,
        }
    }

    pub fn code(&self) -> &Code {
        &self.code
    }

    pub fn payment(&self) -> &Payment {
        &self.payment
    }

    pub fn preauth(&self) -> &Preauth {
        &self.preauth
    }

    pub fn pending(&self) -> &Pending {
        &self.pending
    }

    pub fn cashback(&self) -> &Cashback {
        &self.cashback
    }

    pub fn account(&self) -> &Account {
        &self.account
    }

    pub fn user(&self) -> &User {
        &self.user
    }

    pub fn get_version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    pub fn base_url(&self) -> &str {
        &self.inner.base_url
    }

    pub fn production_mode(&self) -> bool {
        self.inner.production_mode
    }

    pub fn perf_mode(&self) -> bool {
        self.inner.perf_mode
    }

    pub fn assume_merchant(&self) -> String {
        self.inner
            .assume_merchant
            .read()
            .map(|merchant| merchant.clone())
            .unwrap_or_default()
    }

    pub fn set_assume_merchant(&self, merchant: impl AsRef<str>) {
        let merchant = merchant.as_ref();
        if merchant.is_empty() {
            return;
        }
        if let Ok(mut assume_merchant) = self.inner.assume_merchant.write() {
            *assume_merchant = merchant.to_owned();
        }
    }

    pub fn encode_jwt(
        &self,
        secret: &str,
        scope: Option<&str>,
        redirect_url: Option<&str>,
        reference_id: Option<&str>,
        device_id: Option<&str>,
        phone_number: Option<&str>,
    ) -> Result<String> {
        let secret = STANDARD.decode(secret)?;
        let exp = (Utc::now() + Duration::minutes(5)).timestamp() as usize;
        let claims = JwtClaims {
            iss: Some("merchant".to_owned()),
            exp,
            scope: Some(scope.unwrap_or("direct_debit").to_owned()),
            nonce: Some(short_uuid()),
            redirect_url: redirect_url.map(ToOwned::to_owned),
            reference_id: Some(
                reference_id
                    .map(ToOwned::to_owned)
                    .unwrap_or_else(short_uuid),
            ),
            device_id: Some(device_id.unwrap_or("").to_owned()),
            phone_number: Some(phone_number.unwrap_or("").to_owned()),
            user_authorization_id: None,
        };

        Ok(encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(&secret),
        )?)
    }

    pub fn decode_jwt(
        &self,
        secret: &str,
        token: &str,
    ) -> Result<(Option<String>, Option<String>)> {
        let secret = STANDARD.decode(secret)?;
        let claims = decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(&secret),
            &Validation::new(Algorithm::HS256),
        )?
        .claims;

        Ok((claims.user_authorization_id, claims.reference_id))
    }

    pub fn auth_header(
        &self,
        api_key: &str,
        api_secret: &str,
        method: &str,
        resource: &str,
        content_type: Option<&str>,
        request_body: Option<&str>,
    ) -> String {
        Self::auth_header_with_nonce_timestamp(
            api_key,
            api_secret,
            method,
            resource,
            content_type.unwrap_or("empty"),
            request_body,
            &short_uuid(),
            &Utc::now().timestamp().to_string(),
        )
    }

    pub fn auth_header_with_nonce_timestamp(
        api_key: &str,
        api_secret: &str,
        method: &str,
        resource: &str,
        content_type: &str,
        request_body: Option<&str>,
        nonce: &str,
        timestamp: &str,
    ) -> String {
        let body_hash = match request_body {
            Some(request_body) => {
                let mut md5 = Md5::new();
                md5.update(content_type.as_bytes());
                md5.update(request_body.as_bytes());
                STANDARD.encode(md5.finalize())
            }
            None => "empty".to_owned(),
        };

        let signature_list = [
            resource,
            method,
            nonce,
            timestamp,
            content_type,
            body_hash.as_str(),
        ]
        .join("\n");

        let mut hmac = HmacSha256::new_from_slice(api_secret.as_bytes())
            .expect("HMAC accepts keys of any size");
        hmac.update(signature_list.as_bytes());
        let signature = STANDARD.encode(hmac.finalize().into_bytes());
        format!("hmac OPA-Auth:{api_key}:{signature}:{nonce}:{timestamp}:{body_hash}")
    }

    pub fn get(&self, path: &str, params: Option<&Value>, api_id: &str) -> Result<Value> {
        self.inner.get(path, params, api_id)
    }

    pub fn post(&self, path: &str, data: Option<&Value>, api_id: &str) -> Result<Value> {
        self.inner.post(path, data, api_id)
    }

    pub fn patch(&self, path: &str, data: Option<&Value>, api_id: &str) -> Result<Value> {
        self.inner.patch(path, data, api_id)
    }

    pub fn delete(&self, path: &str, data: Option<&Value>, api_id: &str) -> Result<Value> {
        self.inner.delete(path, data, api_id)
    }

    pub fn put(&self, path: &str, data: Option<&Value>, api_id: &str) -> Result<Value> {
        self.inner.put(path, data, api_id)
    }

    fn set_base_url(options: &ClientOptions) -> String {
        if let Some(base_url) = &options.base_url {
            return base_url.clone();
        }

        if options.perf_mode {
            return Url::PERF_BASE_URL.to_owned();
        }

        if options.production_mode {
            Url::PRODUCTION_BASE_URL.to_owned()
        } else {
            Url::SANDBOX_BASE_URL.to_owned()
        }
    }
}

impl ClientInner {
    pub(crate) fn get(&self, path: &str, params: Option<&Value>, api_id: &str) -> Result<Value> {
        self.dispatch(Method::GET, "GET", path, None, params, api_id)
    }

    pub(crate) fn post(&self, path: &str, data: Option<&Value>, api_id: &str) -> Result<Value> {
        self.dispatch(Method::POST, "POST", path, data, None, api_id)
    }

    pub(crate) fn patch(&self, path: &str, data: Option<&Value>, api_id: &str) -> Result<Value> {
        self.dispatch(Method::PATCH, "PATCH", path, data, None, api_id)
    }

    pub(crate) fn delete(&self, path: &str, data: Option<&Value>, api_id: &str) -> Result<Value> {
        self.dispatch(Method::DELETE, "DELETE", path, data, None, api_id)
    }

    pub(crate) fn put(&self, path: &str, data: Option<&Value>, api_id: &str) -> Result<Value> {
        self.dispatch(Method::PUT, "PUT", path, data, None, api_id)
    }

    fn dispatch(
        &self,
        method: Method,
        method_name: &str,
        path: &str,
        data: Option<&Value>,
        params: Option<&Value>,
        api_id: &str,
    ) -> Result<Value> {
        let (request_body, content_type) = match data {
            Some(data) => (
                Some(serde_json::to_string(data)?),
                "application/json;charset=UTF-8",
            ),
            None => (None, "empty"),
        };

        let auth_header = Client::auth_header_with_nonce_timestamp(
            &self.auth.api_key,
            &self.auth.api_secret,
            method_name,
            path,
            content_type,
            request_body.as_deref(),
            &short_uuid(),
            &Utc::now().timestamp().to_string(),
        );

        let url = format!("{}{}", self.base_url, path);
        let assume_merchant = self
            .assume_merchant
            .read()
            .map(|merchant| merchant.clone())
            .unwrap_or_default();

        let mut request = self
            .http_client
            .request(method, url)
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json;charset=UTF-8")
            .header("X-ASSUME-MERCHANT", assume_merchant);

        if let Some(params) = params {
            let query = query_pairs(params);
            if !query.is_empty() {
                request = request.query(&query);
            }
        }

        if let Some(request_body) = request_body {
            request = request.body(request_body);
        }

        let response = request.send()?;
        let status = response.status().as_u16();
        let json_response: Value = response.json()?;

        if (HttpStatusCode::OK..HttpStatusCode::REDIRECT).contains(&status) {
            Ok(json_response)
        } else {
            if let Some(result_info) = json_response.get("resultInfo") {
                if let (Some(code), Some(code_id)) = (
                    result_info.get("code").and_then(Value::as_str),
                    result_info.get("codeId").and_then(Value::as_str),
                ) {
                    println!(
                        "This link should help you to troubleshoot the error: {}?api_name={}&code={}&code_id={}",
                        Url::RESOLVE,
                        api_id,
                        code,
                        code_id
                    );
                }
            }
            Ok(json_response)
        }
    }
}

fn short_uuid() -> String {
    Uuid::new_v4()
        .simple()
        .to_string()
        .chars()
        .take(8)
        .collect()
}

fn query_pairs(params: &Value) -> Vec<(String, String)> {
    match params {
        Value::Object(map) => map
            .iter()
            .map(|(key, value)| (key.clone(), query_value_to_string(value)))
            .collect(),
        _ => Vec::new(),
    }
}

fn query_value_to_string(value: &Value) -> String {
    match value {
        Value::Null => String::new(),
        Value::Bool(value) => value.to_string(),
        Value::Number(value) => value.to_string(),
        Value::String(value) => value.clone(),
        Value::Array(values) => values
            .iter()
            .map(query_value_to_string)
            .collect::<Vec<_>>()
            .join(","),
        Value::Object(map) => map
            .values()
            .next()
            .map(query_value_to_string)
            .unwrap_or_default(),
    }
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use bytes::Bytes;
use data_encoding::BASE64URL;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE},
    Client, RequestBuilder, Url,
};
use std::path::Path;
use std::{fmt::Debug, sync::Arc};

use libparsec_platform_http_proxy::ProxyConfig;
use libparsec_protocol::{api_version_major_to_full, API_LATEST_VERSION};
use libparsec_types::prelude::*;

#[cfg(feature = "test-with-testbed")]
use crate::testbed::{get_send_hook, SendHookConfig};
use crate::{
    error::{ConnectionError, ConnectionResult},
    API_VERSION_HEADER_NAME, PARSEC_CONTENT_TYPE,
};

const API_LATEST_MAJOR_VERSION: u32 = API_LATEST_VERSION.version;

/// Method name that will be used for the `Authorization` header
pub const PARSEC_AUTH_METHOD: &str = "PARSEC-PASSWORD-HMAC-BLAKE2B";

#[derive(Debug)]
pub struct AccountPassword {
    pub email: String,
    pub time_provider: TimeProvider,
    pub hmac_key: SecretKey,
}

/// Send commands in an authenticated account context.
#[derive(Debug)]
pub struct AuthenticatedAccountCmds {
    /// HTTP Client that contain the basic configuration to communicate with the server.
    client: Client,
    addr: ParsecAuthenticatedAccountAddr,
    url: Url,
    #[cfg(feature = "test-with-testbed")]
    send_hook: SendHookConfig,
    time_provider: TimeProvider,
    account: Arc<AccountPassword>,
}

impl AuthenticatedAccountCmds {
    pub fn new(
        config_dir: &Path,
        addr: ParsecAuthenticatedAccountAddr,
        proxy: ProxyConfig,
        account: Arc<AccountPassword>,
    ) -> anyhow::Result<Self> {
        let client = crate::build_client_with_proxy(proxy)?;
        Ok(Self::from_client(client, config_dir, addr, account))
    }

    pub fn from_client(
        client: Client,
        _config_dir: &Path,
        addr: ParsecAuthenticatedAccountAddr,
        account: Arc<AccountPassword>,
    ) -> Self {
        let url = addr.to_authenticated_account_url();

        #[cfg(feature = "test-with-testbed")]
        let send_hook = get_send_hook(_config_dir);

        Self {
            client,
            addr,
            url,
            #[cfg(feature = "test-with-testbed")]
            send_hook,
            time_provider: account.time_provider.new_child(),
            account,
        }
    }

    pub fn addr(&self) -> &ParsecAuthenticatedAccountAddr {
        &self.addr
    }

    pub async fn send<T>(&self, request: T) -> ConnectionResult<<T>::Response>
    where
        T: ProtocolRequest<API_LATEST_MAJOR_VERSION> + Debug + 'static,
    {
        #[cfg(feature = "test-with-testbed")]
        let request = {
            match self.send_hook.high_level_send(request).await {
                crate::testbed::HighLevelSendResult::Resolved(rep) => return rep,
                crate::testbed::HighLevelSendResult::PassToLowLevel(req) => req,
            }
        };

        let request_body = request.api_dump().expect("Unexpected serialization error");

        // Split non-generic code out of `send` to limit the amount of code generated
        // by monomorphization
        let api_version = api_version_major_to_full(T::API_MAJOR_VERSION);
        let response_body = self.internal_send(api_version, request_body).await?;

        Ok(T::api_load_response(&response_body)?)
    }

    async fn internal_send(
        &self,
        api_version: &ApiVersion,
        request_body: Vec<u8>,
    ) -> Result<Bytes, ConnectionError> {
        let api_version_header_value = HeaderValue::from_str(&api_version.to_string())
            .expect("api version must contains valid char");
        let request_builder = self.client.post(self.url.clone());

        let req = prepare_request(
            request_builder,
            api_version_header_value,
            request_body,
            &self.time_provider,
            &self.account,
        );

        #[cfg(feature = "test-with-testbed")]
        let resp = self.send_hook.low_level_send(req).await?;
        #[cfg(not(feature = "test-with-testbed"))]
        let resp = req.send().await?;

        match resp.status().as_u16() {
            200 => {
                let response_body = resp.bytes().await?;
                Ok(response_body)
            }

            // HTTP codes used by Parsec

            401 => Err(ConnectionError::MissingAuthenticationInfo),
            403 => Err(ConnectionError::BadAuthenticationInfo),
            // No 404: no organization
            406 => Err(ConnectionError::BadAcceptType),
            415 => Err(ConnectionError::BadContent),
            422 => Err(crate::error::unsupported_api_version_from_headers(
                *api_version, resp.headers(),
            )),
            // No 460: no organization
            // No 461/462/463: no user here
            // No 464: no organization here
            // No 498: invitation token cannot expire like an authentication token

            // Other HTTP codes

            // We typically use HTTP 503 in the tests to simulate server offline,
            // so it should behave just like if we were not able to connect
            // On top of that we should also handle 502 and 504 as they are related
            // to a gateway.
            #[allow(clippy::manual_range_patterns)]
            502 // Bad Gateway
            | 503 // Service Unavailable
            | 504 // Gateway Timeout
            => Err(ConnectionError::NoResponse(None)),

            // Finally all other HTTP codes are not supposed to occur (well except if
            // an HTTP proxy starts modifying the response, but that's another story...)
            _ => Err(ConnectionError::InvalidResponseStatus(resp.status())),
        }
    }
}

/// Prepare a new request, the body will be added to the Request using [RequestBuilder::body]
fn prepare_request(
    request_builder: RequestBuilder,
    api_version_header_value: HeaderValue,
    body: Vec<u8>,
    time_provider: &TimeProvider,
    account: &AccountPassword,
) -> RequestBuilder {
    // TODO: user hmac key is sent alongside the request
    //       until it can be provided by the internal state.
    let mut content_headers = HeaderMap::with_capacity(4 + 1);
    content_headers.insert(API_VERSION_HEADER_NAME, api_version_header_value);
    content_headers.insert(CONTENT_TYPE, HeaderValue::from_static(PARSEC_CONTENT_TYPE));
    content_headers.insert(
        CONTENT_LENGTH,
        HeaderValue::from_str(&body.len().to_string()).expect("numeric value are valid char"),
    );
    content_headers.insert(
        reqwest::header::HeaderName::from_static("x-hmac-key"),
        HeaderValue::from_str(&BASE64URL.encode(account.hmac_key.as_ref())).expect("Always valid"),
    );
    let body_sha256 = libparsec_crypto::HashDigest::from_data(&body);
    let authorization_header_value = HeaderValue::from_str(&generate_authorization_header_value(
        &BASE64URL.encode(account.email.as_bytes()),
        time_provider.now(),
        &body_sha256,
        &account.hmac_key,
    ))
    .expect("always valid");
    content_headers.insert(AUTHORIZATION, authorization_header_value);

    request_builder.headers(content_headers).body(body)
}

fn generate_authorization_header_value(
    email_base64: &str,
    now: DateTime,
    body_sha256: &HashDigest,
    secret: &SecretKey,
) -> String {
    let timestamp = now.as_timestamp_seconds().to_string();
    let content = [
        PARSEC_AUTH_METHOD.as_bytes(),
        email_base64.as_bytes(),
        timestamp.as_bytes(),
        body_sha256.as_ref(),
    ]
    .join(b".".as_slice());
    let signature = secret.hmac_full(&content);
    let b64_signature = BASE64URL.encode(&signature);

    format!("Bearer {PARSEC_AUTH_METHOD}.{email_base64}.{timestamp}.{b64_signature}")
}

#[cfg(test)]
#[path = "../tests/unit/authenticated_account.rs"]
mod tests;

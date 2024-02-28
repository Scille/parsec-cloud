// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

pub(crate) mod sse;

use bytes::Bytes;
use data_encoding::BASE64URL;
use eventsource_stream::Eventsource;
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE},
    Client, RequestBuilder, StatusCode, Url,
};
use std::{fmt::Debug, marker::PhantomData, path::Path, sync::Arc};

use libparsec_platform_http_proxy::ProxyConfig;
use libparsec_protocol::{api_version_major_to_full, API_LATEST_MAJOR_VERSION};
use libparsec_types::prelude::*;

#[cfg(feature = "test-with-testbed")]
use crate::testbed::{get_send_hook, SendHookConfig};
use crate::{
    error::{ConnectionError, ConnectionResult},
    SSEConnectionError, SSEStream, API_VERSION_HEADER_NAME, PARSEC_CONTENT_TYPE,
};

use self::sse::EVENT_STREAM_CONTENT_TYPE;

/// Method name that will be used for the header `Authorization` to indicate that will be using this method.
pub const PARSEC_AUTH_METHOD: &str = "PARSEC-SIGN-ED25519";

/// Factory that send commands in a authenticated context.
#[derive(Debug)]
pub struct AuthenticatedCmds {
    /// HTTP Client that contain the basic configuration to communicate with the server.
    client: Client,
    url: Url,
    device: Arc<LocalDevice>,
    #[cfg(feature = "test-with-testbed")]
    send_hook: SendHookConfig,
}

impl AuthenticatedCmds {
    pub fn new(
        config_dir: &Path,
        device: Arc<LocalDevice>,
        proxy: ProxyConfig,
    ) -> anyhow::Result<Self> {
        let client = {
            let builder = reqwest::ClientBuilder::default().user_agent(crate::CLIENT_USER_AGENT);
            let builder = proxy.configure_http_client(builder);
            builder.build()?
        };
        Ok(Self::from_client(client, config_dir, device))
    }

    pub fn from_client(client: Client, _config_dir: &Path, device: Arc<LocalDevice>) -> Self {
        let url = device.organization_addr.to_authenticated_http_url();

        #[cfg(feature = "test-with-testbed")]
        let send_hook = get_send_hook(_config_dir);

        Self {
            client,
            url,
            device,
            #[cfg(feature = "test-with-testbed")]
            send_hook,
        }
    }

    pub fn addr(&self) -> &ParsecOrganizationAddr {
        &self.device.organization_addr
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

        let request_body = request.api_dump()?;

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
        let mut content_headers = HeaderMap::with_capacity(4);

        let api_version_header_value =
            HeaderValue::from_str(&api_version.to_string()).expect("always valid");
        content_headers.insert(API_VERSION_HEADER_NAME, api_version_header_value);
        content_headers.insert(CONTENT_TYPE, HeaderValue::from_static(PARSEC_CONTENT_TYPE));
        content_headers.insert(
            CONTENT_LENGTH,
            HeaderValue::from_str(&request_body.len().to_string())
                .expect("numeric value always valid"),
        );
        let authorization_header_value = HeaderValue::from_str(
            &generate_authorization_header_value(&self.device.device_id, &self.device.signing_key),
        )
        .expect("always valid");
        content_headers.insert(AUTHORIZATION, authorization_header_value);

        let request_builder = self
            .client
            .post(self.url.clone())
            .headers(content_headers)
            .body(request_body);

        #[cfg(feature = "test-with-testbed")]
        let resp = self.send_hook.low_level_send(request_builder).await?;
        #[cfg(not(feature = "test-with-testbed"))]
        let resp = request_builder.send().await?;

        match resp.status().as_u16() {
            200 => {
                let response_body = resp.bytes().await?;
                Ok(response_body)
            }
            415 => Err(ConnectionError::BadContent),
            422 => Err(crate::error::unsupported_api_version_from_headers(
                resp.headers(),
            )),
            460 => Err(ConnectionError::ExpiredOrganization),
            461 => Err(ConnectionError::RevokedUser),
            // We typically use HTTP 503 in the tests to simulate server offline,
            // so it should behave just like if we were not able to connect
            503 => Err(ConnectionError::NoResponse(None)),
            _ => Err(ConnectionError::InvalidResponseStatus(resp.status())),
        }
    }
}

// SSE method impls

impl AuthenticatedCmds {
    fn sse_request_builder<T>(&self, last_event_id: Option<&str>) -> RequestBuilder
    where
        T: ProtocolRequest<API_LATEST_MAJOR_VERSION> + Debug + 'static,
    {
        let url = {
            let mut url = self.url.clone();
            let mut psm = url
                .path_segments_mut()
                .expect("url is not a cannot-be-a-base");
            psm.push("events");
            drop(psm);
            url
        };

        let mut content_headers = HeaderMap::with_capacity(5);

        let api_version = api_version_major_to_full(T::API_MAJOR_VERSION);
        let api_version_header_value =
            HeaderValue::from_str(&api_version.to_string()).expect("always valid");
        content_headers.insert(API_VERSION_HEADER_NAME, api_version_header_value);
        // No Content-Type as this request is a GET
        content_headers.insert(CONTENT_LENGTH, HeaderValue::from_static("0"));
        content_headers.insert(ACCEPT, HeaderValue::from_static(EVENT_STREAM_CONTENT_TYPE));

        let authorization_header_value = HeaderValue::from_str(
            &generate_authorization_header_value(&self.device.device_id, &self.device.signing_key),
        )
        .expect("always valid");
        content_headers.insert(AUTHORIZATION, authorization_header_value);

        if let Some(last_event_id) = last_event_id {
            // Last event ID is passed as a utf8 string, hence it is possible
            // (in theory at least) that it cannot be encoded as a header value
            // (e.g. if it contains NULL bytes).
            //
            // In such unlikely case we cannot just ignore the event id and not
            // send a Last-Event-ID header as it would silently make us miss events.
            //
            // Instead we default to a dummy header value (i.e. empty string, so
            // the server will be forced to send us a `missed_events` event message
            // (just like if we had provided a very old but valid Last-Event-ID).
            let value =
                HeaderValue::from_str(last_event_id).unwrap_or(HeaderValue::from_static(""));
            content_headers.insert("Last-Event-ID", value);
        }

        self.client.get(url).headers(content_headers)
    }

    pub async fn start_sse<T>(
        &self,
        last_event_id: Option<String>,
    ) -> Result<SSEStream<T>, SSEConnectionError>
    where
        T: ProtocolRequest<API_LATEST_MAJOR_VERSION> + Debug + 'static,
    {
        let request_builder = self.sse_request_builder::<T>(last_event_id.as_deref());

        #[cfg(feature = "test-with-testbed")]
        let response = self
            .send_hook
            .low_level_send(request_builder)
            .await
            .map_err(SSEConnectionError::Transport)?;
        #[cfg(not(feature = "test-with-testbed"))]
        let response = request_builder
            .send()
            .await
            .map_err(SSEConnectionError::Transport)?;

        match response.status() {
            StatusCode::OK => {}
            status => return Err(SSEConnectionError::InvalidStatusCode(status)),
        }

        let content_type = response
            .headers()
            .get(CONTENT_TYPE)
            .ok_or_else(|| SSEConnectionError::InvalidContentType(HeaderValue::from_static("")))?;
        let mime_type = content_type
            .to_str()
            .map_err(|_| SSEConnectionError::InvalidContentType(content_type.clone()))
            .map(|e| e.split(';').next().expect("first item always exists"))?;

        if mime_type != EVENT_STREAM_CONTENT_TYPE {
            return Err(SSEConnectionError::InvalidContentType(content_type.clone()));
        }

        let mut sse_stream = response.bytes_stream().eventsource();
        if let Some(last_event_id) = last_event_id {
            sse_stream.set_last_event_id(last_event_id);
        }
        let event_source = Box::pin(sse_stream);

        Ok(SSEStream::<T> {
            event_source,
            phantom: PhantomData,
        })
    }
}

/// Return the Bearer token to use for the `Authorization` header.
///
/// Authorization token format: `PARSEC-SIGN-ED25519.<b64_device_id>.<timestamp>.<b64_signature>`
/// with:
///     <b64_device_id> = base64(<device_id>)
///     <timestamp> = str(<seconds since UNIX epoch>)
///     <b64_signature> = base64(ed25519(`PARSEC-SIGN-ED25519.<b64_device_id>.<timestamp>`))
/// base64() is the URL-safe variant (https://tools.ietf.org/html/rfc4648#section-5).
fn generate_authorization_header_value(
    author: &DeviceID,
    author_signing_key: &SigningKey,
) -> String {
    let b64_device_id = BASE64URL.encode(author.to_string().as_bytes());
    let timestamp = chrono::Utc::now().timestamp().to_string();
    let header_and_body = format!("{}.{}.{}", PARSEC_AUTH_METHOD, &b64_device_id, &timestamp);
    let signature = author_signing_key.sign_only_signature(header_and_body.as_bytes());
    let b64_signature = BASE64URL.encode(&signature);

    format!("Bearer {}.{}", &header_and_body, &b64_signature)
}

#[cfg(test)]
#[path = "../../tests/unit/authenticated.rs"]
mod tests;

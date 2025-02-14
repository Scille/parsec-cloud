// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

pub(crate) mod sse;

use bytes::Bytes;
use data_encoding::BASE64URL;
use eventsource_stream::Eventsource;
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE},
    Client, RequestBuilder, Url,
};
use std::{fmt::Debug, marker::PhantomData, path::Path, sync::Arc};

use libparsec_platform_http_proxy::ProxyConfig;
use libparsec_protocol::{api_version_major_to_full, API_LATEST_MAJOR_VERSION};
use libparsec_types::prelude::*;

#[cfg(feature = "test-with-testbed")]
use crate::testbed::{get_send_hook, SendHookConfig};
use crate::{
    error::{ConnectionError, ConnectionResult},
    SSEStream, API_VERSION_HEADER_NAME, PARSEC_CONTENT_TYPE,
};

use self::sse::EVENT_STREAM_CONTENT_TYPE;

/// Method name that will be used for the header `Authorization` to indicate that will be using this method.
pub const PARSEC_AUTH_METHOD: &str = "PARSEC-SIGN-ED25519";

/// Send commands in an authenticated context.
///
/// This supports both the `authenticated` and `tos` cmds families.
/// From the client point of view, these families work pretty much
/// the same way, the only difference is that the server can return
/// "TOS must be accepted" error when using the `authenticated` family.
#[derive(Debug)]
pub struct AuthenticatedCmds {
    /// HTTP Client that contain the basic configuration to communicate with the server.
    client: Client,
    url: Url,
    device: Arc<LocalDevice>,
    #[cfg(feature = "test-with-testbed")]
    send_hook: SendHookConfig,
    /// Create a sub time provider so that we can send request (hence the timestamp in
    /// authentication header is not mocked) containing manifests with different timestamps.
    pub time_provider: TimeProvider,
}

impl AuthenticatedCmds {
    pub fn new(
        config_dir: &Path,
        device: Arc<LocalDevice>,
        proxy: ProxyConfig,
    ) -> anyhow::Result<Self> {
        let client = crate::build_client_with_proxy(proxy)?;
        Ok(Self::from_client(client, config_dir, device))
    }

    pub fn from_client(client: Client, _config_dir: &Path, device: Arc<LocalDevice>) -> Self {
        let url = device.organization_addr.to_authenticated_http_url();

        #[cfg(feature = "test-with-testbed")]
        let send_hook = get_send_hook(_config_dir);

        Self {
            client,
            url,
            time_provider: device.time_provider.new_child(),
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

        let request_body = request.api_dump().expect("Unexpected serialization error");

        let url = match T::FAMILY {
            ProtocolFamily::Authenticated => self.url.clone(),
            ProtocolFamily::Tos => {
                let mut url = self.url.clone();
                url.path_segments_mut()
                    .expect("Url is not cannot-be-a-base")
                    .push("tos");
                url
            }
            // Other families are not supported
            _ => {
                // TODO: Once const panic is available we can replace this runtime
                // error by doing the `match T:::FAMILY { ... }` in a const function.
                // (see https://rust-lang.github.io/rfcs/2345-const-panic.html)
                unreachable!("Unsupported family")
            }
        };

        // Split non-generic code out of `send` to limit the amount of code generated
        // by monomorphization
        let api_version = api_version_major_to_full(T::API_MAJOR_VERSION);
        let response_body = self.internal_send(url, api_version, request_body).await?;

        Ok(T::api_load_response(&response_body)?)
    }

    async fn internal_send(
        &self,
        url: Url,
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
        let authorization_header_value =
            HeaderValue::from_str(&generate_authorization_header_value(
                self.device.device_id,
                &self.device.signing_key,
                self.time_provider.now(),
            ))
            .expect("always valid");
        content_headers.insert(AUTHORIZATION, authorization_header_value);

        let request_builder = self
            .client
            .post(url)
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

            // HTTP codes used by Parsec

            401 => Err(ConnectionError::MissingAuthenticationInfo),
            403 => Err(ConnectionError::BadAuthenticationInfo),
            404 => Err(ConnectionError::OrganizationNotFound),
            406 => Err(ConnectionError::BadAcceptType),
            // No 410: no invitation here
            415 => Err(ConnectionError::BadContent),
            422 => Err(crate::error::unsupported_api_version_from_headers(
                *api_version, resp.headers(),
            )),
            460 => Err(ConnectionError::ExpiredOrganization),
            461 => Err(ConnectionError::RevokedUser),
            462 => Err(ConnectionError::FrozenUser),
            463 => Err(ConnectionError::UserMustAcceptTos),
            498 => Err(ConnectionError::AuthenticationTokenExpired),

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

        let authorization_header_value =
            HeaderValue::from_str(&generate_authorization_header_value(
                self.device.device_id,
                &self.device.signing_key,
                self.time_provider.now(),
            ))
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
    ) -> Result<SSEStream<T>, ConnectionError>
    where
        T: ProtocolRequest<API_LATEST_MAJOR_VERSION> + Debug + 'static,
    {
        let api_version = api_version_major_to_full(T::API_MAJOR_VERSION);
        let request_builder = self.sse_request_builder::<T>(last_event_id.as_deref());

        #[cfg(feature = "test-with-testbed")]
        let response = self.send_hook.low_level_send(request_builder).await?;
        #[cfg(not(feature = "test-with-testbed"))]
        let response = request_builder.send().await?;

        match response.status().as_u16() {
            200 => (),

            // HTTP codes used by Parsec

            401 => return Err(ConnectionError::MissingAuthenticationInfo),
            403 => return Err(ConnectionError::BadAuthenticationInfo),
            404 => return Err(ConnectionError::OrganizationNotFound),
            406 => return Err(ConnectionError::BadAcceptType),
            // No 410: no invitation here
            415 => return Err(ConnectionError::BadContent),
            // TODO: cannot  access the response headers here...
            422 => return Err(crate::error::unsupported_api_version_from_headers(
                *api_version, response.headers(),
            )),
            460 => return Err(ConnectionError::ExpiredOrganization),
            461 => return Err(ConnectionError::RevokedUser),
            462 => return Err(ConnectionError::FrozenUser),
            463 => return Err(ConnectionError::UserMustAcceptTos),
            498 => return Err(ConnectionError::AuthenticationTokenExpired),

            // Other HTTP codes

            // We typically use HTTP 503 in the tests to simulate server offline,
            // so it should behave just like if we were not able to connect
            // On top of that we should also handle 502 and 504 as they are related
            // to a gateway.
            #[allow(clippy::manual_range_patterns)]
            502 // Bad Gateway
            | 503 // Service Unavailable
            | 504 // Gateway Timeout
            => return Err(ConnectionError::NoResponse(None)),

            // Finally all other HTTP codes are not supposed to occur (well except if
            // an HTTP proxy starts modifying the response, but that's another story...)
            _ => return Err(ConnectionError::InvalidResponseStatus(response.status())),
        };

        response
            .headers()
            // Get `Content-Type` header
            .get(CONTENT_TYPE)
            // Headers are expected to be ASCII (so Rust's `String` compatible)
            .and_then(|content_type| content_type.to_str().ok())
            // `Content-Type` header can have parameters, we only want the MIME type
            .and_then(|content_type| content_type.split(';').next())
            // Check if the MIME type is the one we expect
            .filter(|content_type| *content_type == EVENT_STREAM_CONTENT_TYPE)
            // We end up with `None` if any of the previous steps failed, now give the actual error
            .ok_or(ConnectionError::BadContent)?;

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
/// Authorization token format: `PARSEC-SIGN-ED25519.<author_id_hex>.<timestamp>.<b64_signature>`
/// with:
///     <author_id_hex> = hex(<author's device ID>)
///     <timestamp> = str(<seconds since UNIX epoch>)
///     <b64_signature> = base64(ed25519(`PARSEC-SIGN-ED25519.<author_id_hex>.<timestamp>`))
/// base64() is the URL-safe variant (https://tools.ietf.org/html/rfc4648#section-5).
fn generate_authorization_header_value(
    author: DeviceID,
    author_signing_key: &SigningKey,
    now: DateTime,
) -> String {
    let author_id_hex = author.hex();
    let timestamp = now.as_timestamp_seconds().to_string();
    let header_and_body = format!("{}.{}.{}", PARSEC_AUTH_METHOD, &author_id_hex, &timestamp);
    let signature = author_signing_key.sign_only_signature(header_and_body.as_bytes());
    let b64_signature = BASE64URL.encode(&signature);

    format!("Bearer {}.{}", &header_and_body, &b64_signature)
}

#[cfg(test)]
#[path = "../../tests/unit/authenticated.rs"]
mod tests;

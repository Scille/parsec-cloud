// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use bytes::Bytes;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE},
    Client, RequestBuilder, Url,
};
use std::fmt::Debug;
use std::path::Path;

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

/// Send commands in an invited context.
#[derive(Debug)]
pub struct InvitedCmds {
    /// HTTP Client that contain the basic configuration to communicate with the server.
    client: Client,
    addr: ParsecInvitationAddr,
    url: Url,
    #[cfg(feature = "test-with-testbed")]
    send_hook: SendHookConfig,
}

impl InvitedCmds {
    pub fn new(
        config_dir: &Path,
        addr: ParsecInvitationAddr,
        proxy: ProxyConfig,
    ) -> anyhow::Result<Self> {
        let client = crate::build_client_with_proxy(proxy)?;
        Ok(Self::from_client(client, config_dir, addr))
    }

    pub fn from_client(client: Client, _config_dir: &Path, addr: ParsecInvitationAddr) -> Self {
        let url = addr.to_invited_url();

        #[cfg(feature = "test-with-testbed")]
        let send_hook = get_send_hook(_config_dir);

        Self {
            client,
            addr,
            url,
            #[cfg(feature = "test-with-testbed")]
            send_hook,
        }
    }

    pub fn addr(&self) -> &ParsecInvitationAddr {
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
            self.addr().token(),
            api_version_header_value,
            request_body,
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
            404 => Err(ConnectionError::OrganizationNotFound),
            406 => Err(ConnectionError::BadAcceptType),
            410 => Err(ConnectionError::InvitationAlreadyUsedOrDeleted),
            415 => Err(ConnectionError::BadContent),
            422 => Err(crate::error::unsupported_api_version_from_headers(
                *api_version, resp.headers(),
            )),
            460 => Err(ConnectionError::ExpiredOrganization),
            // No 461/462/463: no user here
            464 => Err(ConnectionError::WebClientNotAllowedByOrganization),
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
    token: AccessToken,
    api_version_header_value: HeaderValue,
    body: Vec<u8>,
) -> RequestBuilder {
    let mut content_headers = HeaderMap::with_capacity(4);
    content_headers.insert(API_VERSION_HEADER_NAME, api_version_header_value);
    content_headers.insert(CONTENT_TYPE, HeaderValue::from_static(PARSEC_CONTENT_TYPE));
    content_headers.insert(
        CONTENT_LENGTH,
        HeaderValue::from_str(&body.len().to_string()).expect("numeric value are valid char"),
    );
    let authorization_value = format!("Bearer {}", token.hex());
    content_headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&authorization_value)
            .expect("Invitation Token is UUID, so made of ASCII characters"),
    );

    request_builder.headers(content_headers).body(body)
}

#[cfg(test)]
#[path = "../tests/unit/invited.rs"]
mod tests;

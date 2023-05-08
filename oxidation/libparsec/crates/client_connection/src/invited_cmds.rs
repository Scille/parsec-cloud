// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use bytes::Bytes;
use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_LENGTH, CONTENT_TYPE},
    Client, RequestBuilder, Url,
};
use std::fmt::Debug;
use std::path::Path;

use libparsec_platform_http_proxy::ProxyConfig;
use libparsec_types::prelude::*;

#[cfg(feature = "test-with-testbed")]
use crate::testbed::{get_send_hook, SendHookConfig};
use crate::{
    error::{ConnectionError, ConnectionResult},
    API_VERSION_HEADER_NAME, PARSEC_CONTENT_TYPE,
};

const INVITATION_TOKEN_HEADER_NAME: &str = "Invitation-Token";

/// Factory that send commands in a invited context.
#[derive(Debug)]
pub struct InvitedCmds {
    /// HTTP Client that contain the basic configuration to communicate with the server.
    client: Client,
    addr: BackendInvitationAddr,
    url: Url,
    #[cfg(feature = "test-with-testbed")]
    send_hook: std::sync::Arc<SendHookConfig>,
}

impl InvitedCmds {
    pub fn new(
        config_dir: &Path,
        addr: BackendInvitationAddr,
        config: ProxyConfig,
    ) -> reqwest::Result<Self> {
        let client = {
            let builder = reqwest::ClientBuilder::default();
            let builder = config.configure_http_client(builder)?;
            builder.build()?
        };
        Ok(Self::from_client(client, config_dir, addr))
    }

    pub fn from_client(client: Client, _config_dir: &Path, addr: BackendInvitationAddr) -> Self {
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

    pub fn addr(&self) -> &BackendInvitationAddr {
        &self.addr
    }

    pub async fn send<T>(&self, request: T) -> ConnectionResult<<T>::Response>
    where
        T: ProtocolRequest + Debug + 'static,
    {
        #[cfg(feature = "test-with-testbed")]
        let request = {
            match self.send_hook.high_level_send(request).await {
                crate::testbed::HighLevelSendResult::Resolved(rep) => return rep,
                crate::testbed::HighLevelSendResult::PassToLowLevel(req) => req,
            }
        };

        let request_body = request.dump()?;

        // Split non-generic code out of `send` to limit the amount of code generated
        // by monomorphization
        let response_body = self.internal_send(request_body).await?;

        Ok(T::load_response(&response_body)?)
    }

    async fn internal_send(&self, request_body: Vec<u8>) -> Result<Bytes, ConnectionError> {
        let request_builder = self.client.post(self.url.clone());

        let req = prepare_request(request_builder, self.addr().token(), request_body);

        #[cfg(feature = "test-with-testbed")]
        let resp = self.send_hook.low_level_send(req).await?;
        #[cfg(not(feature = "test-with-testbed"))]
        let resp = req.send().await?;

        match resp.status().as_u16() {
            200 => {
                let response_body = resp.bytes().await?;
                Ok(response_body)
            }
            404 => Err(ConnectionError::InvitationNotFound),
            410 => Err(ConnectionError::InvitationAlreadyDeleted),
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

/// Prepare a new request, the body will be added to the Request using [RequestBuilder::body]
fn prepare_request(
    request_builder: RequestBuilder,
    token: InvitationToken,
    body: Vec<u8>,
) -> RequestBuilder {
    let mut content_headers = HeaderMap::with_capacity(4);
    content_headers.insert(
        API_VERSION_HEADER_NAME,
        HeaderValue::from_str(&libparsec_protocol::API_VERSION.to_string())
            .expect("api version must contains valid char"),
    );
    content_headers.insert(CONTENT_TYPE, HeaderValue::from_static(PARSEC_CONTENT_TYPE));
    content_headers.insert(
        CONTENT_LENGTH,
        HeaderValue::from_str(&body.len().to_string()).expect("numeric value are valid char"),
    );
    content_headers.insert(
        INVITATION_TOKEN_HEADER_NAME,
        HeaderValue::from_str(&token.hex())
            .expect("Invitation Token is UUID, so made of ASCII characters"),
    );

    request_builder.headers(content_headers).body(body)
}

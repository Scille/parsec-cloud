// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_LENGTH, CONTENT_TYPE},
    Client, RequestBuilder, Url,
};

use libparsec_types::prelude::*;

use crate::{
    error::{CommandError, CommandResult},
    API_VERSION_HEADER_NAME, PARSEC_CONTENT_TYPE,
};

const INVITATION_TOKEN_HEADER_NAME: &str = "Invitation-Token";

/// Factory that send commands in a invited context.
#[derive(Debug, Clone)]
pub struct InvitedCmds {
    /// HTTP Client that contain the basic configuration to communicate with the server.
    client: Client,
    addr: BackendInvitationAddr,
    url: Url,
}

impl InvitedCmds {
    /// Create a new `InvitedCmds`
    pub fn new(client: Client, addr: BackendInvitationAddr) -> Self {
        let url = addr.to_invited_url();

        Self { client, addr, url }
    }

    pub fn addr(&self) -> &BackendInvitationAddr {
        &self.addr
    }
}

/// Prepare a new request, the body will be added to the Request using [RequestBuilder::body]
fn prepare_request(
    request_builder: RequestBuilder,
    body: Vec<u8>,
    token: InvitationToken,
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

impl InvitedCmds {
    pub async fn send<T>(&self, request: T) -> CommandResult<<T>::Response>
    where
        T: ProtocolRequest,
    {
        let request_builder = self.client.post(self.url.clone());

        let data = request.dump()?;

        let req = prepare_request(request_builder, data, self.addr().token()).send();
        let resp = req.await?;
        match resp.status().as_u16() {
            200 => {
                let response_body = resp.bytes().await?;
                Ok(T::load_response(&response_body)?)
            }
            404 => Err(CommandError::InvitationNotFound),
            410 => Err(CommandError::InvitationAlreadyDeleted),
            415 => Err(CommandError::BadContent),
            422 => Err(crate::error::unsupported_api_version_from_headers(
                resp.headers(),
            )),
            460 => Err(CommandError::ExpiredOrganization),
            461 => Err(CommandError::RevokedUser),
            // We typically use HTTP 503 in the tests to simulate server offline,
            // so it should behave just like if we were not able to connect
            503 => Err(CommandError::NoResponse(None)),
            _ => Err(CommandError::InvalidResponseStatus(resp.status(), resp)),
        }
    }
}

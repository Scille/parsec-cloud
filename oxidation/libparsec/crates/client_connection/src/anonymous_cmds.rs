// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use libparsec_protocol::Request;
use libparsec_types::BackendAnonymousAddr;
use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_LENGTH, CONTENT_TYPE},
    Client, RequestBuilder, Url,
};

use crate::{
    error::{CommandError, CommandResult},
    API_VERSION_HEADER_NAME, PARSEC_CONTENT_TYPE,
};

/// Factory that send commands in a anonymous context.
pub struct AnonymousCmds {
    /// HTTP Client that contain the basic configuration to communicate with the server.
    client: Client,
    addr: BackendAnonymousAddr,
    url: Url,
}

impl AnonymousCmds {
    /// Create a new `AnonymousCmds`
    pub fn new(client: Client, addr: BackendAnonymousAddr) -> Result<Self, url::ParseError> {
        let url = addr.to_anonymous_http_url();

        Ok(Self { client, addr, url })
    }

    pub fn addr(&self) -> &BackendAnonymousAddr {
        &self.addr
    }
}

/// Prepare a new request, the body will be added to the Request using [RequestBuilder::body]
fn prepare_request(request_builder: RequestBuilder, body: Vec<u8>) -> RequestBuilder {
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

    request_builder.headers(content_headers).body(body)
}

impl AnonymousCmds {
    pub async fn send<T>(
        &self,
        request: T,
    ) -> CommandResult<<T as libparsec_protocol::Request>::Response>
    where
        T: Request,
    {
        let request_builder = self.client.post(self.url.clone());

        let data = request.dump()?;

        let req = prepare_request(request_builder, data).send();
        let resp = req.await?;
        match resp.status().as_u16() {
            200 => {
                let response_body = resp.bytes().await?;
                Ok(T::load_response(&response_body)?)
            }
            415 => Err(CommandError::BadContent),
            422 => Err(crate::error::unsupported_api_version_from_headers(
                resp.headers(),
            )),
            460 => Err(CommandError::ExpiredOrganization),
            461 => Err(CommandError::RevokedUser),
            _ => Err(CommandError::InvalidResponseStatus(resp.status(), resp)),
        }
    }
}

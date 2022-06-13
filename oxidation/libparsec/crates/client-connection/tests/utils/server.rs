use http_body::Full;
use hyper::{
    body::{self, Bytes},
    header::{AUTHORIZATION, WWW_AUTHENTICATE},
    service::Service,
    Body, HeaderMap, Request, Response, StatusCode,
};
use libparsec_client_connection::authenticated_cmds::PARSEC_AUTH_METHOD;
use parsec_api_crypto::VerifyKey;
use parsec_api_protocol::authenticated_cmds::{self, AnyCmdReq};
use std::{
    collections::HashMap,
    convert::Infallible,
    future::Future,
    ops::Deref,
    pin::Pin,
    task::{Context, Poll},
};

pub type ID = String;

#[derive(Debug)]
pub struct AuthRequest {
    author_b64: String,
    verify_key: VerifyKey,
    timestamp: u128,
    signature_b64: String,
}

pub struct SignatureVerifier {
    registered_public_keys: HashMap<ID, VerifyKey>,
}

impl SignatureVerifier {
    pub fn new(registered_public_keys: HashMap<ID, VerifyKey>) -> Self {
        Self {
            registered_public_keys,
        }
    }
}

impl Service<Request<Body>> for SignatureVerifier {
    type Response = Response<Full<Bytes>>;
    type Error = anyhow::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // The service is directly available.
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        log::debug!("server recv request");
        let res =
            parse_headers(req.headers()).and_then(|(raw_author, timestamp, raw_signature)| {
                let (_author, verify_key) = base64::decode(&raw_author)
                    .map_err(anyhow::Error::from)
                    .and_then(|bytes| {
                        String::from_utf8(bytes)
                            .map_err(anyhow::Error::from)
                            .and_then(|author| {
                                if let Some(vk) = self.registered_public_keys.get(&author) {
                                    Ok((author, vk.clone()))
                                } else {
                                    anyhow::bail!("author {author} not found")
                                }
                            })
                    })?;
                Ok(AuthRequest {
                    author_b64: raw_author,

                    verify_key,
                    timestamp,
                    signature_b64: raw_signature,
                })
            });
        log::debug!("parsed header: {res:?}");

        let fut = async move {
            let auth_req = res?;
            let body = body::to_bytes(req.into_body()).await?;

            let signature = base64::decode(auth_req.signature_b64)?;
            let signed_message = Vec::from_iter(
                (&signature)
                    .iter()
                    .chain(auth_req.author_b64.as_bytes())
                    .chain(&auth_req.timestamp.to_be_bytes())
                    .chain(body.deref())
                    .copied(),
            );
            if let Err(e) = auth_req.verify_key.verify(&signed_message) {
                log::error!("invalid signed request: {e}");
                return Ok(Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .header(
                        WWW_AUTHENTICATE,
                        libparsec_client_connection::authenticated_cmds::PARSEC_AUTH_METHOD,
                    )
                    .body(Full::from(Bytes::from_static(b"invalid signed request")))?);
            }

            let cmd = authenticated_cmds::AnyCmdReq::load(body.as_ref())
                .map_err(|e| anyhow::anyhow!("{e}"))?;
            let ping_req = if let AnyCmdReq::Ping(req) = cmd {
                req
            } else {
                anyhow::bail!("mock server only support ping command")
            };
            let ping_rep = authenticated_cmds::ping::Rep::Ok {
                pong: format!(
                    "hello from the server side!, thanks for your message: \"{}\"",
                    ping_req.ping
                ),
            };

            let rep_body: Full<Bytes> = ping_rep
                .dump()
                .map_err(|e| anyhow::anyhow!("{e}"))
                .map(|bytes| Full::from(Bytes::from(bytes)))?;

            Ok(Response::builder().status(StatusCode::OK).body(rep_body)?)
        };
        Box::pin(fut)
    }
}

fn parse_headers(headers: &HeaderMap) -> anyhow::Result<(String, u128, String)> {
    match headers.get(AUTHORIZATION) {
        Some(value) => {
            if Some(PARSEC_AUTH_METHOD) != value.to_str().ok() {
                anyhow::bail!("invalid authorization header")
            }
        }
        _ => anyhow::bail!("missing authorization header"),
    }
    let raw_user_id = headers
        .get("Author")
        .ok_or_else(|| anyhow::anyhow!("missing author header"))?
        .to_str()
        .map_err(anyhow::Error::from)?
        .to_string();

    let raw_timestamp = headers
        .get("Timestamp")
        .ok_or_else(|| anyhow::anyhow!("missing timestamp header"))?
        .to_str()
        .map_err(anyhow::Error::from)?;
    let timestamp = raw_timestamp.parse().map_err(anyhow::Error::from)?;

    let raw_signature = headers
        .get("Signature")
        .ok_or_else(|| anyhow::anyhow!("missing signature header"))?
        .to_str()
        .map_err(anyhow::Error::from)?
        .to_string();

    Ok((raw_user_id, timestamp, raw_signature))
}

#[derive(Default)]
pub struct MakeSignatureVerifier {
    registered_public_keys: HashMap<String, VerifyKey>,
}

impl MakeSignatureVerifier {
    pub fn register_public_key(&mut self, id: String, key: VerifyKey) {
        self.registered_public_keys.insert(id, key);
    }
}

impl<T> Service<T> for MakeSignatureVerifier {
    type Response = SignatureVerifier;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _req: T) -> Self::Future {
        let registered_public_keys = self.registered_public_keys.clone();
        let fut = async move { Ok(SignatureVerifier::new(registered_public_keys)) };
        Box::pin(fut)
    }
}

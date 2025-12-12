// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

//! This module defines the type of URL used in Parsec.
//!
//! Notes:
//! - Parsec URLs have a specific `parsec3://` schema, but are under the hood regular
//!   `http(s)://` urls.
//! - There is no http vs https dichotomy here: Parsec always uses SSL in production
//!   so a `parsec3://` is expected to be equivalent to a `https://` URL.
//!   However for test purpose only we can disable SSL by passing a `no_ssl=true` query
//!   parameter (e.g. `parsec3://foo` -> `https://foo`, `parsec3://foo?no_ssl=true` -> `http://foo`).
//! - Non-http scheme are often poorly supported (e.g. url in email displayed
//!   in Outlook are not clickable). Hence the need for redirection HTTP URLs
//!   point to the Parsec server and then redirect to the final `parsec3://` URL.
//! - Parsec URLs are not serializable with serde as we shouldn't use them in the data
//!   schemes. This is so that we can change the URL format without breaking the data
//!   serialization format.
//!   The only exception is the Parsec server URL (i.e. `ParsecAddr`) that gets serialized
//!   as a regular http(s) URL (typically used in the device_file_* schemas).

use std::str::FromStr;

use data_encoding::BASE64URL_NOPAD;
pub use url::Url;

use libparsec_crypto::VerifyKey;

use crate::{BootstrapToken, IndexInt, InvitationToken, InvitationType, OrganizationID, VlobID};

pub const PARSEC_SCHEME: &str = "parsec3";
const HTTP_OR_HTTPS_SCHEME: &str = "http(s)";
const PARSEC_SSL_DEFAULT_PORT: u16 = 443;
const PARSEC_NO_SSL_DEFAULT_PORT: u16 = 80;
const PARSEC_PARAM_ACTION: &str = "a";
const PARSEC_PARAM_PAYLOAD: &str = "p";
const PARSEC_ACTION_BOOTSTRAP_ORGANIZATION: &str = "bootstrap_organization";
const PARSEC_ACTION_WORKSPACE_PATH: &str = "path";
const PARSEC_ACTION_CLAIM_USER: &str = "claim_user";
const PARSEC_ACTION_CLAIM_DEVICE: &str = "claim_device";
const PARSEC_ACTION_CLAIM_SHAMIR_RECOVERY: &str = "claim_shamir_recovery";
const PARSEC_ACTION_PKI_ENROLLMENT: &str = "pki_enrollment";
const PARSEC_ACTION_ASYNC_ENROLLMENT: &str = "async_enrollment";

/// Url has a special way to parse http/https schemes. This is because those kind
/// of url have special needs (for instance host cannot be empty).
/// The issue is we want our custom parsec scheme to work in a similar fashion, but
/// we cannot just tell Url "apply to parsec3:// whatever rules you use for http://".
/// So instead we have to rely on a hack and manually replace our scheme by http before
/// handing it to Url :'(
/// see. https://github.com/servo/rust-url/issues/763
struct ParsecUrlAsHTTPScheme(Url);

impl ParsecUrlAsHTTPScheme {
    /// Parse a redirection url (i.e. `http(s)://<domain>/redirect/<path>`)
    fn from_http_redirection(url: &str) -> Result<Self, AddrError> {
        // 1) Validate the http/https url

        let mut parsed = Url::parse(url).map_err(AddrError::InvalidUrl)?;

        match parsed.scheme() {
            "http" | "https" => (),
            _ => {
                return Err(AddrError::InvalidUrlScheme {
                    expected: HTTP_OR_HTTPS_SCHEME,
                });
            }
        };

        // `no_ssl` is defined by http/https scheme and shouldn't be
        // overwritten by the query part of the url
        let mut cleaned_query = url::form_urlencoded::Serializer::new(String::new());
        cleaned_query.extend_pairs(parsed.query_pairs().filter(|(k, _)| k != "no_ssl"));
        parsed.set_query(Some(&cleaned_query.finish()));

        // 2) Remove the `/redirect/` path prefix

        let path = {
            match parsed.path_segments() {
                Some(mut path_segments) => {
                    if path_segments.next() != Some("redirect") {
                        return Err(AddrError::NotARedirection);
                    }

                    path_segments.collect::<Vec<&str>>().join("/")
                }
                None => return Err(AddrError::NotARedirection),
            }
        };
        parsed.set_path(&path);

        // 3) Handle per-specific-address type parsing details
        Ok(ParsecUrlAsHTTPScheme(parsed))
    }

    // Parse a `http(s)://` url just like if it had the `parsec3://` scheme
    fn from_http_url(url: &str) -> Result<Self, AddrError> {
        let mut parsed = Url::parse(url).map_err(AddrError::InvalidUrl)?;

        match parsed.scheme() {
            "http" | "https" => (),
            _ => {
                return Err(AddrError::InvalidUrlScheme {
                    expected: HTTP_OR_HTTPS_SCHEME,
                });
            }
        };

        // `no_ssl` is defined by http/https scheme and shouldn't be
        // overwritten by the query part of the url
        let mut cleaned_query = url::form_urlencoded::Serializer::new(String::new());
        cleaned_query.extend_pairs(parsed.query_pairs().filter(|(k, _)| k != "no_ssl"));
        parsed.set_query(Some(&cleaned_query.finish()));

        Ok(ParsecUrlAsHTTPScheme(parsed))
    }
}

impl FromStr for ParsecUrlAsHTTPScheme {
    type Err = AddrError;

    fn from_str(url: &str) -> Result<Self, Self::Err> {
        // 1) Validate the url with it parsec3:// scheme

        let parsed = Url::parse(url).map_err(AddrError::InvalidUrl)?;

        let mut no_ssl_queries = parsed.query_pairs().filter(|(k, _)| k == "no_ssl");

        let use_ssl = match no_ssl_queries.next() {
            None => true,
            Some((_, value)) => match value.to_lowercase().as_str() {
                // param is no_ssl, but we store use_ssl (so invert the values)
                "false" => true,
                "true" => false,
                _ => {
                    return Err(AddrError::InvalidParamValue {
                        param: "no_ssl",
                        help: "Expected `no_ssl=true` or `no_ssl=false`".to_string(),
                    });
                }
            },
        };

        if no_ssl_queries.next().is_some() {
            return Err(AddrError::DuplicateParam("no_ssl"));
        }

        // 2) Convert the url into a http/https scheme

        // http vs https is important as it changes the default port for parsing !
        let http_scheme = if use_ssl { "https" } else { "http" };
        let url_as_http = match parsed.scheme() {
            // It is vital to do the replace on the parsed URL, this is because
            // the URL parser strips invalid characters (i.e. \n, \t,
            // trailing/leading spaces, etc.).
            //
            // see https://url.spec.whatwg.org/#concept-basic-url-parser
            //
            // For instance, considering the input url starting with `par\nsec3://`:
            // - Parsing is successful.
            // - `parsed.scheme()` returns `parsec3`.
            // - `parsed.as_str()` returns `parsec3://...`.
            // - If we do the replace on `url`, `par\nsec3` will not be matched,
            //   typically leading to a panic later on in `BaseParsecAddr::_from_url`.
            PARSEC_SCHEME => parsed.as_str().replacen(PARSEC_SCHEME, http_scheme, 1),
            _ => {
                return Err(AddrError::InvalidUrlScheme {
                    expected: PARSEC_SCHEME,
                });
            }
        };

        // 3) Continue parsing with the http/https url

        Url::parse(&url_as_http)
            .map_err(AddrError::InvalidUrl)
            .map(Self)
    }
}

macro_rules! impl_common_stuff {
    ($name:ty, _internal_) => {
        impl $name {
            /// Returns a `parsec3://` url
            pub fn to_url(&self) -> Url {
                self._to_url(self.base.to_url())
            }

            /// Returns a http redirection url (i.e. `http(s)://<domain>/redirect/<path>`)
            pub fn to_http_redirection_url(&self) -> Url {
                let mut url = self.base.to_http_url(None);
                url.path_segments_mut()
                    .expect("expected url not to be a cannot-be-a-base")
                    .push("redirect");
                self._to_url(url)
            }

            /// Accept both `parsec3://` and http redirection url
            pub fn from_any(url: &str) -> Result<Self, AddrError> {
                // End with parsec3:// parsing given it error message is the
                // more interesting to return
                Self::from_http_redirection(url).or_else(|_| url.parse())
            }

            /// Create from a http redirection url (i.e. `http(s)://<domain>/redirect/<path>`)
            pub fn from_http_redirection(url: &str) -> Result<Self, AddrError> {
                let parsed = ParsecUrlAsHTTPScheme::from_http_redirection(url)?;
                Self::_from_url(&parsed)
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("url", &self.to_url().as_str())
                    .finish()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", &self.to_url().as_str())
            }
        }

        impl FromStr for $name {
            type Err = AddrError;

            fn from_str(url: &str) -> Result<Self, Self::Err> {
                let parsed = url.parse()?;
                Self::_from_url(&parsed)
            }
        }
    };
    (ParsecAddr) => {
        impl_common_stuff!(ParsecAddr, _internal_);
    };
    ($name:ty) => {
        impl From<$name> for ParsecAddr {
            fn from(value: $name) -> Self {
                ParsecAddr { base: value.base }
            }
        }
        impl From<&$name> for ParsecAddr {
            fn from(value: &$name) -> Self {
                ParsecAddr {
                    base: value.base.to_owned(),
                }
            }
        }
        impl_common_stuff!($name, _internal_);
    };
}

/// The enum contains the list of possible errors when working with url / addr.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum AddrError {
    /// We failed to parse the url
    #[error("Cannot parse URL: {0}")]
    InvalidUrl(url::ParseError),
    #[error("Not a redirection URL")]
    NotARedirection,
    #[error("Invalid URL scheme, expected `{expected}`")]
    InvalidUrlScheme { expected: &'static str },
    #[error("Invalid value for param `{param}` ({help})")]
    InvalidParamValue { param: &'static str, help: String },
    #[error("Multiple values for param `{0}` only one should be provided")]
    DuplicateParam(&'static str),
    #[error("Missing mandatory `{0}` param")]
    MissingParam(&'static str),
    #[error("The URL has a path, which is not expected")]
    ShouldNotHaveAPath,
    #[error("Path does not form a valid organization ID")]
    InvalidOrganizationID,
}

// Define `BaseParsecAddr` in its own sub-module to make sure it can only
// be constructed using `BaseParsecAddr::new()`
mod base {
    use super::*;

    #[derive(Clone, PartialEq, Eq, Hash)]
    pub(super) struct BaseParsecAddr {
        hostname: String,
        port: Option<u16>,
        use_ssl: bool,
    }

    impl BaseParsecAddr {
        pub fn new(hostname: String, port: Option<u16>, use_ssl: bool) -> Self {
            // Discard port if it corresponds to the default value
            let default_port = if use_ssl {
                PARSEC_SSL_DEFAULT_PORT
            } else {
                PARSEC_NO_SSL_DEFAULT_PORT
            };
            let port = port.and_then(|p| if p == default_port { None } else { Some(p) });

            Self {
                hostname,
                port,
                use_ssl,
            }
        }

        pub fn hostname(&self) -> &str {
            &self.hostname
        }

        pub fn port(&self) -> u16 {
            match self.port {
                Some(port) => port,
                None => {
                    if self.use_ssl {
                        PARSEC_SSL_DEFAULT_PORT
                    } else {
                        PARSEC_NO_SSL_DEFAULT_PORT
                    }
                }
            }
        }

        /// `true` when the default port is overloaded.
        pub fn is_default_port(&self) -> bool {
            self.port.is_none()
        }

        pub fn use_ssl(&self) -> bool {
            self.use_ssl
        }

        pub fn from_url(parsed: &ParsecUrlAsHTTPScheme) -> Result<Self, AddrError> {
            let hostname = parsed
                .0
                .host_str()
                .expect("cannot-be-a-base url must contain a hostname");

            // `ParsecUrlAsHTTPScheme`'s creation has consumed the `no_ssl` param to
            // determine if it should be an http or https
            let use_ssl = match parsed.0.scheme() {
                "http" => false,
                "https" => true,
                _ => unreachable!("scheme can only be http or https"),
            };

            Ok(Self::new(hostname.to_owned(), parsed.0.port(), use_ssl))
        }

        /// create a url in parsec format (i.e.: `parsec3://foo.bar[...]`)
        pub fn to_url(&self) -> Url {
            let mut url = Url::parse(&format!("{}://{}", PARSEC_SCHEME, &self.hostname))
                .expect("hostname already validated");
            url.set_port(self.port).expect("port already validated");
            if !self.use_ssl {
                url.query_pairs_mut().append_pair("no_ssl", "true");
            }
            url
        }

        /// Create a url for http request with an optional path.
        pub fn to_http_url(&self, path: Option<&str>) -> Url {
            let scheme = if self.use_ssl { "https" } else { "http" };
            let mut url = Url::parse(&format!("{}://{}", scheme, &self.hostname))
                .expect("hostname already validated");
            url.set_port(self.port).expect("port already validated");
            let path = path.unwrap_or("");
            url.set_path(path);
            url
        }
    }
}
use base::BaseParsecAddr;

macro_rules! expose_base_parsec_addr_fields {
    () => {
        pub fn hostname(&self) -> &str {
            self.base.hostname()
        }

        pub fn port(&self) -> u16 {
            self.base.port()
        }

        /// `true` when the default port is overloaded.
        pub fn is_default_port(&self) -> bool {
            self.base.is_default_port()
        }

        pub fn use_ssl(&self) -> bool {
            self.base.use_ssl()
        }
    };
}

fn extract_param<'a>(
    pairs: &'a url::form_urlencoded::Parse,
    param: &'static str,
) -> Result<std::borrow::Cow<'a, str>, AddrError> {
    let mut action_queries = pairs.filter(|(k, _)| k == param);
    let action = match action_queries.next() {
        None => return Err(AddrError::MissingParam(param)),
        Some((_, value)) => value,
    };
    if action_queries.next().is_some() {
        return Err(AddrError::DuplicateParam(param));
    }
    Ok(action)
}

fn extract_param_and_expect_value<'a>(
    pairs: &'a url::form_urlencoded::Parse,
    param: &'static str,
    expected_value: &str,
) -> Result<std::borrow::Cow<'a, str>, AddrError> {
    let value = extract_param(pairs, param)?;
    if value != expected_value {
        return Err(AddrError::InvalidParamValue {
            param,
            help: format!("Expected `{param}={expected_value}`"),
        });
    }
    Ok(value)
}

fn b64_msgpack_serialize<T: serde::ser::Serialize>(data: &T) -> String {
    rmp_serde::to_vec::<T>(data)
        .map(|raw| BASE64URL_NOPAD.encode(&raw))
        .expect("data are valid")
}

macro_rules! extract_param_and_b64_msgpack_deserialize {
    ($pairs:expr, $param:ident, $output:ty) => {{
        let x = extract_param($pairs, $param)?;
        BASE64URL_NOPAD
            .decode(x.as_bytes())
            .ok()
            .and_then(|x| rmp_serde::from_slice::<$output>(&x).ok())
            .ok_or_else(|| AddrError::InvalidParamValue {
                param: $param,
                help: format!("Invalid `{}` parameter", $param),
            })
    }};
}

fn extract_organization_id(parsed: &ParsecUrlAsHTTPScheme) -> Result<OrganizationID, AddrError> {
    const ERR_MSG: AddrError = AddrError::InvalidOrganizationID;
    // Strip the initial `/`
    let raw = &parsed.0.path().get(1..).ok_or(ERR_MSG)?;
    // Handle percent-encoding
    let decoded = percent_encoding::percent_decode_str(raw)
        .decode_utf8()
        .map_err(|_| ERR_MSG)?;
    decoded.parse().or(Err(ERR_MSG))
}

/*
 * ParsecAddr
 */

/// Represent the URL to reach a server (e.g. ``parsec3://parsec.example.com/``)
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ParsecAddr {
    base: BaseParsecAddr,
}

impl_common_stuff!(ParsecAddr);

impl ParsecAddr {
    pub fn new(hostname: String, port: Option<u16>, use_ssl: bool) -> Self {
        // TODO: handle correctly the errors
        if hostname.is_empty() {
            panic!("Hostname cannot be empty !")
        }
        Self {
            base: BaseParsecAddr::new(hostname, port, use_ssl),
        }
    }

    pub fn from_http_url(url: &str) -> Result<Self, AddrError> {
        let parsed = ParsecUrlAsHTTPScheme::from_http_url(url)?;
        Self::_from_url(&parsed)
    }

    /// Create a url for http request with an optional path
    pub fn to_http_url(&self, path: Option<&str>) -> Url {
        self.base.to_http_url(path)
    }

    fn _from_url(parsed: &ParsecUrlAsHTTPScheme) -> Result<Self, AddrError> {
        let base = BaseParsecAddr::from_url(parsed)?;

        if parsed.0.path() != "" && parsed.0.path() != "/" {
            return Err(AddrError::ShouldNotHaveAPath);
        }

        Ok(Self { base })
    }

    expose_base_parsec_addr_fields!();

    fn _to_url(&self, url: Url) -> Url {
        url
    }

    /// Return an [Url] that points to the server endpoint for authenticated account commands.
    pub fn to_authenticated_account_url(&self) -> Url {
        self.base.to_http_url(Some("/authenticated_account"))
    }

    /// Return an [Url] that points to the server endpoint for anonymous account commands.
    pub fn to_anonymous_server_url(&self) -> Url {
        self.base.to_http_url(Some("/anonymous_server"))
    }
}

// Parsec URLs are not serializable with serde as we shouldn't use them in the data
// schemes. This is so that we can change the URL format without breaking the data
// serialization format.
// The only exception is the Parsec server URL (i.e. `ParsecAddr`) that gets serialized
// as a regular http(s) URL (typically used in the device_file_* schemas).

impl ::serde::Serialize for ParsecAddr {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.to_http_url(None).as_ref())
    }
}

impl<'de> ::serde::Deserialize<'de> for ParsecAddr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ParsecAddr;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a `ParsecAddr` URL")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                ParsecAddr::from_http_url(v)
                    .map_err(|_| ::serde::de::Error::custom("Invalid server URL"))
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

/*
 * ParsecOrganizationAddr
 */

/// Represent the URL to access an organization within a server
///
/// (e.g. ``parsec3://parsec.example.com/MyOrg?p=xCBs8zpdIwovR8EdliVVo2vUOmtumnfsI6Fdndjm0WconA``)  // cspell:disable-line
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ParsecOrganizationAddr {
    base: BaseParsecAddr,
    organization_id: OrganizationID,
    root_verify_key: VerifyKey,
}

impl_common_stuff!(ParsecOrganizationAddr);

impl ParsecOrganizationAddr {
    pub fn new(
        server_addr: impl Into<ParsecAddr>,
        organization_id: OrganizationID,
        root_verify_key: VerifyKey,
    ) -> Self {
        Self {
            base: server_addr.into().base,
            organization_id,
            root_verify_key,
        }
    }

    fn _from_url(parsed: &ParsecUrlAsHTTPScheme) -> Result<Self, AddrError> {
        let base = BaseParsecAddr::from_url(parsed)?;
        let organization_id = extract_organization_id(parsed)?;

        let pairs = parsed.0.query_pairs();
        let root_verify_key =
            extract_param_and_b64_msgpack_deserialize!(&pairs, PARSEC_PARAM_PAYLOAD, VerifyKey)?;

        Ok(Self {
            base,
            organization_id,
            root_verify_key,
        })
    }

    expose_base_parsec_addr_fields!();

    fn _to_url(&self, mut url: Url) -> Url {
        url.path_segments_mut()
            .expect("expected url not to be a cannot-be-a-base")
            .push(self.organization_id.as_ref());

        let payload = b64_msgpack_serialize(&self.root_verify_key);

        url.query_pairs_mut()
            .append_pair(PARSEC_PARAM_PAYLOAD, &payload);
        url
    }

    pub fn organization_id(&self) -> &OrganizationID {
        &self.organization_id
    }

    pub fn root_verify_key(&self) -> &VerifyKey {
        &self.root_verify_key
    }

    /// Return an [Url] that points to the server endpoint for authenticated commands.
    pub fn to_authenticated_http_url(&self) -> Url {
        self.base
            .to_http_url(Some(&format!("/authenticated/{}", self.organization_id)))
    }

    /// Return an [Url] that points to the server endpoint for anonymous commands.
    pub fn to_anonymous_http_url(&self) -> Url {
        self.base
            .to_http_url(Some(&format!("/anonymous/{}", self.organization_id)))
    }
}

/*
 * ParsecActionAddr
 */

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsecActionAddr {
    OrganizationBootstrap(ParsecOrganizationBootstrapAddr),
    WorkspacePath(ParsecWorkspacePathAddr),
    Invitation(ParsecInvitationAddr),
    PkiEnrollment(ParsecPkiEnrollmentAddr),
    AsyncEnrollment(ParsecAsyncEnrollmentAddr),
}

impl ParsecActionAddr {
    pub fn from_any(url: &str) -> Result<Self, AddrError> {
        if let Ok(addr) = ParsecOrganizationBootstrapAddr::from_any(url) {
            Ok(ParsecActionAddr::OrganizationBootstrap(addr))
        } else if let Ok(addr) = ParsecWorkspacePathAddr::from_any(url) {
            Ok(ParsecActionAddr::WorkspacePath(addr))
        } else if let Ok(addr) = ParsecInvitationAddr::from_any(url) {
            Ok(ParsecActionAddr::Invitation(addr))
        } else if let Ok(addr) = ParsecAsyncEnrollmentAddr::from_any(url) {
            Ok(ParsecActionAddr::AsyncEnrollment(addr))
        } else {
            ParsecPkiEnrollmentAddr::from_any(url).map(ParsecActionAddr::PkiEnrollment)
        }
    }

    pub fn from_http_redirection(url: &str) -> Result<Self, AddrError> {
        let parsed = ParsecUrlAsHTTPScheme::from_http_redirection(url)?;

        if let Ok(addr) = ParsecOrganizationBootstrapAddr::_from_url(&parsed) {
            Ok(ParsecActionAddr::OrganizationBootstrap(addr))
        } else if let Ok(addr) = ParsecWorkspacePathAddr::_from_url(&parsed) {
            Ok(ParsecActionAddr::WorkspacePath(addr))
        } else if let Ok(addr) = ParsecInvitationAddr::_from_url(&parsed) {
            Ok(ParsecActionAddr::Invitation(addr))
        } else if let Ok(addr) = ParsecAsyncEnrollmentAddr::_from_url(&parsed) {
            Ok(ParsecActionAddr::AsyncEnrollment(addr))
        } else {
            ParsecPkiEnrollmentAddr::_from_url(&parsed).map(ParsecActionAddr::PkiEnrollment)
        }
    }
}

impl std::str::FromStr for ParsecActionAddr {
    type Err = AddrError;

    #[inline]
    fn from_str(url: &str) -> Result<Self, Self::Err> {
        let parsed = url.parse()?;

        if let Ok(addr) = ParsecOrganizationBootstrapAddr::_from_url(&parsed) {
            Ok(ParsecActionAddr::OrganizationBootstrap(addr))
        } else if let Ok(addr) = ParsecWorkspacePathAddr::_from_url(&parsed) {
            Ok(ParsecActionAddr::WorkspacePath(addr))
        } else if let Ok(addr) = ParsecInvitationAddr::_from_url(&parsed) {
            Ok(ParsecActionAddr::Invitation(addr))
        } else if let Ok(addr) = ParsecAsyncEnrollmentAddr::_from_url(&parsed) {
            Ok(ParsecActionAddr::AsyncEnrollment(addr))
        } else {
            ParsecPkiEnrollmentAddr::_from_url(&parsed).map(ParsecActionAddr::PkiEnrollment)
        }
    }
}

/*
 * ParsecOrganizationBootstrapAddr
 */

/// Represent the URL to bootstrap an organization within a server
///
/// (e.g. ``parsec3://parsec.example.com/my_org?a=bootstrap_organization&p=xBCgAAAAAAAAAAAAAAAAAAAB``)  // cspell:disable-line
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ParsecOrganizationBootstrapAddr {
    base: BaseParsecAddr,
    organization_id: OrganizationID,
    token: Option<BootstrapToken>,
}

impl_common_stuff!(ParsecOrganizationBootstrapAddr);

impl ParsecOrganizationBootstrapAddr {
    pub fn new(
        server_addr: impl Into<ParsecAddr>,
        organization_id: OrganizationID,
        token: Option<BootstrapToken>,
    ) -> Self {
        Self {
            base: server_addr.into().base,
            organization_id,
            token,
        }
    }

    fn _from_url(parsed: &ParsecUrlAsHTTPScheme) -> Result<Self, AddrError> {
        let base = BaseParsecAddr::from_url(parsed)?;
        let organization_id = extract_organization_id(parsed)?;
        let pairs = parsed.0.query_pairs();

        extract_param_and_expect_value(
            &pairs,
            PARSEC_PARAM_ACTION,
            PARSEC_ACTION_BOOTSTRAP_ORGANIZATION,
        )?;
        let token = extract_param_and_b64_msgpack_deserialize!(
            &pairs,
            PARSEC_PARAM_PAYLOAD,
            Option<BootstrapToken>
        )?;

        Ok(Self {
            base,
            organization_id,
            token,
        })
    }

    expose_base_parsec_addr_fields!();

    fn _to_url(&self, mut url: Url) -> Url {
        url.path_segments_mut()
            .expect("expected url not to be a cannot-be-a-base")
            .push(self.organization_id.as_ref());

        let payload = b64_msgpack_serialize(&self.token);

        url.query_pairs_mut()
            .append_pair(PARSEC_PARAM_ACTION, PARSEC_ACTION_BOOTSTRAP_ORGANIZATION)
            .append_pair(PARSEC_PARAM_PAYLOAD, &payload);
        url
    }

    pub fn organization_id(&self) -> &OrganizationID {
        &self.organization_id
    }

    pub fn token(&self) -> Option<&BootstrapToken> {
        self.token.as_ref()
    }

    pub fn generate_organization_addr(&self, root_verify_key: VerifyKey) -> ParsecOrganizationAddr {
        ParsecOrganizationAddr::new(
            self.clone(),
            self.organization_id().clone(),
            root_verify_key,
        )
    }
}

/*
 * ParsecWorkspacePathAddr
 */

/// Represent the URL to share a file link
///
/// (e.g. ``parsec3://parsec.example.com/my_org?a=path&p=k9gCLU3tEnQGRgiDO39X8BFW4gHcADTM4WfM1MzhzNnMvTPMq8y-BnrM-8yiDcyvdlvMv2wjzIskB8zZWi4yFwRtzMxAzIDM0iPMnX8czKY7Pm3M5szoODd-NiI8U3A``)  // cspell:disable-line
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ParsecWorkspacePathAddr {
    base: BaseParsecAddr,
    organization_id: OrganizationID,
    workspace_id: VlobID,
    key_index: IndexInt,
    encrypted_path: Vec<u8>,
}

impl_common_stuff!(ParsecWorkspacePathAddr);

impl ParsecWorkspacePathAddr {
    pub fn new(
        server_addr: impl Into<ParsecAddr>,
        organization_id: OrganizationID,
        workspace_id: VlobID,
        key_index: IndexInt,
        encrypted_path: Vec<u8>,
    ) -> Self {
        Self {
            base: server_addr.into().base,
            organization_id,
            workspace_id,
            key_index,
            encrypted_path,
        }
    }

    fn _from_url(parsed: &ParsecUrlAsHTTPScheme) -> Result<Self, AddrError> {
        let base = BaseParsecAddr::from_url(parsed)?;
        let organization_id = extract_organization_id(parsed)?;
        let pairs = parsed.0.query_pairs();

        extract_param_and_expect_value(&pairs, PARSEC_PARAM_ACTION, PARSEC_ACTION_WORKSPACE_PATH)?;
        let (workspace_id, key_index, encrypted_path) = extract_param_and_b64_msgpack_deserialize!(
            &pairs,
            PARSEC_PARAM_PAYLOAD,
            (VlobID, IndexInt, Vec<u8>)
        )?;

        Ok(Self {
            base,
            organization_id,
            workspace_id,
            key_index,
            encrypted_path,
        })
    }

    expose_base_parsec_addr_fields!();

    fn _to_url(&self, mut url: Url) -> Url {
        url.path_segments_mut()
            .expect("expected url not to be a cannot-be-a-base")
            .push(self.organization_id.as_ref());

        let payload =
            b64_msgpack_serialize(&(self.workspace_id, self.key_index, &self.encrypted_path));

        url.query_pairs_mut()
            .append_pair(PARSEC_PARAM_ACTION, PARSEC_ACTION_WORKSPACE_PATH)
            .append_pair(PARSEC_PARAM_PAYLOAD, &payload);

        url
    }

    pub fn key_index(&self) -> IndexInt {
        self.key_index
    }

    pub fn organization_id(&self) -> &OrganizationID {
        &self.organization_id
    }

    pub fn workspace_id(&self) -> VlobID {
        self.workspace_id
    }

    pub fn encrypted_path(&self) -> &Vec<u8> {
        &self.encrypted_path
    }
}

/*
 * ParsecInvitationAddr
 */

/// Represent the URL to invite a user or a device
///
/// (e.g. ``parsec3://parsec.example.com/my_org?a=claim_user&p=xBCgAAAAAAAAAAAAAAAAAAAB``)  // cspell:disable-line
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ParsecInvitationAddr {
    base: BaseParsecAddr,
    organization_id: OrganizationID,
    invitation_type: InvitationType,
    token: InvitationToken,
}

impl_common_stuff!(ParsecInvitationAddr);

impl ParsecInvitationAddr {
    pub fn new(
        server_addr: impl Into<ParsecAddr>,
        organization_id: OrganizationID,
        invitation_type: InvitationType,
        token: InvitationToken,
    ) -> Self {
        Self {
            base: server_addr.into().base,
            organization_id,
            invitation_type,
            token,
        }
    }

    fn _from_url(parsed: &ParsecUrlAsHTTPScheme) -> Result<Self, AddrError> {
        let base = BaseParsecAddr::from_url(parsed)?;
        let organization_id = extract_organization_id(parsed)?;
        let pairs = parsed.0.query_pairs();

        let invitation_type = match extract_param(&pairs, PARSEC_PARAM_ACTION)? {
            x if x == PARSEC_ACTION_CLAIM_USER => InvitationType::User,
            x if x == PARSEC_ACTION_CLAIM_DEVICE => InvitationType::Device,
            x if x == PARSEC_ACTION_CLAIM_SHAMIR_RECOVERY => InvitationType::ShamirRecovery,
            _ => {
                return Err(AddrError::InvalidParamValue {
                    param: PARSEC_PARAM_ACTION,
                    help: format!(
                        "Expected `{PARSEC_PARAM_ACTION}={PARSEC_ACTION_CLAIM_USER}`, `{PARSEC_PARAM_ACTION}={PARSEC_ACTION_CLAIM_DEVICE}` or `{PARSEC_PARAM_ACTION}={PARSEC_ACTION_CLAIM_SHAMIR_RECOVERY}`"
                    ),
                })
            }
        };
        let token = extract_param_and_b64_msgpack_deserialize!(
            &pairs,
            PARSEC_PARAM_PAYLOAD,
            InvitationToken
        )?;

        Ok(Self {
            base,
            organization_id,
            invitation_type,
            token,
        })
    }

    expose_base_parsec_addr_fields!();

    fn _to_url(&self, mut url: Url) -> Url {
        url.path_segments_mut()
            .expect("expected url not to be a cannot-be-a-base")
            .push(self.organization_id.as_ref());

        let payload = b64_msgpack_serialize(&self.token);

        url.query_pairs_mut()
            .append_pair(
                PARSEC_PARAM_ACTION,
                match self.invitation_type() {
                    InvitationType::User => PARSEC_ACTION_CLAIM_USER,
                    InvitationType::Device => PARSEC_ACTION_CLAIM_DEVICE,
                    InvitationType::ShamirRecovery => PARSEC_ACTION_CLAIM_SHAMIR_RECOVERY,
                },
            )
            .append_pair(PARSEC_PARAM_PAYLOAD, &payload);
        url
    }

    pub fn organization_id(&self) -> &OrganizationID {
        &self.organization_id
    }

    pub fn invitation_type(&self) -> InvitationType {
        self.invitation_type
    }

    pub fn token(&self) -> InvitationToken {
        self.token
    }

    pub fn generate_organization_addr(&self, root_verify_key: VerifyKey) -> ParsecOrganizationAddr {
        ParsecOrganizationAddr::new(
            self.clone(),
            self.organization_id().clone(),
            root_verify_key,
        )
    }

    /// Return an [Url] that points to the server endpoint for invited commands.
    pub fn to_invited_url(&self) -> Url {
        self.base
            .to_http_url(Some(&format!("/invited/{}", &self.organization_id)))
    }
}

/*
 * ParsecPkiEnrollmentAddr
 */

/// Represent the URL to invite a user using PKI
///
/// (e.g. ``parsec3://parsec.example.com/my_org?a=pki_enrollment``)
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ParsecPkiEnrollmentAddr {
    base: BaseParsecAddr,
    organization_id: OrganizationID,
}

impl_common_stuff!(ParsecPkiEnrollmentAddr);

impl ParsecPkiEnrollmentAddr {
    pub fn new(server_addr: impl Into<ParsecAddr>, organization_id: OrganizationID) -> Self {
        Self {
            base: server_addr.into().base,
            organization_id,
        }
    }

    fn _from_url(parsed: &ParsecUrlAsHTTPScheme) -> Result<Self, AddrError> {
        let base = BaseParsecAddr::from_url(parsed)?;
        let organization_id = extract_organization_id(parsed)?;
        let pairs = parsed.0.query_pairs();
        extract_param_and_expect_value(&pairs, PARSEC_PARAM_ACTION, PARSEC_ACTION_PKI_ENROLLMENT)?;

        Ok(Self {
            base,
            organization_id,
        })
    }

    expose_base_parsec_addr_fields!();

    fn _to_url(&self, mut url: Url) -> Url {
        url.path_segments_mut()
            .expect("expected url not to be a cannot-be-a-base")
            .push(self.organization_id.as_ref());
        url.query_pairs_mut()
            .append_pair(PARSEC_PARAM_ACTION, PARSEC_ACTION_PKI_ENROLLMENT);
        url
    }

    pub fn organization_id(&self) -> &OrganizationID {
        &self.organization_id
    }

    pub fn generate_organization_addr(&self, root_verify_key: VerifyKey) -> ParsecOrganizationAddr {
        ParsecOrganizationAddr::new(
            self.clone(),
            self.organization_id().clone(),
            root_verify_key,
        )
    }
}

/*
 * ParsecAsyncEnrollmentAddr
 */

/// Represent the URL used to request an asynchronous enrollment (i.e. enrollment
/// started by the submitter using a 3rd party identity system such as a PKI).
///
/// (e.g. ``parsec3://parsec.example.com/my_org?a=async_enrollment``)
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ParsecAsyncEnrollmentAddr {
    base: BaseParsecAddr,
    organization_id: OrganizationID,
}

impl_common_stuff!(ParsecAsyncEnrollmentAddr);

impl ParsecAsyncEnrollmentAddr {
    pub fn new(server_addr: impl Into<ParsecAddr>, organization_id: OrganizationID) -> Self {
        Self {
            base: server_addr.into().base,
            organization_id,
        }
    }

    fn _from_url(parsed: &ParsecUrlAsHTTPScheme) -> Result<Self, AddrError> {
        let base = BaseParsecAddr::from_url(parsed)?;
        let organization_id = extract_organization_id(parsed)?;
        let pairs = parsed.0.query_pairs();
        extract_param_and_expect_value(
            &pairs,
            PARSEC_PARAM_ACTION,
            PARSEC_ACTION_ASYNC_ENROLLMENT,
        )?;

        Ok(Self {
            base,
            organization_id,
        })
    }

    expose_base_parsec_addr_fields!();

    fn _to_url(&self, mut url: Url) -> Url {
        url.path_segments_mut()
            .expect("expected url not to be a cannot-be-a-base")
            .push(self.organization_id.as_ref());
        url.query_pairs_mut()
            .append_pair(PARSEC_PARAM_ACTION, PARSEC_ACTION_ASYNC_ENROLLMENT);
        url
    }

    pub fn organization_id(&self) -> &OrganizationID {
        &self.organization_id
    }

    pub fn generate_organization_addr(&self, root_verify_key: VerifyKey) -> ParsecOrganizationAddr {
        ParsecOrganizationAddr::new(
            self.clone(),
            self.organization_id().clone(),
            root_verify_key,
        )
    }
}

/*
 * ParsecActionAddr
 */

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsecAnonymousAddr {
    OrganizationBootstrap(ParsecOrganizationBootstrapAddr),
    PkiEnrollment(ParsecPkiEnrollmentAddr),
    AsyncEnrollment(ParsecAsyncEnrollmentAddr),
}

impl ParsecAnonymousAddr {
    /// Return an [Url] that points to the server endpoint for anonymous commands.
    pub fn to_anonymous_http_url(&self) -> Url {
        let (ParsecAnonymousAddr::OrganizationBootstrap(ParsecOrganizationBootstrapAddr {
            base,
            organization_id,
            ..
        })
        | ParsecAnonymousAddr::PkiEnrollment(ParsecPkiEnrollmentAddr {
            base,
            organization_id,
        })
        | ParsecAnonymousAddr::AsyncEnrollment(ParsecAsyncEnrollmentAddr {
            base,
            organization_id,
        })) = self;
        base.to_http_url(Some(&format!("/anonymous/{organization_id}")))
    }

    pub fn organization_id(&self) -> &OrganizationID {
        match self {
            ParsecAnonymousAddr::OrganizationBootstrap(addr) => addr.organization_id(),
            ParsecAnonymousAddr::PkiEnrollment(addr) => addr.organization_id(),
            ParsecAnonymousAddr::AsyncEnrollment(addr) => addr.organization_id(),
        }
    }
}

impl From<ParsecOrganizationBootstrapAddr> for ParsecAnonymousAddr {
    fn from(addr: ParsecOrganizationBootstrapAddr) -> Self {
        Self::OrganizationBootstrap(addr)
    }
}

impl From<ParsecPkiEnrollmentAddr> for ParsecAnonymousAddr {
    fn from(addr: ParsecPkiEnrollmentAddr) -> Self {
        Self::PkiEnrollment(addr)
    }
}

impl From<ParsecAsyncEnrollmentAddr> for ParsecAnonymousAddr {
    fn from(addr: ParsecAsyncEnrollmentAddr) -> Self {
        Self::AsyncEnrollment(addr)
    }
}

impl From<ParsecAnonymousAddr> for ParsecAddr {
    fn from(addr: ParsecAnonymousAddr) -> Self {
        let base = match addr {
            ParsecAnonymousAddr::OrganizationBootstrap(addr) => addr.base,
            ParsecAnonymousAddr::PkiEnrollment(addr) => addr.base,
            ParsecAnonymousAddr::AsyncEnrollment(addr) => addr.base,
        };
        ParsecAddr { base }
    }
}

#[cfg(test)]
#[path = "../tests/unit/addr.rs"]
mod tests;

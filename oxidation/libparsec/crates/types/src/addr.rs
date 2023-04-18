// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::collections::HashMap;
use std::convert::TryFrom;
use std::str::FromStr;

use data_encoding::BASE32;
use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};
use url::Url;

use libparsec_crypto::VerifyKey;

use crate::{EntryID, InvitationToken, InvitationType, OrganizationID};

const PARSEC_SCHEME: &str = "parsec";
const PARSEC_SSL_DEFAULT_PORT: u16 = 443;
const PARSEC_NO_SSL_DEFAULT_PORT: u16 = 80;

/// Url has a special way to parse http/https schemes. This is because those kind
/// of url have special needs (for instance host cannot be empty).
/// The issue is we want our custom parsec scheme to work in a similar fashion, but
/// we cannot just tell Url "apply to parsec:// whatever rules you use for http://".
/// So instead we have to rely on a hack and manually replace our scheme by http before
/// handing it to Url :'(
/// see. https://github.com/servo/rust-url/issues/763
struct ParsecUrlAsHTTPScheme(pub Url);

macro_rules! impl_common_stuff {
    (BackendAddr) => {
        impl_common_stuff!(BackendAddr, _internal_);
    };
    ($name:ty) => {
        impl From<$name> for BackendAddr {
            fn from(value: $name) -> Self {
                BackendAddr { base: value.base }
            }
        }
        impl_common_stuff!($name, _internal_);
    };
    ($name:ty, _internal_) => {
        impl $name {
            pub fn to_url(&self) -> Url {
                self._to_url(self.base.to_url())
            }

            pub fn to_http_redirection_url(&self) -> Url {
                let mut url = self.base.to_http_url(None);
                url.path_segments_mut()
                    .expect("expected url not to be a cannot-be-a-base")
                    .push("redirect");
                self._to_url(url)
            }

            pub fn from_any(url: &str) -> Result<Self, AddrError> {
                // End with parsec:// parsing given it error message is the
                // more interesting to return
                Self::from_http_redirection(url).or_else(|_| url.parse())
            }

            /// Create a new Addr from a raw http url that must have `/redirect` as prefix of its path.
            pub fn from_http_redirection(url: &str) -> Result<Self, AddrError> {
                // 1) Validate the http/https url

                let mut parsed =
                    Url::parse(url).map_err(|e| AddrError::InvalidUrl(url.to_string(), e))?;

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
                                return Err(AddrError::NotARedirection(parsed));
                            }
                            path_segments.collect::<Vec<&str>>().join("/")
                        }
                        None => return Err(AddrError::NotARedirection(parsed)),
                    }
                };
                parsed.set_path(&path);

                // 3) Handle per-specific-address type parsing details

                Self::_from_url(ParsecUrlAsHTTPScheme(parsed))
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
                // 1) Validate the url with it parsec:// scheme

                let parsed =
                    Url::parse(&url).map_err(|e| AddrError::InvalidUrl(url.to_string(), e))?;
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
                                value: value.to_string(),
                                help: "Expected `no_ssl=true` or `no_ssl=false`".to_string(),
                            });
                        }
                    },
                };
                if no_ssl_queries.next().is_some() {
                    return Err(AddrError::DuplicateParam("no_ssl".to_string()));
                }

                // 2) Convert the url into a http/https scheme

                if parsed.scheme() != PARSEC_SCHEME {
                    return Err(AddrError::InvalidUrlScheme {
                        got: parsed.scheme().to_string(),
                        expected: PARSEC_SCHEME,
                    });
                }
                // http vs https is important as it changes the default port for parsing !
                let http_scheme = if use_ssl { "https" } else { "http" };
                let url_as_http = url.replacen(PARSEC_SCHEME, http_scheme, 1);

                // 3) Continue parsing with the http/https url

                let parsed = Url::parse(&url_as_http)
                    .map_err(|e| AddrError::InvalidUrl(url.to_string(), e))?;
                Self::_from_url(ParsecUrlAsHTTPScheme(parsed))
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(self.to_url().as_str())
            }
        }

        paste::paste! {
            impl<'de> Deserialize<'de> for $name {
                fn deserialize<D>(deserializer: D) -> Result<$name, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    deserializer.deserialize_str([<$name Visitor>])
                }
            }

            struct [<$name Visitor>];

            impl<'de> Visitor<'de> for [<$name Visitor>] {
                type Value = $name;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str(concat!("a ", stringify!($name), " URL"))
                }

                fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    value.parse().map_err(E::custom)
                }
            }

        }
    };
}

/// The enum constains the list of possible errors when working with url / addr.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum AddrError {
    /// We failed to parse the url
    #[error("Cannot parse raw url `{0}`: {1}")]
    InvalidUrl(String, url::ParseError),
    #[error("Not a redirection URL (url: `{0}`)")]
    NotARedirection(url::Url),
    #[error("Invalid url scheme, got `{got}` but expected `{expected}`")]
    InvalidUrlScheme { got: String, expected: &'static str },
    #[error("Invalid value `{value}` for {param} ({help})")]
    InvalidParamValue {
        param: &'static str,
        value: String,
        help: String,
    },
    #[error("Multiple values for param `{0}` only one should be provided")]
    DuplicateParam(String),
    #[error("Missing mandatory `{0}` param")]
    MissingParam(&'static str),
    #[error("The provided url (`{0}`) should not have a path")]
    ShouldNotHaveAPath(url::Url),
    #[error("Invalid base32 data: {0}")]
    InvalidBase32Data(data_encoding::DecodeError),
    #[error("Path does not form a valid organization id")]
    InvalidOrganizationID,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct BaseBackendAddr {
    hostname: String,
    port: Option<u16>,
    use_ssl: bool,
}

impl BaseBackendAddr {
    fn from_url(parsed: &ParsecUrlAsHTTPScheme) -> Result<Self, AddrError> {
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

        let default_port = if use_ssl {
            PARSEC_SSL_DEFAULT_PORT
        } else {
            PARSEC_NO_SSL_DEFAULT_PORT
        };
        let port = parsed
            .0
            .port()
            .and_then(|p| if p == default_port { None } else { Some(p) });

        Ok(Self {
            hostname: hostname.to_owned(),
            port,
            use_ssl,
        })
    }

    /// create a url in parsec format (i.e.: `parsec://foo.bar[...]`)
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

macro_rules! expose_BaseBackendAddr_fields {
    () => {
        pub fn hostname(&self) -> &str {
            &self.base.hostname
        }

        pub fn port(&self) -> u16 {
            match self.base.port {
                Some(port) => port,
                None => {
                    if self.base.use_ssl {
                        PARSEC_SSL_DEFAULT_PORT
                    } else {
                        PARSEC_NO_SSL_DEFAULT_PORT
                    }
                }
            }
        }

        /// `true` when the default port is overloaded.
        pub fn is_default_port(&self) -> bool {
            self.base.port.is_none()
        }

        pub fn use_ssl(&self) -> bool {
            self.base.use_ssl
        }

        /// Create a url for http request with an optional path
        pub fn to_http_url(&self, path: Option<&str>) -> Url {
            self.base.to_http_url(path)
        }
    };
}

fn extract_action<'a>(
    pairs: &'a url::form_urlencoded::Parse,
) -> Result<std::borrow::Cow<'a, str>, AddrError> {
    let mut action_queries = pairs.filter(|(k, _)| k == "action");
    let action = match action_queries.next() {
        None => return Err(AddrError::MissingParam("action")),
        Some((_, value)) => value,
    };
    if action_queries.next().is_some() {
        return Err(AddrError::DuplicateParam("action".to_string()));
    }
    Ok(action)
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
 * BackendAddr
 */

/// Represent the URL to reach a backend (e.g. ``parsec://parsec.example.com/``)
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BackendAddr {
    base: BaseBackendAddr,
}

impl_common_stuff!(BackendAddr);

impl BackendAddr {
    pub fn new(hostname: String, port: Option<u16>, use_ssl: bool) -> Self {
        if hostname.is_empty() {
            panic!("Hostname cannot be empty !")
        }
        Self {
            base: BaseBackendAddr {
                hostname,
                port,
                use_ssl,
            },
        }
    }

    pub fn to_http_url_with_path(&self, path: Option<&str>) -> Url {
        self.base.to_http_url(path)
    }

    fn _from_url(parsed: ParsecUrlAsHTTPScheme) -> Result<Self, AddrError> {
        let base = BaseBackendAddr::from_url(&parsed)?;

        if parsed.0.path() != "" && parsed.0.path() != "/" {
            return Err(AddrError::ShouldNotHaveAPath(parsed.0));
        }

        Ok(Self { base })
    }

    expose_BaseBackendAddr_fields!();

    fn _to_url(&self, url: Url) -> Url {
        url
    }
}

/*
 * BackendOrganizationAddr
 */

/// Represent the URL to access an organization within a backend
/// (e.g. ``parsec://parsec.example.com/MyOrg?rvk=7NFDS4VQLP3XPCMTSEN34ZOXKGGIMTY2W2JI2SPIHB2P3M6K4YWAssss``)
#[derive(Clone, PartialEq, Eq)]
pub struct BackendOrganizationAddr {
    base: BaseBackendAddr,
    organization_id: OrganizationID,
    root_verify_key: VerifyKey,
}

impl_common_stuff!(BackendOrganizationAddr);

impl BackendOrganizationAddr {
    pub fn new(
        backend_addr: impl Into<BackendAddr>,
        organization_id: OrganizationID,
        root_verify_key: VerifyKey,
    ) -> Self {
        Self {
            base: backend_addr.into().base,
            organization_id,
            root_verify_key,
        }
    }

    fn _from_url(parsed: ParsecUrlAsHTTPScheme) -> Result<Self, AddrError> {
        let base = BaseBackendAddr::from_url(&parsed)?;
        let organization_id = extract_organization_id(&parsed)?;

        let pairs = parsed.0.query_pairs();
        let mut rvk_queries = pairs.filter(|(k, _)| k == "rvk");
        let root_verify_key = match rvk_queries.next() {
            None => return Err(AddrError::MissingParam("rvk")),
            Some((_, value)) => {
                import_root_verify_key(&value).map_err(|e| AddrError::InvalidParamValue {
                    param: "rvk",
                    value: value.to_string(),
                    help: e.to_string(),
                })?
            }
        };
        if rvk_queries.next().is_some() {
            return Err(AddrError::DuplicateParam("rvk".to_string()));
        }

        Ok(Self {
            base,
            organization_id,
            root_verify_key,
        })
    }

    expose_BaseBackendAddr_fields!();

    fn _to_url(&self, mut url: Url) -> Url {
        url.path_segments_mut()
            .expect("expected url not to be a cannot-be-a-base")
            .push(self.organization_id.as_ref());
        url.query_pairs_mut()
            .append_pair("rvk", &export_root_verify_key(&self.root_verify_key));
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
 * BackendActionAddr
 */

pub enum BackendActionAddr {
    OrganizationBootstrap(BackendOrganizationBootstrapAddr),
    OrganizationFileLink(BackendOrganizationFileLinkAddr),
    Invitation(BackendInvitationAddr),
    PkiEnrollment(BackendPkiEnrollmentAddr),
}

impl BackendActionAddr {
    pub fn from_any(url: &str) -> Result<Self, AddrError> {
        if let Ok(addr) = BackendOrganizationBootstrapAddr::from_any(url) {
            Ok(BackendActionAddr::OrganizationBootstrap(addr))
        } else if let Ok(addr) = BackendOrganizationFileLinkAddr::from_any(url) {
            Ok(BackendActionAddr::OrganizationFileLink(addr))
        } else if let Ok(addr) = BackendInvitationAddr::from_any(url) {
            Ok(BackendActionAddr::Invitation(addr))
        } else {
            BackendPkiEnrollmentAddr::from_any(url).map(BackendActionAddr::PkiEnrollment)
        }
    }

    pub fn from_http_redirection(url: &str) -> Result<Self, AddrError> {
        if let Ok(addr) = BackendOrganizationBootstrapAddr::from_http_redirection(url) {
            Ok(BackendActionAddr::OrganizationBootstrap(addr))
        } else if let Ok(addr) = BackendOrganizationFileLinkAddr::from_http_redirection(url) {
            Ok(BackendActionAddr::OrganizationFileLink(addr))
        } else if let Ok(addr) = BackendInvitationAddr::from_http_redirection(url) {
            Ok(BackendActionAddr::Invitation(addr))
        } else {
            BackendPkiEnrollmentAddr::from_http_redirection(url)
                .map(BackendActionAddr::PkiEnrollment)
        }
    }
}

impl std::str::FromStr for BackendActionAddr {
    type Err = AddrError;

    #[inline]
    fn from_str(url: &str) -> Result<Self, Self::Err> {
        if let Ok(addr) = url.parse::<BackendOrganizationBootstrapAddr>() {
            Ok(BackendActionAddr::OrganizationBootstrap(addr))
        } else if let Ok(addr) = url.parse::<BackendOrganizationFileLinkAddr>() {
            Ok(BackendActionAddr::OrganizationFileLink(addr))
        } else if let Ok(addr) = url.parse::<BackendInvitationAddr>() {
            Ok(BackendActionAddr::Invitation(addr))
        } else {
            url.parse::<BackendPkiEnrollmentAddr>()
                .map(BackendActionAddr::PkiEnrollment)
        }
    }
}

/*
 * BackendOrganizationBootstrapAddr
 */

// Represent the URL to bootstrap an organization within a backend
// (e.g. ``parsec://parsec.example.com/my_org?action=bootstrap_organization&token=1234ABCD``)
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BackendOrganizationBootstrapAddr {
    base: BaseBackendAddr,
    organization_id: OrganizationID,
    token: Option<String>,
}
impl_common_stuff!(BackendOrganizationBootstrapAddr);
impl BackendOrganizationBootstrapAddr {
    pub fn new(
        backend_addr: impl Into<BackendAddr>,
        organization_id: OrganizationID,
        token: Option<String>,
    ) -> Self {
        Self {
            base: backend_addr.into().base,
            organization_id,
            token,
        }
    }

    fn _from_url(parsed: ParsecUrlAsHTTPScheme) -> Result<Self, AddrError> {
        let base = BaseBackendAddr::from_url(&parsed)?;
        let organization_id = extract_organization_id(&parsed)?;
        let pairs = parsed.0.query_pairs();
        let action = extract_action(&pairs)?;
        if action != "bootstrap_organization" {
            return Err(AddrError::InvalidParamValue {
                param: "action",
                value: action.to_string(),
                help: "Expected `action=bootstrap_organization`".to_string(),
            });
        }

        let mut token_queries = pairs.filter(|(k, _)| k == "token");
        let token = token_queries.next().map(|(_, v)| (*v).to_owned());
        // Note invalid percent-encoding is not considered a failure here:
        // the replacement character EF BF BD is used instead. This should be
        // ok for our use case (but it differs from Python implementation).
        if token_queries.next().is_some() {
            return Err(AddrError::DuplicateParam("token".to_string()));
        }
        // Consider empty token as no token
        // It's important to do this cooking eagerly (instead of e.g. doing it in
        // the token getter) to avoid broken comparison between empty and None tokens
        let token = match token {
            Some(content) if content.is_empty() => None,
            token => token,
        };

        Ok(Self {
            base,
            organization_id,
            token,
        })
    }

    expose_BaseBackendAddr_fields!();

    fn _to_url(&self, mut url: Url) -> Url {
        url.path_segments_mut()
            .expect("expected url not to be a cannot-be-a-base")
            .push(self.organization_id.as_ref());
        url.query_pairs_mut()
            .append_pair("action", "bootstrap_organization");
        if let Some(ref tk) = self.token {
            if !tk.is_empty() {
                url.query_pairs_mut().append_pair("token", tk);
            }
        }
        url
    }

    pub fn organization_id(&self) -> &OrganizationID {
        &self.organization_id
    }

    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    pub fn to_http_url_with_path(&self, path: Option<&str>) -> Url {
        self.base.to_http_url(path)
    }

    pub fn generate_organization_addr(
        &self,
        root_verify_key: VerifyKey,
    ) -> BackendOrganizationAddr {
        BackendOrganizationAddr::new(
            BackendAddr::new(
                self.hostname().into(),
                if !self.is_default_port() {
                    Some(self.port())
                } else {
                    None
                },
                self.use_ssl(),
            ),
            self.organization_id().clone(),
            root_verify_key,
        )
    }
}

/*
 * BackendOrganizationFileLinkAddr
 */

/// Represent the URL to share a file link
/// (e.g. ``parsec://parsec.example.com/my_org?action=file_link&workspace_id=3a50b191122b480ebb113b10216ef343&path=7NFDS4VQLP3XPCMTSEN34ZOXKGGIMTY2W2JI2SPIHB2P3M6K4YWAssss``)
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BackendOrganizationFileLinkAddr {
    base: BaseBackendAddr,
    organization_id: OrganizationID,
    workspace_id: EntryID,
    encrypted_path: Vec<u8>,
    encrypted_timestamp: Option<Vec<u8>>,
}

impl_common_stuff!(BackendOrganizationFileLinkAddr);

impl BackendOrganizationFileLinkAddr {
    pub fn new(
        backend_addr: impl Into<BackendAddr>,
        organization_id: OrganizationID,
        workspace_id: EntryID,
        encrypted_path: Vec<u8>,
        encrypted_timestamp: Option<Vec<u8>>,
    ) -> Self {
        Self {
            base: backend_addr.into().base,
            organization_id,
            workspace_id,
            encrypted_path,
            encrypted_timestamp,
        }
    }

    fn _from_url(parsed: ParsecUrlAsHTTPScheme) -> Result<Self, AddrError> {
        let base = BaseBackendAddr::from_url(&parsed)?;
        let organization_id = extract_organization_id(&parsed)?;
        let pairs = parsed.0.query_pairs();
        let action = extract_action(&pairs)?;
        if action != "file_link" {
            return Err(AddrError::InvalidParamValue {
                param: "action",
                value: action.to_string(),
                help: "Expected `action=file_link`".to_string(),
            });
        }

        let mut query_str_map = HashMap::new();
        for (key, value) in pairs {
            match query_str_map.entry(key) {
                std::collections::hash_map::Entry::Occupied(occupied) => {
                    return Err(AddrError::DuplicateParam(occupied.key().to_string()))
                }
                std::collections::hash_map::Entry::Vacant(vacant) => {
                    vacant.insert(value);
                }
            }
        }

        let encrypted_timestamp = if let Some(ts) = query_str_map.get("timestamp") {
            Some(binary_urlsafe_decode(ts)?)
        } else {
            None
        };

        Ok(Self {
            base,
            organization_id,
            workspace_id: query_str_map
                .get("workspace_id")
                .ok_or(AddrError::MissingParam("workspace_id"))
                .and_then(|v| {
                    EntryID::from_hex(v).map_err(|e| AddrError::InvalidParamValue {
                        param: "workspace_id",
                        value: v.to_string(),
                        help: e.to_string(),
                    })
                })?,
            encrypted_path: query_str_map
                .get("path")
                .ok_or(AddrError::MissingParam("path"))
                .and_then(|v| {
                    binary_urlsafe_decode(v).map_err(|e| AddrError::InvalidParamValue {
                        param: "path",
                        value: v.to_string(),
                        help: e.to_string(),
                    })
                })?,
            encrypted_timestamp,
        })
    }

    expose_BaseBackendAddr_fields!();

    fn _to_url(&self, mut url: Url) -> Url {
        url.path_segments_mut()
            .expect("expected url not to be a cannot-be-a-base")
            .push(self.organization_id.as_ref());
        url.query_pairs_mut()
            .append_pair("action", "file_link")
            .append_pair("workspace_id", &self.workspace_id.hex())
            .append_pair("path", &binary_urlsafe_encode(&self.encrypted_path));
        if let Some(ts) = &self.encrypted_timestamp {
            url.query_pairs_mut()
                .append_pair("timestamp", &binary_urlsafe_encode(ts));
        }

        url
    }

    pub fn encrypted_timestamp(&self) -> &Option<Vec<u8>> {
        &self.encrypted_timestamp
    }

    pub fn organization_id(&self) -> &OrganizationID {
        &self.organization_id
    }

    pub fn workspace_id(&self) -> EntryID {
        self.workspace_id
    }

    pub fn encrypted_path(&self) -> &Vec<u8> {
        &self.encrypted_path
    }
}

/*
 * BackendInvitationAddr
 */

/// Represent the URL to invite a user or a device
/// (e.g. ``parsec://parsec.example.com/my_org?action=claim_user&token=3a50b191122b480ebb113b10216ef343``)
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BackendInvitationAddr {
    base: BaseBackendAddr,
    organization_id: OrganizationID,
    invitation_type: InvitationType,
    token: InvitationToken,
}

impl_common_stuff!(BackendInvitationAddr);

impl BackendInvitationAddr {
    pub fn new(
        backend_addr: impl Into<BackendAddr>,
        organization_id: OrganizationID,
        invitation_type: InvitationType,
        token: InvitationToken,
    ) -> Self {
        Self {
            base: backend_addr.into().base,
            organization_id,
            invitation_type,
            token,
        }
    }

    fn _from_url(parsed: ParsecUrlAsHTTPScheme) -> Result<Self, AddrError> {
        let base = BaseBackendAddr::from_url(&parsed)?;
        let organization_id = extract_organization_id(&parsed)?;
        let pairs = parsed.0.query_pairs();
        let invitation_type = match extract_action(&pairs)? {
            x if x == "claim_user" => InvitationType::User,
            x if x == "claim_device" => InvitationType::Device,
            value => {
                return Err(AddrError::InvalidParamValue {
                    param: "action",
                    value: value.to_string(),
                    help: "Expected `action=claim_user` or `action=claim_device`".to_string(),
                })
            }
        };

        let mut token_queries = pairs.filter(|(k, _)| k == "token");
        let token = match token_queries.next() {
            None => return Err(AddrError::MissingParam("token")),
            Some((_, value)) => {
                InvitationToken::from_hex(&value).map_err(|e| AddrError::InvalidParamValue {
                    param: "token",
                    value: value.to_string(),
                    help: e.to_string(),
                })?
            }
        };
        if token_queries.next().is_some() {
            return Err(AddrError::DuplicateParam("token".to_string()));
        }

        Ok(Self {
            base,
            organization_id,
            invitation_type,
            token,
        })
    }

    expose_BaseBackendAddr_fields!();

    fn _to_url(&self, mut url: Url) -> Url {
        url.path_segments_mut()
            .expect("expected url not to be a cannot-be-a-base")
            .push(self.organization_id.as_ref());
        url.query_pairs_mut()
            .append_pair(
                "action",
                match self.invitation_type() {
                    InvitationType::User => "claim_user",
                    InvitationType::Device => "claim_device",
                },
            )
            .append_pair("token", &self.token.hex());
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

    pub fn generate_organization_addr(
        &self,
        root_verify_key: VerifyKey,
    ) -> BackendOrganizationAddr {
        BackendOrganizationAddr::new(
            BackendAddr::new(
                self.hostname().into(),
                if !self.is_default_port() {
                    Some(self.port())
                } else {
                    None
                },
                self.use_ssl(),
            ),
            self.organization_id().clone(),
            root_verify_key,
        )
    }

    /// Return an [Url] that points to the server endpoint for invited commands.
    pub fn to_invited_url(&self) -> Url {
        self.to_http_url(Some(&format!("/invited/{}", self.organization_id())))
    }
}

/*
 * BackendPkiEnrollmentAddr
 */

/// Represent the URL to invite a user using PKI
/// (e.g. ``parsec://parsec.example.com/my_org?action=pki_enrollment``)
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BackendPkiEnrollmentAddr {
    base: BaseBackendAddr,
    organization_id: OrganizationID,
}

impl_common_stuff!(BackendPkiEnrollmentAddr);

impl BackendPkiEnrollmentAddr {
    pub fn new(backend_addr: impl Into<BackendAddr>, organization_id: OrganizationID) -> Self {
        Self {
            base: backend_addr.into().base,
            organization_id,
        }
    }

    fn _from_url(parsed: ParsecUrlAsHTTPScheme) -> Result<Self, AddrError> {
        let base = BaseBackendAddr::from_url(&parsed)?;
        let organization_id = extract_organization_id(&parsed)?;
        let pairs = parsed.0.query_pairs();
        let action = extract_action(&pairs)?;
        if action != "pki_enrollment" {
            return Err(AddrError::InvalidParamValue {
                param: "action",
                value: action.to_string(),
                help: "Expected `action=pki_enrollment".to_string(),
            });
        }

        Ok(Self {
            base,
            organization_id,
        })
    }

    expose_BaseBackendAddr_fields!();

    fn _to_url(&self, mut url: Url) -> Url {
        url.path_segments_mut()
            .expect("expected url not to be a cannot-be-a-base")
            .push(self.organization_id.as_ref());
        url.query_pairs_mut()
            .append_pair("action", "pki_enrollment");
        url
    }

    pub fn to_http_url_with_path(&self, path: Option<&str>) -> Url {
        self.base.to_http_url(path)
    }

    pub fn organization_id(&self) -> &OrganizationID {
        &self.organization_id
    }

    pub fn generate_organization_addr(
        &self,
        root_verify_key: VerifyKey,
    ) -> BackendOrganizationAddr {
        BackendOrganizationAddr::new(
            BackendAddr::new(
                self.hostname().into(),
                if !self.is_default_port() {
                    Some(self.port())
                } else {
                    None
                },
                self.use_ssl(),
            ),
            self.organization_id().clone(),
            root_verify_key,
        )
    }
}

pub enum BackendAnonymousAddr {
    BackendOrganizationBootstrapAddr(BackendOrganizationBootstrapAddr),
    BackendPkiEnrollmentAddr(BackendPkiEnrollmentAddr),
}

impl BackendAnonymousAddr {
    /// Return an [Url] that points to the server endpoint for anonymous commands.
    pub fn to_anonymous_http_url(&self) -> Url {
        let (BackendAnonymousAddr::BackendOrganizationBootstrapAddr(
            BackendOrganizationBootstrapAddr {
                base,
                organization_id,
                ..
            },
        )
        | BackendAnonymousAddr::BackendPkiEnrollmentAddr(BackendPkiEnrollmentAddr {
            base,
            organization_id,
        })) = self;
        base.to_http_url(Some(&format!("/anonymous/{}", organization_id)))
    }
}

/*
 * Helpers
 */

// Binary encoder/decoder for url use.
// Notes:
// - We replace padding char `=` by a simple `s` (which is not part of
//   the base32 table so no risk of collision) to avoid copy/paste errors
//   and silly escaping issues when carrying the key around.
// - We could be using base64url (see RFC 4648) which would be more efficient,
//   but backward compatibility prevent us from doing it :'(

pub fn binary_urlsafe_encode(data: &[u8]) -> String {
    BASE32.encode(data).replace('=', "s")
}

pub fn binary_urlsafe_decode(data: &str) -> Result<Vec<u8>, AddrError> {
    BASE32
        .decode(data.replace('s', "=").as_bytes())
        .map_err(AddrError::InvalidBase32Data)
}

pub fn export_root_verify_key(key: &VerifyKey) -> String {
    binary_urlsafe_encode(key.as_ref())
}

pub fn import_root_verify_key(encoded: &str) -> Result<VerifyKey, &'static str> {
    let err_msg = "Invalid root verify key";
    let encoded_b32 = encoded.replace('s', "=");
    // TODO: would be better to directly decode into key
    let buff = BASE32.decode(encoded_b32.as_bytes()).or(Err(err_msg))?;
    VerifyKey::try_from(buff.as_slice()).map_err(|_| err_msg)
}

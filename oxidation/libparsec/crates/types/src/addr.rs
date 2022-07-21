// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use data_encoding::BASE32;
use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};
use std::convert::TryFrom;
use std::str::FromStr;
use url::Url;

use libparsec_crypto::VerifyKey;

use super::{EntryID, InvitationToken, InvitationType, OrganizationID};

const PARSEC_SCHEME: &str = "parsec";
const PARSEC_SSL_DEFAULT_PORT: u16 = 443;
const PARSEC_NO_SSL_DEFAULT_PORT: u16 = 80;

macro_rules! impl_common_stuff {
    ($name:ty) => {
        impl $name {
            pub fn to_url(&self) -> Url {
                self._to_url(self.base.to_url())
            }

            pub fn to_http_redirection_url(&self) -> Url {
                let mut url = self.base.to_http_url(None);
                url.path_segments_mut()
                    .unwrap_or_else(|()| unreachable!())
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
                // For whatever reason, Url considers illegal changing scheme
                // from http/https to a custom one, so we cannot just use
                // `Url::set_scheme` and instead have to do this hack :(
                let url_with_forced_custom_scheme = format!("x{}", url);
                let url = &url_with_forced_custom_scheme;

                // Note `Url::parse` takes care of percent-encoding for query params
                let mut parsed = Url::parse(url).map_err(|_| "Invalid URL")?;

                // `no_ssl` is defined by http/https scheme and shouldn't be
                // overwritten by the query part of the url
                let mut cleaned_query = url::form_urlencoded::Serializer::new(String::new());
                cleaned_query.extend_pairs(parsed.query_pairs().filter(|(k, _)| k != "no_ssl"));
                match parsed.scheme() {
                    "xhttp" => {
                        cleaned_query.append_pair("no_ssl", "true");
                    }
                    "xhttps" => (),
                    _ => {
                        return Err("Not a redirection URL");
                    }
                };
                parsed.set_query(Some(&cleaned_query.finish()));
                parsed
                    .set_scheme(PARSEC_SCHEME)
                    .unwrap_or_else(|()| unreachable!());

                // Remove the `/redirect/` path prefix
                let mut path_segments = parsed
                    .path_segments()
                    .ok_or_else(|| "Not a redirection URL")?;
                if path_segments.next() != Some("redirect") {
                    return Err("Redirection URL must have a `/redirect/...` path");
                }
                let path = &path_segments.collect::<Vec<&str>>().join("/");
                parsed.set_path(path);

                let pairs = parsed.query_pairs();
                Self::_from_url(&parsed, &pairs)
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
            type Err = &'static str;

            fn from_str(url: &str) -> Result<Self, Self::Err> {
                // Note `Url::parse` takes care of percent-encoding for query params
                let parsed = Url::parse(url).map_err(|_| "Invalid URL")?;
                let pairs = parsed.query_pairs();

                Self::_from_url(&parsed, &pairs)
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

type AddrError = &'static str;

#[derive(Clone, PartialEq, Eq)]
struct BaseBackendAddr {
    hostname: String,
    port: Option<u16>,
    use_ssl: bool,
}

impl BaseBackendAddr {
    fn from_url(parsed: &Url, pairs: &url::form_urlencoded::Parse) -> Result<Self, AddrError> {
        if parsed.scheme() != PARSEC_SCHEME {
            lazy_static! {
                static ref SCHEME_ERROR_MSG: String =
                    format!("Must start with `{}://`", PARSEC_SCHEME);
            }
            return Err(&SCHEME_ERROR_MSG);
        }

        // Use the same error message than for `Url::parse` because
        // `Url::parse` considers port with no hostname (e.g. `http://:8080`)
        // invalid and we want to stay consistent to simplify testing
        let hostname = parsed.host_str().ok_or("Invalid URL")?;

        let mut no_ssl_queries = pairs.filter(|(k, _)| k == "no_ssl");
        let use_ssl = match no_ssl_queries.next() {
            None => true,
            Some((_, value)) => match value.to_lowercase().as_str() {
                // param is no_ssl, but we store use_ssl (so invert the values)
                "false" => true,
                "true" => false,
                _ => {
                    return Err("Invalid `no_ssl` param value (must be true or false)");
                }
            },
        };
        if no_ssl_queries.next().is_some() {
            return Err("Multiple values for param `no_ssl`");
        }

        let default_port = if use_ssl {
            PARSEC_SSL_DEFAULT_PORT
        } else {
            PARSEC_NO_SSL_DEFAULT_PORT
        };
        let port = parsed
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
            .unwrap_or_else(|_| unreachable!());
        url.set_port(self.port).unwrap_or_else(|_| unreachable!());
        if !self.use_ssl {
            url.query_pairs_mut().append_pair("no_ssl", "true");
        }
        url
    }

    /// Create a url for http request with an optional path.
    pub fn to_http_url(&self, path: Option<&str>) -> Url {
        let scheme = if self.use_ssl { "https" } else { "http" };

        let mut url = Url::parse(&format!("{}://{}", scheme, &self.hostname))
            .unwrap_or_else(|_| unreachable!());
        url.set_port(self.port).unwrap_or_else(|_| unreachable!());

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
        None => return Err("Missing mandatory `action` param"),
        Some((_, value)) => value,
    };
    if action_queries.next().is_some() {
        return Err("Multiple values for param `action`");
    }
    Ok(action)
}

fn extract_organization_id(parsed: &Url) -> Result<OrganizationID, AddrError> {
    const ERR_MSG: &str = "Path doesn't form a valid organization id";
    // Strip the initial `/`
    let raw = &parsed.path().get(1..).ok_or(ERR_MSG)?;
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
#[derive(Clone, PartialEq, Eq)]
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

    fn _from_url(parsed: &Url, pairs: &url::form_urlencoded::Parse) -> Result<Self, AddrError> {
        let base = BaseBackendAddr::from_url(parsed, pairs)?;

        if parsed.path() != "" && parsed.path() != "/" {
            return Err("Cannot have path");
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
        backend_addr: BackendAddr,
        organization_id: OrganizationID,
        root_verify_key: VerifyKey,
    ) -> Self {
        Self {
            base: backend_addr.base,
            organization_id,
            root_verify_key,
        }
    }

    fn _from_url(parsed: &Url, pairs: &url::form_urlencoded::Parse) -> Result<Self, AddrError> {
        let base = BaseBackendAddr::from_url(parsed, pairs)?;
        let organization_id = extract_organization_id(parsed)?;

        let mut rvk_queries = pairs.filter(|(k, _)| k == "rvk");
        let root_verify_key = match rvk_queries.next() {
            None => return Err("Missing mandatory `rvk` param"),
            Some((_, value)) => {
                import_root_verify_key(&value).or(Err("Invalid `rvk` param value"))?
            }
        };
        if rvk_queries.next().is_some() {
            return Err("Multiple values for param `rvk`");
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
            .unwrap_or_else(|()| unreachable!())
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

    /// Return an [Url] that point to the server endpoint for authenticated commands.
    pub fn to_authenticated_http_url(&self) -> Url {
        self.base
            .to_http_url(Some(&format!("/authenticated/{}", self.organization_id)))
    }

    /// Return an [Url] that point to the server endpoint for anonymous commands.
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
            return Ok(BackendActionAddr::OrganizationBootstrap(addr));
        }
        if let Ok(addr) = BackendOrganizationFileLinkAddr::from_any(url) {
            return Ok(BackendActionAddr::OrganizationFileLink(addr));
        }
        if let Ok(addr) = BackendInvitationAddr::from_any(url) {
            return Ok(BackendActionAddr::Invitation(addr));
        }
        if let Ok(addr) = BackendPkiEnrollmentAddr::from_any(url) {
            return Ok(BackendActionAddr::PkiEnrollment(addr));
        }
        Err("Invalid URL format")
    }

    pub fn from_http_redirection(url: &str) -> Result<Self, AddrError> {
        if let Ok(addr) = BackendOrganizationBootstrapAddr::from_http_redirection(url) {
            return Ok(BackendActionAddr::OrganizationBootstrap(addr));
        }
        if let Ok(addr) = BackendOrganizationFileLinkAddr::from_http_redirection(url) {
            return Ok(BackendActionAddr::OrganizationFileLink(addr));
        }
        if let Ok(addr) = BackendInvitationAddr::from_http_redirection(url) {
            return Ok(BackendActionAddr::Invitation(addr));
        }
        if let Ok(addr) = BackendPkiEnrollmentAddr::from_http_redirection(url) {
            return Ok(BackendActionAddr::PkiEnrollment(addr));
        }
        Err("Invalid URL format")
    }
}

impl std::str::FromStr for BackendActionAddr {
    type Err = &'static str;

    #[inline]
    fn from_str(url: &str) -> Result<Self, Self::Err> {
        if let Ok(addr) = url.parse::<BackendOrganizationBootstrapAddr>() {
            return Ok(BackendActionAddr::OrganizationBootstrap(addr));
        }
        if let Ok(addr) = url.parse::<BackendOrganizationFileLinkAddr>() {
            return Ok(BackendActionAddr::OrganizationFileLink(addr));
        }
        if let Ok(addr) = url.parse::<BackendInvitationAddr>() {
            return Ok(BackendActionAddr::Invitation(addr));
        }
        if let Ok(addr) = url.parse::<BackendPkiEnrollmentAddr>() {
            return Ok(BackendActionAddr::PkiEnrollment(addr));
        }
        Err("Invalid URL format")
    }
}

/*
 * BackendOrganizationBootstrapAddr
 */

// Represent the URL to bootstrap an organization within a backend
// (e.g. ``parsec://parsec.example.com/my_org?action=bootstrap_organization&token=1234ABCD``)
#[derive(Clone, PartialEq, Eq)]
pub struct BackendOrganizationBootstrapAddr {
    base: BaseBackendAddr,
    organization_id: OrganizationID,
    token: Option<String>,
}
impl_common_stuff!(BackendOrganizationBootstrapAddr);
impl BackendOrganizationBootstrapAddr {
    pub fn new(
        backend_addr: BackendAddr,
        organization_id: OrganizationID,
        token: Option<String>,
    ) -> Self {
        Self {
            base: backend_addr.base,
            organization_id,
            token,
        }
    }

    fn _from_url(parsed: &Url, pairs: &url::form_urlencoded::Parse) -> Result<Self, AddrError> {
        let base = BaseBackendAddr::from_url(parsed, pairs)?;
        let organization_id = extract_organization_id(parsed)?;
        let action = extract_action(pairs)?;
        if action != "bootstrap_organization" {
            return Err("Expected `action=bootstrap_organization` param value");
        }

        let mut token_queries = pairs.filter(|(k, _)| k == "token");
        let token = token_queries.next().map(|(_, v)| (*v).to_owned());
        // Note invalid percent-encoding is not considered a failure here:
        // the replacement character EF BF BD is used instead. This should be
        // ok for our use case (but it differs from Python implementation).
        if token_queries.next().is_some() {
            return Err("Multiple values for param `token`");
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
            .unwrap_or_else(|()| unreachable!())
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
}

/*
 * BackendOrganizationFileLinkAddr
 */

/// Represent the URL to share a file link
/// (e.g. ``parsec://parsec.example.com/my_org?action=file_link&workspace_id=3a50b191122b480ebb113b10216ef343&path=7NFDS4VQLP3XPCMTSEN34ZOXKGGIMTY2W2JI2SPIHB2P3M6K4YWAssss``)
#[derive(Clone, PartialEq, Eq)]
pub struct BackendOrganizationFileLinkAddr {
    base: BaseBackendAddr,
    organization_id: OrganizationID,
    workspace_id: EntryID,
    encrypted_path: Vec<u8>,
}

impl_common_stuff!(BackendOrganizationFileLinkAddr);

impl BackendOrganizationFileLinkAddr {
    pub fn new(
        backend_addr: BackendAddr,
        organization_id: OrganizationID,
        workspace_id: EntryID,
        encrypted_path: Vec<u8>,
    ) -> Self {
        Self {
            base: backend_addr.base,
            organization_id,
            workspace_id,
            encrypted_path,
        }
    }

    fn _from_url(parsed: &Url, pairs: &url::form_urlencoded::Parse) -> Result<Self, AddrError> {
        let base = BaseBackendAddr::from_url(parsed, pairs)?;
        let organization_id = extract_organization_id(parsed)?;
        let action = extract_action(pairs)?;
        if action != "file_link" {
            return Err("Expected `action=file_link` param value");
        }

        let mut workspace_id_queries = pairs.filter(|(k, _)| k == "workspace_id");
        let workspace_id = match workspace_id_queries.next() {
            None => return Err("Missing mandatory `workspace_id` param"),
            Some((_, value)) => value
                .parse::<EntryID>()
                .or(Err("Invalid `workspace_id` param value"))?,
        };
        if workspace_id_queries.next().is_some() {
            return Err("Multiple values for param `workspace_id`");
        }

        let mut path_queries = pairs.filter(|(k, _)| k == "path");
        let encrypted_path = match path_queries.next() {
            None => return Err("Missing mandatory `path` param"),
            Some((_, value)) => {
                binary_urlsafe_decode(&value).or(Err("Invalid `path` param value"))?
            }
        };
        if path_queries.next().is_some() {
            return Err("Multiple values for param `path`");
        }

        Ok(Self {
            base,
            organization_id,
            workspace_id,
            encrypted_path,
        })
    }

    expose_BaseBackendAddr_fields!();

    fn _to_url(&self, mut url: Url) -> Url {
        url.path_segments_mut()
            .unwrap_or_else(|()| unreachable!())
            .push(self.organization_id.as_ref());
        url.query_pairs_mut()
            .append_pair("action", "file_link")
            .append_pair("workspace_id", &self.workspace_id.to_string())
            .append_pair("path", &binary_urlsafe_encode(&self.encrypted_path));
        url
    }

    pub fn organization_id(&self) -> &OrganizationID {
        &self.organization_id
    }

    pub fn workspace_id(&self) -> &EntryID {
        &self.workspace_id
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
#[derive(Clone, PartialEq, Eq)]
pub struct BackendInvitationAddr {
    base: BaseBackendAddr,
    organization_id: OrganizationID,
    invitation_type: InvitationType,
    token: InvitationToken,
}

impl_common_stuff!(BackendInvitationAddr);

impl BackendInvitationAddr {
    pub fn new(
        backend_addr: BackendAddr,
        organization_id: OrganizationID,
        invitation_type: InvitationType,
        token: InvitationToken,
    ) -> Self {
        Self {
            base: backend_addr.base,
            organization_id,
            invitation_type,
            token,
        }
    }

    fn _from_url(parsed: &Url, pairs: &url::form_urlencoded::Parse) -> Result<Self, AddrError> {
        let base = BaseBackendAddr::from_url(parsed, pairs)?;
        let organization_id = extract_organization_id(parsed)?;
        let invitation_type = match extract_action(pairs)? {
            x if x == "claim_user" => InvitationType::User,
            x if x == "claim_device" => InvitationType::Device,
            _ => return Err("Expected `action=claim_user` or `action=claim_device` param value"),
        };

        let mut token_queries = pairs.filter(|(k, _)| k == "token");
        let token = match token_queries.next() {
            None => return Err("Missing mandatory `token` param"),
            Some((_, value)) => value
                .parse::<InvitationToken>()
                .or(Err("Invalid `token` param value"))?,
        };
        if token_queries.next().is_some() {
            return Err("Multiple values for param `token`");
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
            .unwrap_or_else(|()| unreachable!())
            .push(self.organization_id.as_ref());
        url.query_pairs_mut()
            .append_pair(
                "action",
                match self.invitation_type() {
                    InvitationType::User => "claim_user",
                    InvitationType::Device => "claim_device",
                },
            )
            .append_pair("token", &self.token.to_string());
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
}

/*
 * BackendPkiEnrollmentAddr
 */

/// Represent the URL to invite a user using PKI
/// (e.g. ``parsec://parsec.example.com/my_org?action=pki_enrollment``)
#[derive(Clone, PartialEq, Eq)]
pub struct BackendPkiEnrollmentAddr {
    base: BaseBackendAddr,
    organization_id: OrganizationID,
}

impl_common_stuff!(BackendPkiEnrollmentAddr);

impl BackendPkiEnrollmentAddr {
    pub fn new(backend_addr: BackendAddr, organization_id: OrganizationID) -> Self {
        Self {
            base: backend_addr.base,
            organization_id,
        }
    }

    fn _from_url(parsed: &Url, pairs: &url::form_urlencoded::Parse) -> Result<Self, AddrError> {
        let base = BaseBackendAddr::from_url(parsed, pairs)?;
        let organization_id = extract_organization_id(parsed)?;
        let action = extract_action(pairs)?;
        if action != "pki_enrollment" {
            return Err("Expected `action=pki_enrollment` param value");
        }

        Ok(Self {
            base,
            organization_id,
        })
    }

    expose_BaseBackendAddr_fields!();

    fn _to_url(&self, mut url: Url) -> Url {
        url.path_segments_mut()
            .unwrap_or_else(|()| unreachable!())
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

pub fn binary_urlsafe_decode(data: &str) -> Result<Vec<u8>, &'static str> {
    let err = Err("Invalid base32 data");
    BASE32.decode(data.replace('s', "=").as_bytes()).or(err)
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

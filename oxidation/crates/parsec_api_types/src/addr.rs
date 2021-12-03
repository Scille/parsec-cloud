// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use data_encoding::BASE32;
use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};
use std::convert::TryFrom;
use std::str::FromStr;
use url::Url;

use super::{EntryID, InvitationToken, InvitationType, OrganizationID};
use parsec_api_crypto::VerifyKey;

const PARSEC_SCHEME: &str = "parsec";
const PARSEC_SSL_DEFAULT_PORT: u16 = 443;
const PARSEC_NO_SSL_DEFAULT_PORT: u16 = 80;

macro_rules! impl_common_stuff {
    ($name:ident) => {
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("url", &self)
                    .finish()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", &self)
            }
        }

        impl FromStr for $name {
            type Err = &'static str;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                $name::_from_str(s)
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

#[derive(Clone, PartialEq, Eq)]
struct BaseBackendAddr {
    hostname: String,
    port: Option<u16>,
    use_ssl: bool,
}

type AddrError = &'static str;

impl BaseBackendAddr {
    fn from_url(parsed: &Url, pairs: &url::form_urlencoded::Parse) -> Result<Self, AddrError> {
        if parsed.scheme() != PARSEC_SCHEME {
            lazy_static! {
                static ref SCHEME_ERROR_MSG: String = format!("Must start with {}", PARSEC_SCHEME);
            }
            return Err(&SCHEME_ERROR_MSG);
        }

        let hostname = parsed.host_str().ok_or("Missing mandatory hostname")?;

        let mut no_ssl_queries = pairs.filter(|(k, _)| k == "no_ssl");
        let use_ssl = match no_ssl_queries.next() {
            None => true,
            Some((_, value)) => match value.to_lowercase().as_str() {
                // param is no_ssl, but we store use_ssl (so invert the values)
                "false" => true,
                "true" => false,
                _ => {
                    return Err("Invalid `no_ssl` query value (must be true or false)");
                }
            },
        };
        if no_ssl_queries.next().is_some() {
            return Err("Multiple values for query `no_ssl`");
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

    pub fn to_url(&self) -> Url {
        let mut url = Url::parse(&format!("{}://{}", PARSEC_SCHEME, &self.hostname)).unwrap();
        url.set_port(self.port).unwrap();
        if !self.use_ssl {
            url.query_pairs_mut().append_pair("no_ssl", "true");
        }
        url
    }
}

macro_rules! expose_BaseBackendAddr_fields {
    () => {
        pub fn hostname(&self) -> &str {
            &self.base.hostname
        }

        pub fn port(&self) -> Option<u16> {
            self.base.port
        }

        pub fn use_ssl(&self) -> bool {
            self.base.use_ssl
        }
    };
}

fn extract_action<'a>(
    pairs: &'a url::form_urlencoded::Parse,
) -> Result<std::borrow::Cow<'a, str>, AddrError> {
    let mut action_queries = pairs.filter(|(k, _)| k == "action");
    let action = match action_queries.next() {
        None => return Err("Missing mandatory `action` query"),
        Some((_, value)) => value,
    };
    if action_queries.next().is_some() {
        return Err("Multiple values for query `action`");
    }
    Ok(action)
}

fn extract_organization_id(parsed: &Url) -> Result<OrganizationID, AddrError> {
    let raw = &parsed.path()[1..]; // Strip the initial `/`
    raw.parse()
        .or(Err("Path doesn't form a valid organization id"))
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

    fn _from_str(url: &str) -> Result<Self, AddrError> {
        // Note `Url::parse` takes care of percent-encoding for query params
        let parsed = Url::parse(url).map_err(|_| "Invalid url")?;
        let pairs = parsed.query_pairs();

        let base = BaseBackendAddr::from_url(&parsed, &pairs)?;

        if parsed.path() != "" && parsed.path() != "/" {
            return Err("Cannot have path");
        }

        Ok(Self { base })
    }

    expose_BaseBackendAddr_fields!();

    pub fn to_url(&self) -> Url {
        self.base.to_url()
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

    fn _from_str(url: &str) -> Result<Self, AddrError> {
        // Note `Url::parse` takes care of percent-encoding for query params
        let parsed = Url::parse(url).map_err(|_| "Invalid url")?;
        let pairs = parsed.query_pairs();

        let base = BaseBackendAddr::from_url(&parsed, &pairs)?;
        let organization_id = extract_organization_id(&parsed)?;

        let mut rvk_queries = pairs.filter(|(k, _)| k == "rvk");
        let root_verify_key = match rvk_queries.next() {
            None => return Err("Missing mandatory `rvk` query"),
            Some((_, value)) => {
                import_root_verify_key(&value).or(Err("Invalid `rvk` query value"))?
            }
        };
        if rvk_queries.next().is_some() {
            return Err("Multiple values for query `rvk`");
        }

        Ok(Self {
            base,
            organization_id,
            root_verify_key,
        })
    }

    expose_BaseBackendAddr_fields!();

    pub fn to_url(&self) -> Url {
        let mut url = self.base.to_url();
        url.set_path(&format!("/{}", self.organization_id));
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
}

/*
 * BackendActionAddr
 */

pub enum BackendActionAddr {
    OrganizationBootstrap(BackendOrganizationBootstrapAddr),
    OrganizationFileLink(BackendOrganizationFileLinkAddr),
    Invitation(BackendInvitationAddr),
}

impl TryFrom<&str> for BackendActionAddr {
    type Error = &'static str;

    fn try_from(url: &str) -> Result<Self, Self::Error> {
        if let Ok(addr) = url.parse::<BackendOrganizationBootstrapAddr>() {
            return Ok(BackendActionAddr::OrganizationBootstrap(addr));
        }
        if let Ok(addr) = url.parse::<BackendOrganizationFileLinkAddr>() {
            return Ok(BackendActionAddr::OrganizationFileLink(addr));
        }
        if let Ok(addr) = url.parse::<BackendInvitationAddr>() {
            return Ok(BackendActionAddr::Invitation(addr));
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

    fn _from_str(url: &str) -> Result<Self, AddrError> {
        // Note `Url::parse` takes care of percent-encoding for query params
        let parsed = Url::parse(url).map_err(|_| "Invalid url")?;
        let pairs = parsed.query_pairs();

        let base = BaseBackendAddr::from_url(&parsed, &pairs)?;
        let organization_id = extract_organization_id(&parsed)?;
        let action = extract_action(&pairs)?;
        if action != "bootstrap_organization" {
            return Err("Expected `action=bootstrap_organization` query");
        }

        let mut token_queries = pairs.filter(|(k, _)| k == "token");
        let token = token_queries.next().map(|(_, v)| (*v).to_owned());
        if token_queries.next().is_some() {
            return Err("Multiple values for query `token`");
        }

        Ok(Self {
            base,
            organization_id,
            token,
        })
    }

    expose_BaseBackendAddr_fields!();

    pub fn to_url(&self) -> Url {
        let mut url = self.base.to_url();
        url.set_path(&format!("/{}", self.organization_id));
        url.query_pairs_mut()
            .append_pair("action", "bootstrap_organization")
            // For legacy reasons, token must always be provided, hence default
            // token is the empty one (which is used for spontaneous organization
            // bootstrap without prior organization creation)
            .append_pair("token", if let Some(ref x) = self.token { x } else { "" });
        url
    }

    pub fn organization_id(&self) -> &OrganizationID {
        &self.organization_id
    }

    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
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

    fn _from_str(url: &str) -> Result<Self, AddrError> {
        // Note `Url::parse` takes care of percent-encoding for query params
        let parsed = Url::parse(url).map_err(|_| "Invalid url")?;
        let pairs = parsed.query_pairs();

        let base = BaseBackendAddr::from_url(&parsed, &pairs)?;
        let organization_id = extract_organization_id(&parsed)?;
        let action = extract_action(&pairs)?;
        if action != "file_link" {
            return Err("Expected `action=file_link` query");
        }

        let mut workspace_id_queries = pairs.filter(|(k, _)| k == "workspace_id");
        let workspace_id = match workspace_id_queries.next() {
            None => return Err("Missing mandatory `workspace_id` query"),
            Some((_, value)) => value
                .parse::<EntryID>()
                .or(Err("Invalid `workspace_id` query value"))?,
        };
        if workspace_id_queries.next().is_some() {
            return Err("Multiple values for query `workspace_id`");
        }

        let mut path_queries = pairs.filter(|(k, _)| k == "path");
        let encrypted_path = match path_queries.next() {
            None => return Err("Missing mandatory `path` query"),
            Some((_, value)) => {
                binary_urlsafe_decode(&value).or(Err("Invalid `path` query value"))?
            }
        };
        if path_queries.next().is_some() {
            return Err("Multiple values for query `path`");
        }

        Ok(Self {
            base,
            organization_id,
            workspace_id,
            encrypted_path,
        })
    }

    expose_BaseBackendAddr_fields!();

    pub fn to_url(&self) -> Url {
        let mut url = self.base.to_url();
        url.set_path(&format!("/{}", self.organization_id));
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

    fn _from_str(url: &str) -> Result<Self, AddrError> {
        // Note `Url::parse` takes care of percent-encoding for query params
        let parsed = Url::parse(url).map_err(|_| "Invalid url")?;
        let pairs = parsed.query_pairs();

        let base = BaseBackendAddr::from_url(&parsed, &pairs)?;
        let organization_id = extract_organization_id(&parsed)?;
        let invitation_type = match extract_action(&pairs)? {
            x if x == "claim_user" => InvitationType::User,
            x if x == "claim_device" => InvitationType::Device,
            _ => return Err("Expected `action=claim_user` or `action=claim_device` query value"),
        };

        let mut token_queries = pairs.filter(|(k, _)| k == "token");
        let token = match token_queries.next() {
            None => return Err("Missing mandatory `token` query"),
            Some((_, value)) => value
                .parse::<InvitationToken>()
                .or(Err("Invalid `token` query value"))?,
        };
        if token_queries.next().is_some() {
            return Err("Multiple values for query `token`");
        }

        Ok(Self {
            base,
            organization_id,
            invitation_type,
            token,
        })
    }

    expose_BaseBackendAddr_fields!();

    pub fn to_url(&self) -> Url {
        let mut url = self.base.to_url();
        url.set_path(&format!("/{}", self.organization_id));
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

    pub fn invitation_type(&self) -> &InvitationType {
        &self.invitation_type
    }

    pub fn token(&self) -> &InvitationToken {
        &self.token
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
    BASE32.encode(data).replace("=", "s")
}

pub fn binary_urlsafe_decode(data: &str) -> Result<Vec<u8>, &'static str> {
    let err = Err("Invalid base32 data");
    BASE32.decode(data.replace("s", "=").as_bytes()).or(err)
}

pub fn export_root_verify_key(key: &VerifyKey) -> String {
    binary_urlsafe_encode(key.as_ref())
}

pub fn import_root_verify_key(encoded: &str) -> Result<VerifyKey, &'static str> {
    let err_msg = "Invalid root verify key";
    let encoded_b32 = encoded.replace("s", "=");
    // TODO: would be better to directly decode into key
    let buff = BASE32.decode(encoded_b32.as_bytes()).or(Err(err_msg))?;
    VerifyKey::try_from(buff.as_slice()).map_err(|_| err_msg)
}

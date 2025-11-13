// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

/// This type describe the supported SSO authentication that can be used with OpenBao.
///
/// Note this type is not serializable: this is because instead we serialize it as
/// an opaque string for backward compatibility.
///
/// Typically if a local device protected by OpenBao has been created while
/// authenticated with a given SSO auth, we don't want the deserialization to
/// fail if this SSO auth is not supported by our client (instead the GUI is
/// expected to just let the user choose among all the supported SSO auth).
///
/// So this enum mostly used in two places:
/// - In the server to configure the SSO auth
/// - In the GUI to know how to display a given SSO auth
///
/// And between them (i.e. `server_config` API command and `device_file_openbao` schema),
/// the enum is turned into an opaque string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
pub enum OpenBaoAuthType {
    Hexagone,
    ProConnect,
}

impl OpenBaoAuthType {
    pub fn as_str(&self) -> &str {
        match self {
            OpenBaoAuthType::Hexagone => "HEXAGONE",
            OpenBaoAuthType::ProConnect => "PRO_CONNECT",
        }
    }
}

impl TryFrom<&str> for OpenBaoAuthType {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "HEXAGONE" => Ok(OpenBaoAuthType::Hexagone),
            "PRO_CONNECT" => Ok(OpenBaoAuthType::ProConnect),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
#[path = "../tests/unit/openbao.rs"]
mod tests;

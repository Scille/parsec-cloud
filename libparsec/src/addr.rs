// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum ParseParsecAddrError {
    #[error("Invalid URL")]
    InvalidUrl,
}

pub enum ParsedParsecAddr {
    Server {
        hostname: String,
        port: u16,
        is_default_port: bool,
        use_ssl: bool,
    },
    Organization {
        hostname: String,
        port: u16,
        is_default_port: bool,
        use_ssl: bool,
        organization_id: OrganizationID,
    },
    OrganizationBootstrap {
        hostname: String,
        port: u16,
        is_default_port: bool,
        use_ssl: bool,
        organization_id: OrganizationID,
        token: Option<String>,
    },
    WorkspacePath {
        hostname: String,
        port: u16,
        is_default_port: bool,
        use_ssl: bool,
        organization_id: OrganizationID,
        workspace_id: VlobID,
        key_index: IndexInt,
        encrypted_path: Vec<u8>,
    },
    InvitationUser {
        hostname: String,
        port: u16,
        is_default_port: bool,
        use_ssl: bool,
        organization_id: OrganizationID,
        token: InvitationToken,
    },
    InvitationDevice {
        hostname: String,
        port: u16,
        is_default_port: bool,
        use_ssl: bool,
        organization_id: OrganizationID,
        token: InvitationToken,
    },
    InvitationShamirRecovery {
        hostname: String,
        port: u16,
        is_default_port: bool,
        use_ssl: bool,
        organization_id: OrganizationID,
        token: InvitationToken,
    },
    PkiEnrollment {
        hostname: String,
        port: u16,
        is_default_port: bool,
        use_ssl: bool,
        organization_id: OrganizationID,
    },
}

pub fn parse_parsec_addr(url: &str) -> Result<ParsedParsecAddr, ParseParsecAddrError> {
    if let Ok(addr) = ParsecActionAddr::from_any(url) {
        Ok(match addr {
            ParsecActionAddr::OrganizationBootstrap(addr) => {
                ParsedParsecAddr::OrganizationBootstrap {
                    hostname: addr.hostname().into(),
                    port: addr.port(),
                    is_default_port: addr.is_default_port(),
                    use_ssl: addr.use_ssl(),
                    organization_id: addr.organization_id().clone(),
                    token: addr.token().map(|x| x.to_string()),
                }
            }
            ParsecActionAddr::WorkspacePath(addr) => ParsedParsecAddr::WorkspacePath {
                hostname: addr.hostname().into(),
                port: addr.port(),
                is_default_port: addr.is_default_port(),
                use_ssl: addr.use_ssl(),
                organization_id: addr.organization_id().clone(),
                encrypted_path: addr.encrypted_path().clone(),
                key_index: addr.key_index(),
                workspace_id: addr.workspace_id(),
            },
            ParsecActionAddr::Invitation(addr) => match addr.invitation_type() {
                InvitationType::User => ParsedParsecAddr::InvitationUser {
                    hostname: addr.hostname().into(),
                    port: addr.port(),
                    is_default_port: addr.is_default_port(),
                    use_ssl: addr.use_ssl(),
                    organization_id: addr.organization_id().clone(),
                    token: addr.token(),
                },
                InvitationType::Device => ParsedParsecAddr::InvitationDevice {
                    hostname: addr.hostname().into(),
                    port: addr.port(),
                    is_default_port: addr.is_default_port(),
                    use_ssl: addr.use_ssl(),
                    organization_id: addr.organization_id().clone(),
                    token: addr.token(),
                },
                InvitationType::ShamirRecovery => ParsedParsecAddr::InvitationShamirRecovery {
                    hostname: addr.hostname().into(),
                    port: addr.port(),
                    is_default_port: addr.is_default_port(),
                    use_ssl: addr.use_ssl(),
                    organization_id: addr.organization_id().clone(),
                    token: addr.token(),
                },
            },
            ParsecActionAddr::PkiEnrollment(addr) => ParsedParsecAddr::PkiEnrollment {
                hostname: addr.hostname().into(),
                port: addr.port(),
                is_default_port: addr.is_default_port(),
                use_ssl: addr.use_ssl(),
                organization_id: addr.organization_id().clone(),
            },
        })
    } else if let Ok(addr) = ParsecOrganizationAddr::from_any(url) {
        Ok(ParsedParsecAddr::Organization {
            hostname: addr.hostname().into(),
            port: addr.port(),
            is_default_port: addr.is_default_port(),
            use_ssl: addr.use_ssl(),
            organization_id: addr.organization_id().clone(),
        })
    } else if let Ok(addr) = ParsecAddr::from_any(url) {
        Ok(ParsedParsecAddr::Server {
            hostname: addr.hostname().into(),
            port: addr.port(),
            is_default_port: addr.is_default_port(),
            use_ssl: addr.use_ssl(),
        })
    } else {
        Err(ParseParsecAddrError::InvalidUrl)
    }
}

pub fn build_parsec_organization_bootstrap_addr(
    addr: ParsecAddr,
    organization_id: OrganizationID,
) -> ParsecOrganizationBootstrapAddr {
    ParsecOrganizationBootstrapAddr::new(addr, organization_id, None)
}

pub fn build_parsec_addr(hostname: String, port: Option<u16>, use_ssl: bool) -> ParsecAddr {
    ParsecAddr::new(hostname, port, use_ssl)
}

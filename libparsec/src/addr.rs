// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum ParseBackendAddrError {
    #[error("Invalid URL")]
    InvalidUrl,
}

pub enum ParsedParsecAddr {
    Server {
        hostname: String,
        port: u32,
        use_ssl: bool,
    },
    Organization {
        hostname: String,
        port: u32,
        use_ssl: bool,
        organization_id: OrganizationID,
    },
    OrganizationBootstrap {
        hostname: String,
        port: u32,
        use_ssl: bool,
        organization_id: OrganizationID,
        token: Option<String>,
    },
    OrganizationFileLink {
        hostname: String,
        port: u32,
        use_ssl: bool,
        organization_id: OrganizationID,
        workspace_id: VlobID,
        encrypted_path: Vec<u8>,
        encrypted_timestamp: Option<Vec<u8>>,
    },
    InvitationUser {
        hostname: String,
        port: u32,
        use_ssl: bool,
        organization_id: OrganizationID,
        token: InvitationToken,
    },
    InvitationDevice {
        hostname: String,
        port: u32,
        use_ssl: bool,
        organization_id: OrganizationID,
        token: InvitationToken,
    },
    PkiEnrollment {
        hostname: String,
        port: u32,
        use_ssl: bool,
        organization_id: OrganizationID,
    },
}

pub fn parse_backend_addr(url: &str) -> Result<ParsedParsecAddr, ParseBackendAddrError> {
    if let Ok(addr) = ParsecActionAddr::from_any(url) {
        Ok(match addr {
            ParsecActionAddr::OrganizationBootstrap(addr) => {
                ParsedParsecAddr::OrganizationBootstrap {
                    hostname: addr.hostname().into(),
                    port: addr.port() as u32,
                    use_ssl: addr.use_ssl(),
                    organization_id: addr.organization_id().clone(),
                    token: addr.token().map(|x| x.to_string()),
                }
            }
            ParsecActionAddr::OrganizationFileLink(addr) => {
                ParsedParsecAddr::OrganizationFileLink {
                    hostname: addr.hostname().into(),
                    port: addr.port() as u32,
                    use_ssl: addr.use_ssl(),
                    organization_id: addr.organization_id().clone(),
                    encrypted_path: addr.encrypted_path().clone(),
                    encrypted_timestamp: addr.encrypted_timestamp().clone(),
                    workspace_id: addr.workspace_id(),
                }
            }
            ParsecActionAddr::Invitation(addr) => match addr.invitation_type() {
                InvitationType::User => ParsedParsecAddr::InvitationUser {
                    hostname: addr.hostname().into(),
                    port: addr.port() as u32,
                    use_ssl: addr.use_ssl(),
                    organization_id: addr.organization_id().clone(),
                    token: addr.token(),
                },
                InvitationType::Device => ParsedParsecAddr::InvitationDevice {
                    hostname: addr.hostname().into(),
                    port: addr.port() as u32,
                    use_ssl: addr.use_ssl(),
                    organization_id: addr.organization_id().clone(),
                    token: addr.token(),
                },
            },
            ParsecActionAddr::PkiEnrollment(addr) => ParsedParsecAddr::PkiEnrollment {
                hostname: addr.hostname().into(),
                port: addr.port() as u32,
                use_ssl: addr.use_ssl(),
                organization_id: addr.organization_id().clone(),
            },
        })
    } else if let Ok(addr) = ParsecOrganizationAddr::from_any(url) {
        Ok(ParsedParsecAddr::Organization {
            hostname: addr.hostname().into(),
            port: addr.port() as u32,
            use_ssl: addr.use_ssl(),
            organization_id: addr.organization_id().clone(),
        })
    } else if let Ok(addr) = ParsecAddr::from_any(url) {
        Ok(ParsedParsecAddr::Server {
            hostname: addr.hostname().into(),
            port: addr.port() as u32,
            use_ssl: addr.use_ssl(),
        })
    } else {
        Err(ParseBackendAddrError::InvalidUrl)
    }
}

pub fn build_backend_organization_bootstrap_addr(
    addr: ParsecAddr,
    organization_id: OrganizationID,
) -> ParsecOrganizationBootstrapAddr {
    ParsecOrganizationBootstrapAddr::new(addr, organization_id, None)
}

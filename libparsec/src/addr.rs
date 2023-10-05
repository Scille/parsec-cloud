// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum ParseBackendAddrError {
    #[error("Invalid URL")]
    InvalidUrl,
}

pub enum ParsedBackendAddr {
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

pub fn parse_backend_addr(url: &str) -> Result<ParsedBackendAddr, ParseBackendAddrError> {
    if let Ok(addr) = BackendActionAddr::from_any(url) {
        Ok(match addr {
            BackendActionAddr::OrganizationBootstrap(addr) => {
                ParsedBackendAddr::OrganizationBootstrap {
                    hostname: addr.hostname().into(),
                    port: addr.port() as u32,
                    use_ssl: addr.use_ssl(),
                    organization_id: addr.organization_id().clone(),
                    token: addr.token().map(|x| x.to_string()),
                }
            }
            BackendActionAddr::OrganizationFileLink(addr) => {
                ParsedBackendAddr::OrganizationFileLink {
                    hostname: addr.hostname().into(),
                    port: addr.port() as u32,
                    use_ssl: addr.use_ssl(),
                    organization_id: addr.organization_id().clone(),
                    encrypted_path: addr.encrypted_path().clone(),
                    encrypted_timestamp: addr.encrypted_timestamp().clone(),
                    workspace_id: addr.workspace_id(),
                }
            }
            BackendActionAddr::Invitation(addr) => match addr.invitation_type() {
                InvitationType::User => ParsedBackendAddr::InvitationUser {
                    hostname: addr.hostname().into(),
                    port: addr.port() as u32,
                    use_ssl: addr.use_ssl(),
                    organization_id: addr.organization_id().clone(),
                    token: addr.token(),
                },
                InvitationType::Device => ParsedBackendAddr::InvitationDevice {
                    hostname: addr.hostname().into(),
                    port: addr.port() as u32,
                    use_ssl: addr.use_ssl(),
                    organization_id: addr.organization_id().clone(),
                    token: addr.token(),
                },
            },
            BackendActionAddr::PkiEnrollment(addr) => ParsedBackendAddr::PkiEnrollment {
                hostname: addr.hostname().into(),
                port: addr.port() as u32,
                use_ssl: addr.use_ssl(),
                organization_id: addr.organization_id().clone(),
            },
        })
    } else if let Ok(addr) = BackendOrganizationAddr::from_any(url) {
        Ok(ParsedBackendAddr::Organization {
            hostname: addr.hostname().into(),
            port: addr.port() as u32,
            use_ssl: addr.use_ssl(),
            organization_id: addr.organization_id().clone(),
        })
    } else if let Ok(addr) = BackendAddr::from_any(url) {
        Ok(ParsedBackendAddr::Server {
            hostname: addr.hostname().into(),
            port: addr.port() as u32,
            use_ssl: addr.use_ssl(),
        })
    } else {
        Err(ParseBackendAddrError::InvalidUrl)
    }
}

pub fn build_backend_organization_bootstrap_addr(
    addr: BackendAddr,
    organization_id: OrganizationID,
) -> BackendOrganizationBootstrapAddr {
    BackendOrganizationBootstrapAddr::new(addr, organization_id, None)
}

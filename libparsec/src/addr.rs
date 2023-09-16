// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::{
    thiserror, BackendAddr, BackendInvitationAddr, BackendOrganizationBootstrapAddr,
    BackendOrganizationFileLinkAddr, BackendPkiEnrollmentAddr, InvitationToken, InvitationType,
    OrganizationID, VlobID,
};

#[derive(Debug, thiserror::Error)]
pub enum ParseBackendAddrError {
    #[error("Invalid URL")]
    InvalidUrl,
}

pub enum ParsedBackendAddr {
    Base {
        hostname: String,
        port: u32,
        use_ssl: bool,
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
    if let Ok(addr) = BackendOrganizationBootstrapAddr::from_any(url) {
        Ok(ParsedBackendAddr::OrganizationBootstrap {
            hostname: addr.hostname().into(),
            port: addr.port() as u32,
            use_ssl: addr.use_ssl(),
            organization_id: addr.organization_id().clone(),
            token: addr.token().map(|x| x.to_string()),
        })
    } else if let Ok(addr) = BackendOrganizationFileLinkAddr::from_any(url) {
        Ok(ParsedBackendAddr::OrganizationFileLink {
            hostname: addr.hostname().into(),
            port: addr.port() as u32,
            use_ssl: addr.use_ssl(),
            organization_id: addr.organization_id().clone(),
            workspace_id: addr.workspace_id(),
            encrypted_path: addr.encrypted_path().clone(),
            encrypted_timestamp: addr.encrypted_timestamp().clone(),
        })
    } else if let Ok(addr) = BackendInvitationAddr::from_any(url) {
        Ok(match addr.invitation_type() {
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
        })
    } else if let Ok(addr) = BackendPkiEnrollmentAddr::from_any(url) {
        Ok(ParsedBackendAddr::PkiEnrollment {
            hostname: addr.hostname().into(),
            port: addr.port() as u32,
            use_ssl: addr.use_ssl(),
            organization_id: addr.organization_id().clone(),
        })
    } else {
        BackendAddr::from_any(url)
            .map(|addr| ParsedBackendAddr::Base {
                hostname: addr.hostname().into(),
                port: addr.port() as u32,
                use_ssl: addr.use_ssl(),
            })
            .map_err(|_| ParseBackendAddrError::InvalidUrl)
    }
}

pub fn build_backend_organization_bootstrap_addr(
    addr: BackendAddr,
    organization_id: OrganizationID,
) -> BackendOrganizationBootstrapAddr {
    BackendOrganizationBootstrapAddr::new(addr, organization_id, None)
}

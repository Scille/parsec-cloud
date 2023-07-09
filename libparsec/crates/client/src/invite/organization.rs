// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use libparsec_client_connection::{protocol::anonymous_cmds, AnonymousCmds, ConnectionError};
use libparsec_types::prelude::*;

use crate::{ClientConfig, EventBus, EventTooMuchDriftWithServerClock};

/*
 * new_user_invitation
 */

#[derive(Debug, thiserror::Error)]
pub enum BootstrapOrganizationError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Invalid bootstrap token")]
    InvalidToken,
    #[error("Bootstrap token already used")]
    AlreadyUsedToken,
    #[error("Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart")]
    BadTimestamp {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: f64,
        ballpark_client_late_offset: f64,
    },
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for BootstrapOrganizationError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            err => Self::Internal(err.into()),
        }
    }
}

pub async fn bootstrap_organization(
    config: &ClientConfig,
    event_bus: EventBus,
    addr: BackendOrganizationBootstrapAddr,
    human_handle: Option<HumanHandle>,
    device_label: Option<DeviceLabel>,
    sequester_authority_verify_key: Option<SequesterVerifyKeyDer>,
) -> Result<LocalDevice, BootstrapOrganizationError> {
    use anonymous_cmds::latest::organization_bootstrap::{Rep, Req};

    let root_signing_key = SigningKey::generate();
    let root_verify_key = root_signing_key.verify_key();
    let bootstrap_token = addr.token().unwrap_or("").to_owned();
    let organization_id = addr.organization_id().clone();

    let organization_addr =
        BackendOrganizationAddr::new(addr.clone(), organization_id, root_verify_key.clone());

    let device = LocalDevice::generate_new_device(
        organization_addr,
        None,
        UserProfile::Admin,
        human_handle,
        device_label,
        None,
        None,
    );

    let timestamp = device.now();
    let (user_certificate, redacted_user_certificate) = {
        let mut user_certificate = UserCertificate {
            author: CertificateSignerOwned::Root,
            timestamp,
            user_id: device.user_id().to_owned(),
            human_handle: device.human_handle.clone(),
            public_key: device.public_key(),
            profile: device.initial_profile,
        };
        let signed = user_certificate.dump_and_sign(&root_signing_key);

        user_certificate.human_handle = None;
        let redacted_signed = user_certificate.dump_and_sign(&root_signing_key);

        (signed.into(), redacted_signed.into())
    };

    let (device_certificate, redacted_device_certificate) = {
        let mut device_certificate = DeviceCertificate {
            author: CertificateSignerOwned::Root,
            timestamp,
            device_id: device.device_id.clone(),
            device_label: device.device_label.clone(),
            verify_key: device.verify_key(),
        };
        let signed = device_certificate.dump_and_sign(&root_signing_key);

        device_certificate.device_label = None;
        let redacted_signed = device_certificate.dump_and_sign(&root_signing_key);

        (signed.into(), redacted_signed.into())
    };

    let sequester_authority_certificate = match sequester_authority_verify_key {
        Some(sequester_authority_verify_key) => {
            let sequester_authority_certificate = SequesterAuthorityCertificate {
                timestamp,
                verify_key_der: sequester_authority_verify_key,
            };
            Some(
                sequester_authority_certificate
                    .dump_and_sign(&root_signing_key)
                    .into(),
            )
        }
        None => None,
    };

    let cmds = AnonymousCmds::new(
        &config.config_dir,
        BackendAnonymousAddr::BackendOrganizationBootstrapAddr(addr),
        config.proxy.clone(),
    )?;

    let req = Req {
        bootstrap_token,
        root_verify_key,
        user_certificate,
        redacted_user_certificate,
        device_certificate,
        redacted_device_certificate,
        sequester_authority_certificate,
    };

    let rep = cmds.send(req).await?;

    match rep {
        Rep::Ok => Ok(device),
        Rep::AlreadyBootstrapped => Err(BootstrapOrganizationError::AlreadyUsedToken),
        Rep::NotFound => Err(BootstrapOrganizationError::InvalidToken),
        Rep::BadTimestamp {
            backend_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            ..
        } => {
            let event = EventTooMuchDriftWithServerClock {
                backend_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
                client_timestamp,
            };
            event_bus.send(&event);

            Err(BootstrapOrganizationError::BadTimestamp {
                server_timestamp: backend_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            })
        }
        rep @ Rep::InvalidCertification { .. } | rep @ Rep::InvalidData { .. } => {
            // TODO: log those errors ?
            Err(anyhow::anyhow!(
                "Unexpected server response: {:?} (we sent invalid data ?)",
                rep
            )
            .into())
        }
        rep @ Rep::UnknownStatus { .. } => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", rep).into())
        }
    }
}

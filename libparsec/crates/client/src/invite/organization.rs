// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::Path, sync::Arc};

use libparsec_client_connection::{protocol::anonymous_cmds, AnonymousCmds, ConnectionError};
use libparsec_platform_device_loader::{AvailableDevice, DeviceSaveStrategy};
use libparsec_types::prelude::*;

use crate::{ClientConfig, EventBus, EventTooMuchDriftWithServerClock};

#[derive(Debug, thiserror::Error)]
pub enum BootstrapOrganizationError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Organization has expired")]
    OrganizationExpired,
    #[error("Invalid bootstrap token")]
    InvalidToken,
    #[error("Bootstrap token already used")]
    AlreadyUsedToken,
    #[error("Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart")]
    TimestampOutOfBallpark {
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
            ConnectionError::ExpiredOrganization => Self::OrganizationExpired,
            err => Self::Internal(err.into()),
        }
    }
}

pub async fn bootstrap_organization(
    config: Arc<ClientConfig>,
    event_bus: EventBus,
    addr: ParsecOrganizationBootstrapAddr,
    human_handle: HumanHandle,
    device_label: DeviceLabel,
    sequester_authority_verify_key: Option<SequesterVerifyKeyDer>,
) -> Result<OrganizationBootstrapFinalizeCtx, BootstrapOrganizationError> {
    use anonymous_cmds::latest::organization_bootstrap::{Rep, Req};

    let root_signing_key = SigningKey::generate();
    let root_verify_key = root_signing_key.verify_key();
    let bootstrap_token = addr.token().cloned();
    let organization_id = addr.organization_id().clone();

    let organization_addr =
        ParsecOrganizationAddr::new(&addr, organization_id, root_verify_key.clone());

    let device = Arc::new(LocalDevice::generate_new_device(
        organization_addr,
        UserProfile::Admin,
        human_handle,
        device_label,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ));

    let timestamp = device.now();
    let (user_certificate, redacted_user_certificate) = {
        let mut user_certificate = UserCertificate {
            author: CertificateSigner::Root,
            timestamp,
            user_id: device.user_id,
            human_handle: MaybeRedacted::Real(device.human_handle.clone()),
            public_key: device.public_key(),
            algorithm: PrivateKeyAlgorithm::X25519XSalsa20Poly1305,
            profile: device.initial_profile,
        };
        let signed = user_certificate.dump_and_sign(&root_signing_key);

        user_certificate.human_handle =
            MaybeRedacted::Redacted(HumanHandle::new_redacted(device.user_id));
        let redacted_signed = user_certificate.dump_and_sign(&root_signing_key);

        (signed.into(), redacted_signed.into())
    };

    let (device_certificate, redacted_device_certificate) = {
        let mut device_certificate = DeviceCertificate {
            author: CertificateSigner::Root,
            timestamp,
            purpose: DevicePurpose::Standard,
            user_id: device.user_id,
            device_id: device.device_id,
            device_label: MaybeRedacted::Real(device.device_label.clone()),
            verify_key: device.verify_key(),
            algorithm: SigningKeyAlgorithm::Ed25519,
        };
        let signed = device_certificate.dump_and_sign(&root_signing_key);

        device_certificate.device_label =
            MaybeRedacted::Redacted(DeviceLabel::new_redacted(device.device_id));
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

    let cmds = AnonymousCmds::new(&config.config_dir, addr.into(), config.proxy.clone())?;

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
        Rep::Ok => Ok(OrganizationBootstrapFinalizeCtx {
            new_local_device: device,
            config,
            event_bus,
        }),
        Rep::OrganizationAlreadyBootstrapped => Err(BootstrapOrganizationError::AlreadyUsedToken),
        Rep::InvalidBootstrapToken => Err(BootstrapOrganizationError::InvalidToken),
        Rep::TimestampOutOfBallpark {
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            client_timestamp,
            server_timestamp,
        } => {
            let event = EventTooMuchDriftWithServerClock {
                server_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
                client_timestamp,
            };
            event_bus.send(&event);

            Err(BootstrapOrganizationError::TimestampOutOfBallpark {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            })
        }
        rep @ Rep::InvalidCertificate => {
            // TODO: log error
            Err(anyhow::anyhow!(
                "Unexpected server response: {:?} (we sent invalid data ?)",
                rep
            )
            .into())
        }
        rep @ Rep::UnknownStatus { .. } => {
            // TODO: log error
            Err(anyhow::anyhow!("Unexpected server response: {:?}", rep).into())
        }
    }
}

#[derive(Debug)]
pub struct OrganizationBootstrapFinalizeCtx {
    pub new_local_device: Arc<LocalDevice>,
    config: Arc<ClientConfig>,
    #[allow(unused)]
    event_bus: EventBus,
}

// Only needed for test
pub fn test_organization_bootstrap_finalize_ctx_factory(
    config: Arc<ClientConfig>,
    new_local_device: Arc<LocalDevice>,
) -> OrganizationBootstrapFinalizeCtx {
    OrganizationBootstrapFinalizeCtx {
        config,
        new_local_device,
        event_bus: EventBus::default(),
    }
}

pub type OrganizationBootstrapFinalizeSaveLocalDeviceError =
    libparsec_platform_device_loader::SaveDeviceError;

impl OrganizationBootstrapFinalizeCtx {
    pub async fn save_local_device(
        self,
        strategy: &DeviceSaveStrategy,
        key_file: &Path,
    ) -> Result<AvailableDevice, OrganizationBootstrapFinalizeSaveLocalDeviceError> {
        // The organization is brand new, of course there is no existing
        // remote user manifest, hence our placeholder is non-speculative.
        libparsec_platform_storage::user::user_storage_non_speculative_init(
            &self.config.data_base_dir,
            &self.new_local_device,
        )
        .await
        .map_err(|e| {
            OrganizationBootstrapFinalizeSaveLocalDeviceError::Internal(
                e.context("Cannot initialize device storage"),
            )
        })?;

        libparsec_platform_device_loader::save_device(
            &self.config.config_dir,
            strategy,
            &self.new_local_device,
            key_file.to_path_buf(),
        )
        .await
    }
}

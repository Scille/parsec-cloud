// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::{protocol::anonymous_cmds, AnonymousCmds, ConnectionError};
use libparsec_types::prelude::*;

use crate::{ClientConfig, EventBus, EventTooMuchDriftWithServerClock};

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
    config: Arc<ClientConfig>,
    event_bus: EventBus,
    addr: BackendOrganizationBootstrapAddr,
    human_handle: Option<HumanHandle>,
    device_label: Option<DeviceLabel>,
    sequester_authority_verify_key: Option<SequesterVerifyKeyDer>,
) -> Result<OrganizationBootstrapFinalizeCtx, BootstrapOrganizationError> {
    use anonymous_cmds::latest::organization_bootstrap::{Rep, Req};

    let root_signing_key = SigningKey::generate();
    let root_verify_key = root_signing_key.verify_key();
    let bootstrap_token = addr.token().unwrap_or("").to_owned();
    let organization_id = addr.organization_id().clone();

    let organization_addr =
        BackendOrganizationAddr::new(&addr, organization_id, root_verify_key.clone());

    let device = Arc::new(LocalDevice::generate_new_device(
        organization_addr,
        UserProfile::Admin,
        None,
        human_handle,
        device_label,
        None,
        None,
    ));

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
        Rep::Ok => Ok(OrganizationBootstrapFinalizeCtx {
            new_local_device: device,
            config,
            event_bus,
        }),
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

impl OrganizationBootstrapFinalizeCtx {
    pub async fn save_local_device(
        self,
        access: &DeviceAccessStrategy,
    ) -> Result<AvailableDevice, anyhow::Error> {
        // The organization is brand new, of course there is no existing
        // remote user manifest, hence our placeholder is non-speculative.
        libparsec_platform_storage::user::user_storage_non_speculative_init(
            &self.config.data_base_dir,
            &self.new_local_device,
        )
        .await
        .map_err(|e| {
            // TODO: log error
            anyhow::anyhow!("Error while initializing device's user storage: {e}")
        })?;

        libparsec_platform_device_loader::save_device(
            &self.config.config_dir,
            access,
            &self.new_local_device,
        )
        .await
        .map_err(|e| {
            // TODO: log error
            anyhow::anyhow!("Error while saving the device file: {e}")
        })?;

        let (key_file_path, ty) = match access {
            DeviceAccessStrategy::Password { key_file, .. } => {
                (key_file.to_owned(), DeviceFileType::Password)
            }
            DeviceAccessStrategy::Smartcard { key_file } => {
                (key_file.to_owned(), DeviceFileType::Smartcard)
            }
        };

        Ok(AvailableDevice {
            key_file_path,
            organization_id: self.new_local_device.organization_id().to_owned(),
            device_id: self.new_local_device.device_id.clone(),
            // TODO: this will break if human handle / device label is None,
            // this is expected and will be overwritten in the next commit squash
            // *if you see this in the code review, it's time to complain !*
            device_label: self.new_local_device.device_label.clone().expect("TODO"),
            human_handle: self.new_local_device.human_handle.clone().expect("TODO"),
            slug: self.new_local_device.slug(),
            ty,
        })
    }
}

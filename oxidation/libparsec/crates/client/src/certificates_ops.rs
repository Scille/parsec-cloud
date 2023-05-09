// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

#![allow(dead_code)]

use paste::paste;
use std::{path::Path, sync::Arc};

use libparsec_client_connection::{AuthenticatedCmds, ConnectionError};
use libparsec_platform_storage2::certificates as storage;
use libparsec_protocol::authenticated_cmds;
use libparsec_types::prelude::*;

use crate::event_bus::EventBus;

#[derive(Debug, thiserror::Error)]
pub enum InvalidCertificateError {
    #[error("Certificate `{hint}` is corrupted: {error}")]
    Corrupted { hint: String, error: DataError },
    #[error("Certificate `{hint}` declares to be signed by `{author}`, which doesn't exist !")]
    UnknownAuthor { hint: String, author: DeviceID },
    #[error("Certificate `{hint}` declares to be signed by itself, which is not allowed !")]
    SelfSigned { hint: String },
    #[error(transparent)]
    AlreadyKnownButMismatchUser(#[from] storage::UserCertificateAlreadyKnownButMismatch),
    #[error(transparent)]
    AlreadyKnownButMismatchDevice(#[from] storage::DeviceCertificateAlreadyKnownButMismatch),
    #[error(transparent)]
    AlreadyKnownButMismatchRevokedUser(
        #[from] storage::RevokedUserCertificateAlreadyKnownButMismatch,
    ),
    #[error(transparent)]
    AlreadyKnownButMismatchRealmRole(#[from] storage::RealmRoleCertificateAlreadyKnownButMismatch),
    #[error(transparent)]
    AlreadyKnownButMismatchSequesterAuthority(
        #[from] storage::SequesterAuthorityCertificateAlreadyKnownButMismatch,
    ),
    #[error(transparent)]
    AlreadyKnownButMismatchSequesterService(
        #[from] storage::SequesterServiceCertificateAlreadyKnownButMismatch,
    ),
}

#[derive(Debug, thiserror::Error)]
pub enum AddCertificateError {
    #[error(transparent)]
    InvalidCertificate(#[from] InvalidCertificateError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum PollServerError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("A certificate provided by the server is invalid: {0}")]
    InvalidCertificate(#[from] InvalidCertificateError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for PollServerError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            err => Self::Internal(err.into()),
        }
    }
}

impl From<AddCertificateError> for PollServerError {
    fn from(value: AddCertificateError) -> Self {
        match value {
            AddCertificateError::InvalidCertificate(err) => err.into(),
            AddCertificateError::Internal(err) => err.into(),
        }
    }
}

#[derive(Debug)]
pub struct CertificatesOps {
    device: Arc<LocalDevice>,
    event_bus: EventBus,
    cmds: Arc<AuthenticatedCmds>,
    storage: storage::CertificatesStorage,
}

impl CertificatesOps {
    pub async fn new(
        data_base_dir: &Path,
        device: Arc<LocalDevice>,
        event_bus: EventBus,
        cmds: Arc<AuthenticatedCmds>,
    ) -> anyhow::Result<Self> {
        let storage = storage::CertificatesStorage::start(data_base_dir, device.clone()).await?;
        Ok(Self {
            device,
            event_bus,
            cmds,
            storage,
        })
    }

    pub async fn stop(&self) {
        self.storage.stop().await;
    }

    pub async fn poll_server_for_new_certificates(&self) -> Result<(), PollServerError> {
        let last_timestamp = self.storage.get_last_certificate_timestamp().await?;

        let request = authenticated_cmds::latest::certificate_get::Req {
            // Note this is `None` if our local storage is empty, meaning we want to fetch everything !
            created_after: last_timestamp,
        };
        let rep = self.cmds.send(request).await?;
        match rep {
            authenticated_cmds::latest::certificate_get::Rep::Ok { certificates } => {
                for certificate in certificates {
                    self.add_new_certificate(certificate).await?;
                }
                Ok(())
            }
            authenticated_cmds::latest::certificate_get::Rep::UnknownStatus {
                unknown_status,
                ..
            } => {
                Err(anyhow::anyhow!("Unknown error status `{}` from server", unknown_status).into())
            }
        }
    }

    pub async fn add_new_certificate(&self, certificate: Bytes) -> Result<(), AddCertificateError> {
        let unsecure = match AnyCertificate::unsecure_load(certificate) {
            Ok(unsecure) => unsecure,
            Err(error) => {
                // No information can be extracted form the binary data...
                let hint = "<unknown>".into();
                return Err(InvalidCertificateError::Corrupted { hint, error }.into());
            }
        };

        macro_rules! verify_signature_by_device {
            ($storage:expr, $unsecure:expr, $author:expr) => {
                match $storage.get_device_certificate($author).await? {
                    Some(author_certif) => $unsecure.verify_signature(&author_certif.verify_key),
                    None => {
                        // Unknown author... we don't try here to poll the server
                        // for new certificates: this is because certificate are
                        // supposed to be added in a strictly causal order, hence
                        // we are supposed to have already added all the certificates
                        // needed to validate this one. And if that's not the case
                        // it's suspicious and error sould be raised !
                        return Err(InvalidCertificateError::UnknownAuthor {
                            hint: $unsecure.hint(),
                            author: $author.clone(),
                        }
                        .into());
                    }
                }
            };
        }

        macro_rules! verify_signature_by_root_or_device {
            ($local_device:expr, $storage:expr, $unsecure:expr, $author:expr) => {
                match $author {
                    CertificateSignerOwned::Root => {
                        $unsecure.verify_signature($local_device.root_verify_key())
                    }
                    CertificateSignerOwned::User(author) => {
                        verify_signature_by_device!($storage, $unsecure, author)
                    }
                }
            };
        }

        macro_rules! check_verify_signature_result {
            ($verify_result:expr) => {
                match $verify_result {
                    Ok(verified) => verified,
                    Err((unsecure, error)) => {
                        let hint = unsecure.hint();
                        return Err(InvalidCertificateError::Corrupted { hint, error }.into());
                    }
                }
            };
        }

        macro_rules! store {
            ($certificate_type:ident, $storage:expr, $cooked:expr, $raw:expr) => {
                paste!{
                    $storage
                        .[< add_new_ $certificate_type:snake >](Arc::new($cooked), $raw)
                        .await
                        .map_err(|err| match err {
                            storage::[< Add $certificate_type Error >]::AlreadyKnownButMismatch(err) => {
                                AddCertificateError::InvalidCertificate(err.into())
                            }
                            storage::[< Add $certificate_type Error >]::Internal(err) => {
                                AddCertificateError::Internal(err)
                            }
                        })?;
                }
            };
        }

        match unsecure {
            UnsecureAnyCertificate::User(unsecure) => {
                let verify_result = verify_signature_by_root_or_device!(
                    self.device,
                    self.storage,
                    unsecure,
                    unsecure.author()
                );
                let (cooked, raw) = check_verify_signature_result!(verify_result);
                // TODO: check consistency
                store!(UserCertificate, self.storage, cooked, raw);
            }
            UnsecureAnyCertificate::Device(unsecure) => {
                let verify_result = verify_signature_by_root_or_device!(
                    self.device,
                    self.storage,
                    unsecure,
                    unsecure.author()
                );
                let (cooked, raw) = check_verify_signature_result!(verify_result);
                // TODO: check consistency
                store!(DeviceCertificate, self.storage, cooked, raw);
            }
            UnsecureAnyCertificate::RevokedUser(unsecure) => {
                let verify_result =
                    verify_signature_by_device!(self.storage, unsecure, unsecure.author());
                let (cooked, raw) = check_verify_signature_result!(verify_result);
                // TODO: check consistency
                store!(RevokedUserCertificate, self.storage, cooked, raw);
            }
            UnsecureAnyCertificate::RealmRole(unsecure) => {
                let verify_result = verify_signature_by_root_or_device!(
                    self.device,
                    self.storage,
                    unsecure,
                    unsecure.author()
                );
                let (cooked, raw) = check_verify_signature_result!(verify_result);
                // TODO: check consistency
                store!(RealmRoleCertificate, self.storage, cooked, raw);
            }
        }

        Ok(())
    }
}

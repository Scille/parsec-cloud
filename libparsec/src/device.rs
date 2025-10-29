// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::Path;

pub use libparsec_platform_device_loader::{
    ArchiveDeviceError, AvailableDevice, AvailableDeviceType, ListAvailableDeviceError,
    UpdateDeviceError,
};
use libparsec_types::prelude::*;

mod strategy {
    use std::{path::PathBuf, sync::Arc};

    use libparsec_account::{
        Account, AccountFetchOpaqueKeyFromVaultError, AccountUploadOpaqueKeyInVaultError,
    };
    use libparsec_crypto::{Password, SecretKey};
    use libparsec_platform_device_loader::{
        AccountVaultOperationsFetchOpaqueKeyError, AccountVaultOperationsFutureResult,
        AccountVaultOperationsUploadOpaqueKeyError,
    };
    use libparsec_types::prelude::*;

    use crate::handle::{borrow_from_handle, Handle, HandleItem};

    #[derive(Debug)]
    struct AccountVaultOperations {
        account: Arc<Account>,
    }

    impl libparsec_platform_device_loader::AccountVaultOperations for AccountVaultOperations {
        fn account_email(&self) -> &EmailAddress {
            self.account.human_handle().email()
        }

        fn fetch_opaque_key(
            &self,
            ciphertext_key_id: AccountVaultItemOpaqueKeyID,
        ) -> AccountVaultOperationsFutureResult<SecretKey, AccountVaultOperationsFetchOpaqueKeyError>
        {
            let account = self.account.clone();

            Box::pin(async move {
                account
                    .fetch_opaque_key_from_vault(ciphertext_key_id)
                    .await
                    .map_err(|err| match err {
                        AccountFetchOpaqueKeyFromVaultError::BadVaultKeyAccess(err) => {
                            AccountVaultOperationsFetchOpaqueKeyError::BadVaultKeyAccess(err)
                        }
                        AccountFetchOpaqueKeyFromVaultError::UnknownOpaqueKey => {
                            AccountVaultOperationsFetchOpaqueKeyError::UnknownOpaqueKey
                        }
                        AccountFetchOpaqueKeyFromVaultError::CorruptedOpaqueKey => {
                            AccountVaultOperationsFetchOpaqueKeyError::CorruptedOpaqueKey
                        }
                        AccountFetchOpaqueKeyFromVaultError::Offline(err) => {
                            AccountVaultOperationsFetchOpaqueKeyError::Offline(err)
                        }
                        AccountFetchOpaqueKeyFromVaultError::Internal(err) => {
                            AccountVaultOperationsFetchOpaqueKeyError::Internal(err)
                        }
                    })
            })
        }

        fn upload_opaque_key(
            &self,
        ) -> AccountVaultOperationsFutureResult<
            (AccountVaultItemOpaqueKeyID, SecretKey),
            AccountVaultOperationsUploadOpaqueKeyError,
        > {
            let account = self.account.clone();

            Box::pin(async move {
                account
                    .upload_opaque_key_in_vault()
                    .await
                    .map_err(|err| match err {
                        AccountUploadOpaqueKeyInVaultError::BadVaultKeyAccess(err) => {
                            AccountVaultOperationsUploadOpaqueKeyError::BadVaultKeyAccess(err)
                        }
                        AccountUploadOpaqueKeyInVaultError::Offline(err) => {
                            AccountVaultOperationsUploadOpaqueKeyError::Offline(err)
                        }
                        AccountUploadOpaqueKeyInVaultError::Internal(err) => {
                            AccountVaultOperationsUploadOpaqueKeyError::Internal(err)
                        }
                    })
            })
        }
    }

    #[derive(Debug, Clone)]
    pub enum DeviceSaveStrategy {
        Keyring,
        Password {
            password: Password,
        },
        Smartcard {
            certificate_reference: X509CertificateReference,
        },
        AccountVault {
            account_handle: Handle,
        },
    }

    impl DeviceSaveStrategy {
        /// This method may need to do side-effects (typically to obtain the `Account`
        /// object from its handle).
        /// Hence its funny name, and why we don't just replace it by a `impl From<...> for ...`
        pub fn convert_with_side_effects(
            self,
        ) -> anyhow::Result<libparsec_platform_device_loader::DeviceSaveStrategy> {
            Ok(match self {
                DeviceSaveStrategy::Keyring => {
                    libparsec_platform_device_loader::DeviceSaveStrategy::Keyring
                }
                DeviceSaveStrategy::Password { password } => {
                    libparsec_platform_device_loader::DeviceSaveStrategy::Password { password }
                }
                DeviceSaveStrategy::Smartcard {
                    certificate_reference,
                } => libparsec_platform_device_loader::DeviceSaveStrategy::Smartcard {
                    certificate_reference,
                },
                DeviceSaveStrategy::AccountVault { account_handle } => {
                    // Note `borrow_from_handle` does a side-effect here !
                    let account = borrow_from_handle(account_handle, |x| match x {
                        HandleItem::Account(account) => Some(account.clone()),
                        _ => None,
                    })?;

                    libparsec_platform_device_loader::DeviceSaveStrategy::AccountVault {
                        operations: Arc::new(AccountVaultOperations { account }),
                    }
                }
            })
        }
    }

    /// Represent how to load a device file
    #[derive(Debug, Clone)]
    pub enum DeviceAccessStrategy {
        Keyring {
            key_file: PathBuf,
        },
        Password {
            key_file: PathBuf,
            password: Password,
        },
        Smartcard {
            key_file: PathBuf,
        },
        AccountVault {
            key_file: PathBuf,
            account_handle: Handle,
        },
    }

    impl DeviceAccessStrategy {
        /// This method may need to do side-effects (typically to obtain the `Account`
        /// object from its handle).
        /// Hence its funny name, and why we don't just replace it by a `impl From<...> for ...`
        pub fn convert_with_side_effects(
            self,
        ) -> anyhow::Result<libparsec_platform_device_loader::DeviceAccessStrategy> {
            Ok(match self {
                DeviceAccessStrategy::Keyring { key_file } => {
                    libparsec_platform_device_loader::DeviceAccessStrategy::Keyring { key_file }
                }
                DeviceAccessStrategy::Password { key_file, password } => {
                    libparsec_platform_device_loader::DeviceAccessStrategy::Password {
                        key_file,
                        password,
                    }
                }
                DeviceAccessStrategy::Smartcard { key_file } => {
                    libparsec_platform_device_loader::DeviceAccessStrategy::Smartcard { key_file }
                }
                DeviceAccessStrategy::AccountVault {
                    key_file,
                    account_handle,
                } => {
                    // Note `borrow_from_handle` does a side-effect here !
                    let account = borrow_from_handle(account_handle, |x| match x {
                        HandleItem::Account(account) => Some(account.clone()),
                        _ => None,
                    })?;

                    libparsec_platform_device_loader::DeviceAccessStrategy::AccountVault {
                        key_file,
                        operations: Arc::new(AccountVaultOperations { account }),
                    }
                }
            })
        }
    }
}
pub use strategy::{DeviceAccessStrategy, DeviceSaveStrategy};

pub async fn list_available_devices(
    config_dir: &Path,
) -> Result<Vec<AvailableDevice>, ListAvailableDeviceError> {
    libparsec_platform_device_loader::list_available_devices(config_dir).await
}

pub async fn archive_device(
    config_dir: &Path,
    device_path: &Path,
) -> Result<(), ArchiveDeviceError> {
    libparsec_platform_device_loader::archive_device(config_dir, device_path).await
}

pub async fn update_device_change_authentication(
    config_dir: &Path,
    current_auth: DeviceAccessStrategy,
    new_auth: DeviceSaveStrategy,
) -> Result<AvailableDevice, UpdateDeviceError> {
    let current_auth = current_auth
        .convert_with_side_effects()
        .map_err(UpdateDeviceError::Internal)?;
    let new_auth = new_auth
        .convert_with_side_effects()
        .map_err(UpdateDeviceError::Internal)?;

    let key_file = current_auth.key_file().to_owned();

    libparsec_platform_device_loader::update_device_change_authentication(
        config_dir,
        &current_auth,
        &new_auth,
        &key_file,
    )
    .await
}

pub async fn update_device_overwrite_server_addr(
    config_dir: &Path,
    auth: DeviceAccessStrategy,
    new_server_addr: ParsecAddr,
) -> Result<ParsecAddr, UpdateDeviceError> {
    let auth = auth
        .convert_with_side_effects()
        .map_err(UpdateDeviceError::Internal)?;

    libparsec_platform_device_loader::update_device_overwrite_server_addr(
        config_dir,
        &auth,
        new_server_addr,
    )
    .await
}

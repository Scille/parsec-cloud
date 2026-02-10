// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::Path, sync::Arc};

use crate::config::ClientConfig;
pub use libparsec_platform_device_loader::{
    ArchiveDeviceError, AvailableDevice, AvailableDeviceType, ListAvailableDeviceError,
    UpdateDeviceError,
};
pub use libparsec_platform_storage::RemoveDeviceDataError;
use libparsec_types::prelude::*;

mod strategy {
    use std::{path::PathBuf, sync::Arc};

    use libparsec_account::{
        Account, AccountFetchOpaqueKeyFromVaultError, AccountUploadOpaqueKeyInVaultError,
    };
    use libparsec_crypto::{Password, SecretKey};
    use libparsec_openbao::{OpenBaoCmds, OpenBaoFetchOpaqueKeyError, OpenBaoUploadOpaqueKeyError};
    use libparsec_platform_async::{pretend_future_is_send_on_web, PinBoxFutureResult};
    use libparsec_platform_device_loader::{
        AccountVaultOperationsFetchOpaqueKeyError, AccountVaultOperationsUploadOpaqueKeyError,
        OpenBaoOperationsFetchOpaqueKeyError, OpenBaoOperationsUploadOpaqueKeyError,
    };
    use libparsec_types::prelude::*;

    use crate::handle::{borrow_from_handle, Handle, HandleItem};

    /*
     * Account vault operations
     */

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
        ) -> PinBoxFutureResult<SecretKey, AccountVaultOperationsFetchOpaqueKeyError> {
            let account = self.account.clone();

            Box::pin(pretend_future_is_send_on_web(async move {
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
            }))
        }

        fn upload_opaque_key(
            &self,
        ) -> PinBoxFutureResult<
            (AccountVaultItemOpaqueKeyID, SecretKey),
            AccountVaultOperationsUploadOpaqueKeyError,
        > {
            let account = self.account.clone();

            Box::pin(pretend_future_is_send_on_web(async move {
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
                        AccountUploadOpaqueKeyInVaultError::BadServerResponse(err) => {
                            AccountVaultOperationsUploadOpaqueKeyError::BadServerResponse(err)
                        }
                    })
            }))
        }
    }

    /*
     * OpenBao operations
     */

    #[derive(Debug)]
    struct OpenBaoDeviceAccessOperations {
        cmds: Arc<libparsec_openbao::OpenBaoCmds>,
    }

    #[derive(Debug)]
    struct OpenBaoDeviceSaveOperations {
        cmds: Arc<libparsec_openbao::OpenBaoCmds>,
        openbao_preferred_auth_id: String,
    }

    impl libparsec_platform_device_loader::OpenBaoDeviceSaveOperations for OpenBaoDeviceSaveOperations {
        fn openbao_entity_id(&self) -> &str {
            self.cmds.openbao_entity_id()
        }

        fn openbao_preferred_auth_id(&self) -> &str {
            &self.openbao_preferred_auth_id
        }

        fn upload_opaque_key(
            &self,
        ) -> PinBoxFutureResult<(String, SecretKey), OpenBaoOperationsUploadOpaqueKeyError>
        {
            let cmds = self.cmds.clone();

            Box::pin(pretend_future_is_send_on_web(async move {
                cmds.upload_opaque_key().await.map_err(|err| match err {
                    OpenBaoUploadOpaqueKeyError::BadURL(err) => {
                        OpenBaoOperationsUploadOpaqueKeyError::BadURL(err)
                    }
                    OpenBaoUploadOpaqueKeyError::NoServerResponse(err) => {
                        OpenBaoOperationsUploadOpaqueKeyError::NoServerResponse(err.into())
                    }
                    OpenBaoUploadOpaqueKeyError::BadServerResponse(err) => {
                        OpenBaoOperationsUploadOpaqueKeyError::BadServerResponse(err)
                    }
                })
            }))
        }

        fn to_access_operations(
            &self,
        ) -> Arc<dyn libparsec_platform_device_loader::OpenBaoDeviceAccessOperations> {
            Arc::new(OpenBaoDeviceAccessOperations {
                cmds: self.cmds.clone(),
            })
        }
    }

    impl libparsec_platform_device_loader::OpenBaoDeviceAccessOperations
        for OpenBaoDeviceAccessOperations
    {
        fn openbao_entity_id(&self) -> &str {
            self.cmds.openbao_entity_id()
        }

        fn fetch_opaque_key(
            &self,
            openbao_ciphertext_key_path: String,
        ) -> PinBoxFutureResult<SecretKey, OpenBaoOperationsFetchOpaqueKeyError> {
            let cmds = self.cmds.clone();

            Box::pin(pretend_future_is_send_on_web(async move {
                cmds.fetch_opaque_key(&openbao_ciphertext_key_path)
                    .await
                    .map_err(|err| match err {
                        OpenBaoFetchOpaqueKeyError::BadURL(err) => {
                            OpenBaoOperationsFetchOpaqueKeyError::BadURL(err)
                        }
                        OpenBaoFetchOpaqueKeyError::NoServerResponse(err) => {
                            OpenBaoOperationsFetchOpaqueKeyError::NoServerResponse(err.into())
                        }
                        OpenBaoFetchOpaqueKeyError::BadServerResponse(err) => {
                            OpenBaoOperationsFetchOpaqueKeyError::BadServerResponse(err)
                        }
                    })
            }))
        }

        fn to_save_operations(
            &self,
            openbao_preferred_auth_id: String,
        ) -> Arc<dyn libparsec_platform_device_loader::OpenBaoDeviceSaveOperations> {
            Arc::new(OpenBaoDeviceSaveOperations {
                cmds: self.cmds.clone(),
                openbao_preferred_auth_id,
            })
        }
    }

    /*
     * DeviceSaveStrategy
     */

    #[derive(Debug, Clone)]
    pub enum DeviceSaveStrategy {
        Keyring,
        Password {
            password: Password,
        },
        PKI {
            certificate_ref: X509CertificateReference,
        },
        AccountVault {
            account_handle: Handle,
        },
        OpenBao {
            openbao_server_url: String,
            openbao_secret_mount_path: String,
            openbao_transit_mount_path: String,
            openbao_entity_id: String,
            openbao_auth_token: String,
            openbao_preferred_auth_id: String,
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
                DeviceSaveStrategy::PKI { certificate_ref } => {
                    libparsec_platform_device_loader::DeviceSaveStrategy::PKI { certificate_ref }
                }
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
                DeviceSaveStrategy::OpenBao {
                    openbao_server_url,
                    openbao_secret_mount_path,
                    openbao_transit_mount_path,
                    openbao_entity_id,
                    openbao_auth_token,
                    openbao_preferred_auth_id,
                } => {
                    let client = libparsec_client_connection::build_client_with_proxy(
                        libparsec_client_connection::ProxyConfig::default(),
                    )?;

                    let cmds = Arc::new(OpenBaoCmds::new(
                        client,
                        openbao_server_url,
                        openbao_secret_mount_path,
                        openbao_transit_mount_path,
                        openbao_entity_id,
                        openbao_auth_token,
                    ));

                    libparsec_platform_device_loader::DeviceSaveStrategy::OpenBao {
                        operations: Arc::new(OpenBaoDeviceSaveOperations {
                            cmds,
                            openbao_preferred_auth_id,
                        }),
                    }
                }
            })
        }
    }

    /*
     * DeviceAccessStrategy
     */

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
        PKI {
            key_file: PathBuf,
        },
        AccountVault {
            key_file: PathBuf,
            account_handle: Handle,
        },
        OpenBao {
            key_file: PathBuf,
            openbao_server_url: String,
            openbao_secret_mount_path: String,
            openbao_transit_mount_path: String,
            openbao_entity_id: String,
            openbao_auth_token: String,
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
                DeviceAccessStrategy::PKI { key_file } => {
                    libparsec_platform_device_loader::DeviceAccessStrategy::PKI { key_file }
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
                DeviceAccessStrategy::OpenBao {
                    key_file,
                    openbao_server_url,
                    openbao_secret_mount_path,
                    openbao_transit_mount_path,
                    openbao_entity_id,
                    openbao_auth_token,
                } => {
                    let client = libparsec_client_connection::build_client()?;
                    let cmds = Arc::new(OpenBaoCmds::new(
                        client,
                        openbao_server_url,
                        openbao_secret_mount_path,
                        openbao_transit_mount_path,
                        openbao_entity_id,
                        openbao_auth_token,
                    ));

                    libparsec_platform_device_loader::DeviceAccessStrategy::OpenBao {
                        key_file,
                        operations: Arc::new(OpenBaoDeviceAccessOperations { cmds }),
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

pub async fn remove_device_data(
    config: ClientConfig,
    device_id: DeviceID,
) -> Result<(), RemoveDeviceDataError> {
    let config: Arc<libparsec_client::ClientConfig> = config.into();

    libparsec_platform_storage::remove_device_data(&config.data_base_dir, device_id).await
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

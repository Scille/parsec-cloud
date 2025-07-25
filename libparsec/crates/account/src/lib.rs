// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::Arc,
};

use libparsec_client_connection::{AnonymousAccountCmds, AuthenticatedAccountCmds, ProxyConfig};
use libparsec_types::prelude::*;

mod account_create;
mod account_delete;
mod account_recover;
mod create_registration_device;
mod fetch_opaque_key_from_vault;
mod fetch_vault_items;
mod list_invitations;
mod list_registration_devices;
mod login;
mod register_new_device;
mod upload_opaque_key_in_vault;

pub use account_create::*;
pub use account_delete::*;
pub use account_recover::*;
pub use create_registration_device::*;
pub use fetch_opaque_key_from_vault::*;
pub use fetch_vault_items::*;
pub use list_invitations::*;
pub use list_registration_devices::*;
pub use login::*;
pub use register_new_device::*;
pub use upload_opaque_key_in_vault::*;

#[derive(Debug)]
pub struct Account {
    config_dir: PathBuf,
    human_handle: HumanHandle,
    cmds: AuthenticatedAccountCmds,
    // Note the `Account` object is related to a single auth method, this means
    // we don't have to bother with vault rotation since, if it occurs, our auth
    // method will simply get invalidated.
    auth_method_secret_key: SecretKey,
}

impl Account {
    /// To create a new account, the user's email address must first be validated.
    /// This method ask the server to send an email containing a validation code.
    pub async fn create_1_send_validation_email(
        cmds: &AnonymousAccountCmds,
        email: EmailAddress,
    ) -> Result<(), AccountCreateSendValidationEmailError> {
        account_create_send_validation_email(cmds, email).await
    }

    /// Since the validation code is supposed to be copied by the user from his
    /// email client to the Parsec client by hand, this method can be used (but
    /// is not mandatory, `create_3_proceed` can also be called right away!) to
    /// ensure the validation code + email couple is valid and can lead.
    ///
    /// Also note that calling too many time this method with an invalid
    /// validation code leads to a full invalidation of the attempt and requires
    /// a new validation email to be sent.
    pub async fn create_2_check_validation_code(
        cmds: &AnonymousAccountCmds,
        validation_code: ValidationCode,
        email: EmailAddress,
    ) -> Result<(), AccountCreateError> {
        account_create(
            cmds,
            AccountCreateStep::CheckCode {
                validation_code,
                email,
            },
        )
        .await
    }

    /// Similar to `create_2_check_validation_code`, but this time we actually want
    /// to create the account!
    pub async fn create_3_proceed(
        cmds: &AnonymousAccountCmds,
        validation_code: ValidationCode,
        human_handle: HumanHandle,
        password: &Password,
    ) -> Result<(), AccountCreateError> {
        account_create(
            cmds,
            AccountCreateStep::Proceed {
                human_handle,
                password,
                validation_code,
            },
        )
        .await
    }

    /// Convenient helper to avoid the first request (fetching the human handle)
    /// whenever we need an `Account` instance in the tests.
    #[cfg(test)]
    pub async fn test_new(
        config_dir: PathBuf,
        addr: ParsecAddr,
        auth_method_master_secret: KeyDerivation,
        human_handle: HumanHandle,
    ) -> Self {
        account_login_with_master_secret(
            config_dir,
            ProxyConfig::default(),
            addr,
            auth_method_master_secret,
            None,
            Some(human_handle),
        )
        .await
        .unwrap()
    }

    pub fn human_handle(&self) -> &HumanHandle {
        &self.human_handle
    }

    pub fn config_dir(&self) -> &Path {
        &self.config_dir
    }

    pub async fn login_with_master_secret(
        config_dir: PathBuf,
        proxy: ProxyConfig,
        addr: ParsecAddr,
        auth_method_master_secret: KeyDerivation,
    ) -> Result<Self, AccountLoginWithMasterSecretError> {
        account_login_with_master_secret(
            config_dir,
            proxy,
            addr,
            auth_method_master_secret,
            None,
            None,
        )
        .await
    }

    pub async fn login_with_password(
        config_dir: PathBuf,
        proxy: ProxyConfig,
        addr: ParsecAddr,
        email: EmailAddress,
        password: &Password,
    ) -> Result<Self, AccountLoginWithPasswordError> {
        account_login_with_password(config_dir, proxy, addr, email, password).await
    }

    /// Fetch from the server the pending invitations across all organizations
    /// corresponding to the account's email address.
    pub async fn list_invitations(
        &self,
    ) -> Result<Vec<ParsecInvitationAddr>, AccountListInvitationsError> {
        account_list_invitations(self).await
    }

    /// Fetch the account vault items from the server and return the opaque
    /// key matching the ID.
    ///
    /// This key is typically supposed to be used to decrypt the `ciphertext`
    /// field of a `DeviceFileAccountVault`.
    pub async fn fetch_opaque_key_from_vault(
        &self,
        key_id: AccountVaultItemOpaqueKeyID,
    ) -> Result<SecretKey, AccountFetchOpaqueKeyFromVaultError> {
        account_fetch_opaque_key_from_vault(self, key_id).await
    }

    /// Generate a new opaque key and upload it as `AccountVaultItemOpaqueKey` in the
    /// account vault.
    ///
    /// This key is then typically supposed to be used while saving a local device
    /// to encrypt the `ciphertext` field of a `DeviceFileAccountVault`.
    pub async fn upload_opaque_key_in_vault(
        &self,
    ) -> Result<(AccountVaultItemOpaqueKeyID, SecretKey), AccountUploadOpaqueKeyInVaultError> {
        account_upload_opaque_key_in_vault(self).await
    }

    /// Fetch the account vault items from the server and return all available registration devices
    pub async fn list_registration_devices(
        &self,
    ) -> Result<HashSet<(OrganizationID, UserID)>, AccountListRegistrationDevicesError> {
        account_list_registration_devices(self).await
    }

    /// Use an existing local device to create (i.e. upload a device certificate)
    /// and upload (i.e. store a vault item) on the server a new device that will
    /// be used as registration device.
    pub async fn create_registration_device(
        &self,
        existing_local_device: Arc<LocalDevice>,
    ) -> Result<DeviceID, AccountCreateRegistrationDeviceError> {
        account_create_registration_device(self, existing_local_device).await
    }

    /// Use an existing registration device to create (i.e. upload a device certificate
    /// to the server) a new device that then will be saved locally.
    pub async fn register_new_device(
        &self,
        organization_id: OrganizationID,
        user_id: UserID,
        new_device_label: DeviceLabel,
        save_strategy: DeviceSaveStrategy,
    ) -> Result<AvailableDevice, AccountRegisterNewDeviceError> {
        account_register_new_device(
            self,
            organization_id,
            user_id,
            new_device_label,
            save_strategy,
        )
        .await
    }

    /// Before deleting the account, a confirmation email containing a validation
    /// code must first be send to the user's email address.
    pub async fn delete_1_send_validation_email(
        &self,
    ) -> Result<(), AccountDeleteSendValidationEmailError> {
        account_delete_send_validation_email(self).await
    }

    /// Actually delete the account by providing the validation code obtained
    /// from the confirmation email.
    ///
    /// Note that, unlike for account creation, there is no need for a "check code"
    /// flavor here. This is because since the user has no configuration to provide
    /// there is no need to validate a code before actually trying to use it.
    pub async fn delete_2_proceed(
        &self,
        validation_code: ValidationCode,
    ) -> Result<(), AccountDeleteProceedError> {
        account_delete_proceed(self, validation_code).await
    }

    /// Before recovering the account, a confirmation email containing a validation
    /// code must first be send to the user's email address.
    pub async fn recover_1_send_validation_email(
        cmds: &AnonymousAccountCmds,
        email: EmailAddress,
    ) -> Result<(), AccountRecoverSendValidationEmailError> {
        account_recover_send_validation_email(cmds, email).await
    }

    /// Actually recover the account by providing the validation code obtained
    /// from the confirmation email.
    ///
    /// Recovering the account means a new vault and authentication method will be
    /// setup (hence all items stored in the current vault will no longer be
    /// accessible until they are properly recovered).
    ///
    /// Note that, unlike for account creation, there is no need for a "check code"
    /// flavor here. This is because since the user has no configuration to provide
    /// there is no need to validate a code before actually trying to use it.
    pub async fn recover_2_proceed(
        cmds: &AnonymousAccountCmds,
        validation_code: ValidationCode,
        email: EmailAddress,
        new_password: &Password,
    ) -> Result<(), AccountRecoverProceedError> {
        account_recover_proceed(cmds, validation_code, email, new_password).await
    }
}

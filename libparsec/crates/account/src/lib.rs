// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::Arc,
};

use libparsec_client_connection::{AnonymousServerCmds, AuthenticatedAccountCmds, ProxyConfig};
use libparsec_platform_device_loader::{AvailableDevice, DeviceSaveStrategy};
use libparsec_types::prelude::*;

mod account_create;
mod account_delete;
mod account_recover;
mod auth_method_derived_keys;
mod create_auth_method;
mod create_registration_device;
mod disable_auth_method;
mod fetch_opaque_key_from_vault;
mod fetch_vault_items;
mod list_auth_methods;
mod list_invitations;
mod list_organizations;
mod list_registration_devices;
mod login;
mod register_new_device;
mod upload_opaque_key_in_vault;

pub use account_create::*;
pub use account_delete::*;
pub use account_recover::*;
use auth_method_derived_keys::*;
pub use create_auth_method::*;
pub use create_registration_device::*;
pub use disable_auth_method::*;
pub use fetch_opaque_key_from_vault::*;
pub use fetch_vault_items::*;
pub use list_auth_methods::*;
pub use list_invitations::*;
pub use list_organizations::*;
pub use list_registration_devices::*;
pub use login::*;
pub use register_new_device::*;
pub use upload_opaque_key_in_vault::*;

pub enum AccountAuthMethodStrategy<'a> {
    MasterSecret(&'a KeyDerivation),
    Password(&'a Password),
}

impl AccountAuthMethodStrategy<'_> {
    pub(crate) fn derive_keys(
        &self,
        email: &EmailAddress,
    ) -> (Option<UntrustedPasswordAlgorithm>, AuthMethodDerivedKeys) {
        match self {
            AccountAuthMethodStrategy::MasterSecret(auth_method_master_secret) => {
                let auth_method_keys = derive_auth_method_keys(auth_method_master_secret);
                (None, auth_method_keys)
            }

            AccountAuthMethodStrategy::Password(password) => {
                let auth_method_password_algorithm = PasswordAlgorithm::generate_argon2id(
                    PasswordAlgorithmSaltStrategy::DerivedFromEmail {
                        email: &email.to_string(),
                    },
                );
                let auth_method_master_secret = auth_method_password_algorithm
                    .compute_key_derivation(password)
                    .expect("algorithm config is valid");

                let auth_method_keys = derive_auth_method_keys(&auth_method_master_secret);
                (
                    Some(auth_method_password_algorithm.into()),
                    auth_method_keys,
                )
            }
        }
    }
}

pub enum AccountLoginStrategy<'a> {
    MasterSecret(&'a KeyDerivation),
    Password {
        email: &'a EmailAddress,
        password: &'a Password,
    },
}

#[derive(Debug)]
pub struct Account {
    config_dir: PathBuf,
    human_handle: HumanHandle,
    cmds: AuthenticatedAccountCmds,
    // Note the `Account` object is related to a single auth method, this means
    // we don't have to bother with vault rotation since, if it occurs, our auth
    // method will simply get invalidated.
    auth_method_id: AccountAuthMethodID,
    auth_method_secret_key: SecretKey,
}

impl Account {
    /// Helper for `create_auth_method` tests: we want to make sure all auth method
    /// share the same vault key. Without this method this would mean uploading
    /// something to the vault, then try to get it back from a different auth method.
    #[cfg(test)]
    async fn test_get_vault_key(&self) -> SecretKey {
        let (vault_key, _) = fetch_vault_items(self).await.unwrap();
        vault_key
    }

    pub fn auth_method_id(&self) -> AccountAuthMethodID {
        self.auth_method_id
    }

    pub fn server_addr(&self) -> &ParsecAddr {
        self.cmds.addr()
    }

    pub fn human_handle(&self) -> &HumanHandle {
        &self.human_handle
    }

    pub fn config_dir(&self) -> &Path {
        &self.config_dir
    }

    /// To create a new account, the user's email address must first be validated.
    /// This method ask the server to send an email containing a validation code.
    pub async fn create_1_send_validation_email(
        cmds: &AnonymousServerCmds,
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
        cmds: &AnonymousServerCmds,
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
        cmds: &AnonymousServerCmds,
        validation_code: ValidationCode,
        human_handle: HumanHandle,
        auth_method_strategy: AccountAuthMethodStrategy<'_>,
    ) -> Result<(), AccountCreateError> {
        account_create(
            cmds,
            AccountCreateStep::Proceed {
                validation_code,
                human_handle,
                auth_method_strategy,
            },
        )
        .await
    }

    /// Create a new authentication method for the account.
    ///
    /// Note that a new password auth method will replace the existing one
    /// (if any).
    /// On the contrary, there can be any number of master secret auth methods.
    pub async fn create_auth_method(
        &self,
        auth_method_strategy: AccountAuthMethodStrategy<'_>,
    ) -> Result<(), AccountCreateAuthMethodError> {
        account_create_auth_method(self, auth_method_strategy).await
    }

    /// List all authentication methods for the account.
    ///
    /// Note that only non-disabled auth methods are listed.
    /// Also, since this method request the server under the hood, there should
    /// always be at least one auth method (the one used to make the request!).
    pub async fn list_auth_methods(
        &self,
    ) -> Result<Vec<AuthMethodInfo>, AccountListAuthMethodsError> {
        account_list_auth_methods(self).await
    }

    /// Disable an authentication method for the account.
    ///
    /// Note that you cannot disable the authentication method used to make this request.
    pub async fn disable_auth_method(
        &self,
        auth_method_id: AccountAuthMethodID,
    ) -> Result<(), AccountDisableAuthMethodError> {
        account_disable_auth_method(self, auth_method_id).await
    }

    /// Convenient helper to avoid the first request (fetching the human handle)
    /// whenever we need an `Account` instance in the tests.
    #[cfg(test)]
    pub async fn test_new(
        config_dir: PathBuf,
        addr: ParsecAddr,
        auth_method_master_secret: &KeyDerivation,
        human_handle: HumanHandle,
    ) -> Self {
        account_login(
            config_dir,
            ProxyConfig::default(),
            addr,
            AccountLoginStrategy::MasterSecret(auth_method_master_secret),
            Some(human_handle),
        )
        .await
        .unwrap()
    }

    pub async fn login(
        config_dir: PathBuf,
        proxy: ProxyConfig,
        addr: ParsecAddr,
        login_strategy: AccountLoginStrategy<'_>,
    ) -> Result<Self, AccountLoginError> {
        account_login(config_dir, proxy, addr, login_strategy, None).await
    }

    /// Fetch from the server the pending invitations across all organizations
    /// corresponding to the account's email address.
    pub async fn list_invitations(
        &self,
    ) -> Result<Vec<ParsecInvitationAddr>, AccountListInvitationsError> {
        account_list_invitations(self).await
    }

    /// Fetch from the server all organizations the account's email is part of.
    /// This includes both valid and revoked organizations.
    pub async fn list_organizations(
        &self,
    ) -> Result<AccountOrganizations, AccountListOrganizationsError> {
        account_list_organizations(self).await
    }

    /// Fetch the account vault items from the server and return the opaque
    /// key matching the ID.
    ///
    /// This key is typically supposed to be used to decrypt the `ciphertext`
    /// field of a `DeviceFileAccountVault`.
    ///
    /// This method is only supposed to be used by `libparsec_platform_device_loader`
    /// through its `AccountVaultOperations` trait (that is itself implemented
    /// in `libparsec/src/device.rs`).
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
    ///
    /// While the key can be used for any purpose, the organization it is planned
    /// to be used with must be provided so that the organization's account vault
    /// strategy can be retrieved and enforced client-side (note it would be pointless
    /// to have this check done server-side since the uploaded data are encrypted).
    ///
    /// This method is only supposed to be used by `libparsec_platform_device_loader`
    /// through its `AccountVaultOperations` trait (that is itself implemented
    /// in `libparsec/src/device.rs`).
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
        cmds: &AnonymousServerCmds,
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
        cmds: &AnonymousServerCmds,
        validation_code: ValidationCode,
        email: EmailAddress,
        auth_method_strategy: AccountAuthMethodStrategy<'_>,
    ) -> Result<(), AccountRecoverProceedError> {
        account_recover_proceed(cmds, validation_code, email, auth_method_strategy).await
    }
}

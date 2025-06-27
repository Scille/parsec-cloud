// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashSet, path::PathBuf, sync::Arc};

use libparsec_client_connection::{AnonymousAccountCmds, AuthenticatedAccountCmds, ProxyConfig};
use libparsec_types::prelude::*;

mod account_create;
mod fetch_list_registration_devices;
mod list_invitations;
mod login;
mod register_new_device;

pub use account_create::*;
pub use fetch_list_registration_devices::*;
pub use list_invitations::*;
pub use login::*;
pub use register_new_device::*;

// The auth method master secret is the root secret from which are derived
// all other data used for authentication and end-2-end encryption:
// - ID & MAC key: used for server authentication.
// - Secret key: used for end-2-end encryption.
const AUTH_METHOD_ID_DERIVATION_UUID: uuid::Uuid =
    uuid::uuid!("00000000-0000-0000-0000-000000000000");
const AUTH_METHOD_MAC_KEY_DERIVATION_UUID: uuid::Uuid =
    uuid::uuid!("11111111-1111-1111-1111-111111111111");
const AUTH_METHOD_SECRET_KEY_DERIVATION_UUID: uuid::Uuid =
    uuid::uuid!("22222222-2222-2222-2222-222222222222");

#[derive(Debug)]
pub struct Account {
    config_dir: PathBuf,
    time_provider: TimeProvider,
    human_handle: HumanHandle,
    cmds: AuthenticatedAccountCmds,
    auth_method_secret_key: SecretKey,
    registration_devices_cache: Vec<Arc<LocalDevice>>,
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
        password: Password,
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
        password: Password,
    ) -> Result<Self, AccountLoginWithPasswordError> {
        account_login_with_password(config_dir, proxy, addr, email, password).await
    }

    pub async fn list_invitations(
        &self,
    ) -> Result<Vec<(OrganizationID, InvitationToken, InvitationType)>, AccountListInvitationsError>
    {
        account_list_invitations(self).await
    }

    pub async fn fetch_registration_devices(
        &mut self,
    ) -> Result<(), AccountFetchRegistrationDevicesError> {
        account_fetch_registration_devices(self).await
    }

    pub fn list_registration_devices(&self) -> HashSet<(OrganizationID, UserID)> {
        account_list_registration_devices(self)
    }

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
}

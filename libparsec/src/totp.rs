// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::AnonymousCmds;
use libparsec_types::prelude::*;

use crate::{
    handle::{borrow_from_handle, Handle, HandleItem},
    ClientConfig,
};

pub use libparsec_totp::{
    TOTPSetupStatus, TotpFetchOpaqueKeyError, TotpSetupConfirmAnonymousError,
    TotpSetupStatusAnonymousError,
};
use libparsec_totp::{
    TotpCreateOpaqueKeyError, TotpSetupConfirmAuthenticatedError, TotpSetupStatusAuthenticatedError,
};

pub type ClientTotpSetupStatusError = TotpSetupStatusAuthenticatedError;
pub async fn client_totp_setup_status(
    client: Handle,
) -> Result<TOTPSetupStatus, ClientTotpSetupStatusError> {
    let client = borrow_from_handle(client, |x| match x {
        HandleItem::Client { client, .. } => Some(client.clone()),
        _ => None,
    })?;

    libparsec_totp::totp_setup_status_authenticated(&client.cmds()).await
}

pub async fn totp_setup_status_anonymous(
    config: ClientConfig,
    addr: ParsecTOTPResetAddr,
) -> Result<TOTPSetupStatus, TotpSetupStatusAnonymousError> {
    let config: libparsec_client::ClientConfig = config.into();
    let user_id = addr.user_id();
    let token = addr.token();
    let cmds = AnonymousCmds::new(&config.config_dir, addr.into(), config.proxy.clone())
        .map_err(|e| anyhow::anyhow!("Error while configuring connection to server: {e}"))?;

    libparsec_totp::totp_setup_status_anonymous(&cmds, user_id, token).await
}

pub type ClientTOTPSetupConfirmError = TotpSetupConfirmAuthenticatedError;
pub async fn client_totp_setup_confirm(
    client: Handle,
    one_time_password: String,
) -> Result<(), ClientTOTPSetupConfirmError> {
    let client = borrow_from_handle(client, |x| match x {
        HandleItem::Client { client, .. } => Some(client.clone()),
        _ => None,
    })?;

    libparsec_totp::totp_setup_confirm_authenticated(&client.cmds(), one_time_password).await
}

pub async fn totp_setup_confirm_anonymous(
    config: ClientConfig,
    addr: ParsecTOTPResetAddr,
    one_time_password: String,
) -> Result<(), TotpSetupConfirmAnonymousError> {
    let config: libparsec_client::ClientConfig = config.into();
    let user_id = addr.user_id();
    let token = addr.token();
    let cmds = AnonymousCmds::new(&config.config_dir, addr.into(), config.proxy.clone())
        .map_err(|e| anyhow::anyhow!("Error while configuring connection to server: {e}"))?;

    libparsec_totp::totp_setup_confirm_anonymous(&cmds, user_id, token, one_time_password).await
}

pub type ClientTotpCreateOpaqueKeyError = TotpCreateOpaqueKeyError;
pub async fn client_totp_create_opaque_key(
    client: Handle,
) -> Result<(TOTPOpaqueKeyID, SecretKey), ClientTotpCreateOpaqueKeyError> {
    let client = borrow_from_handle(client, |x| match x {
        HandleItem::Client { client, .. } => Some(client.clone()),
        _ => None,
    })?;

    libparsec_totp::totp_create_opaque_key(&client.cmds()).await
}

pub async fn totp_fetch_opaque_key(
    config: ClientConfig,
    // Don't use a `ParsecOrganizationAddr` here since the parameter given to this
    // function are expected to be obtain from a `AvailableDevice`.
    server_addr: ParsecAddr,
    organization_id: OrganizationID,
    user_id: UserID,
    opaque_key_id: TOTPOpaqueKeyID,
    one_time_password: String,
) -> Result<SecretKey, TotpFetchOpaqueKeyError> {
    let config: libparsec_client::ClientConfig = config.into();
    let addr = ParsecAnonymousAddr::new(server_addr, organization_id);
    let cmds = AnonymousCmds::new(&config.config_dir, addr, config.proxy.clone())
        .map_err(|e| anyhow::anyhow!("Error while configuring connection to server: {e}"))?;

    libparsec_totp::totp_fetch_opaque_key(&cmds, user_id, opaque_key_id, one_time_password).await
}

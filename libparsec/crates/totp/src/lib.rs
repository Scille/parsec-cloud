// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{AnonymousCmds, AuthenticatedCmds, ConnectionError};
use libparsec_protocol::{anonymous_cmds, authenticated_cmds};
use libparsec_types::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub enum TOTPSetupStatus {
    /// TOTP setup is not yet set up: the TOTP secret should be displayed so that
    /// the end-user's authenticator app can be configured.
    Unconfirmed { base32_totp_secret: String },
    /// TOTP setup is done (i.e. the server had the confirmation the end-user
    /// is able to generate one-time passwords).
    /// Now the TOTP secret can no longer be obtained (unless a TOTP setup reset
    /// is issued by an administrator), and the opaque keys stored server-side
    /// can be obtained after a one-time password challenge.
    Confirmed,
}

#[derive(Debug, thiserror::Error)]
pub enum TotpSetupStatusAuthenticatedError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn totp_setup_status_authenticated(
    cmds: &AuthenticatedCmds,
) -> Result<TOTPSetupStatus, TotpSetupStatusAuthenticatedError> {
    let req = authenticated_cmds::latest::totp_setup_get_secret::Req;
    let rep = cmds.send(req).await?;
    match rep {
        authenticated_cmds::latest::totp_setup_get_secret::Rep::Ok { totp_secret } => {
            let base32_totp_secret = data_encoding::BASE32_NOPAD.encode(&totp_secret);
            Ok(TOTPSetupStatus::Unconfirmed { base32_totp_secret })
        }
        authenticated_cmds::latest::totp_setup_get_secret::Rep::AlreadySetup => {
            Ok(TOTPSetupStatus::Confirmed)
        }
        authenticated_cmds::latest::totp_setup_get_secret::Rep::UnknownStatus {
            unknown_status,
            ..
        } => Err(TotpSetupStatusAuthenticatedError::Internal(
            anyhow::anyhow!("Unknown error status `{}` from server", unknown_status),
        )),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TotpSetupStatusAnonymousError {
    #[error("The authentication token is invalid")]
    BadToken,
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn totp_setup_status_anonymous(
    cmds: &AnonymousCmds,
    user_id: UserID,
    token: AccessToken,
) -> Result<TOTPSetupStatus, TotpSetupStatusAnonymousError> {
    let req = anonymous_cmds::latest::totp_setup_get_secret::Req { user_id, token };
    let rep = cmds.send(req).await?;
    match rep {
        anonymous_cmds::latest::totp_setup_get_secret::Rep::Ok { totp_secret } => {
            let base32_totp_secret = data_encoding::BASE32_NOPAD.encode(&totp_secret);
            Ok(TOTPSetupStatus::Unconfirmed { base32_totp_secret })
        }
        anonymous_cmds::latest::totp_setup_get_secret::Rep::BadToken => {
            Err(TotpSetupStatusAnonymousError::BadToken)
        }
        anonymous_cmds::latest::totp_setup_get_secret::Rep::UnknownStatus {
            unknown_status,
            ..
        } => Err(TotpSetupStatusAnonymousError::Internal(anyhow::anyhow!(
            "Unknown error status `{}` from server",
            unknown_status
        ))),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TotpSetupConfirmAuthenticatedError {
    #[error("The one-time-password is invalid")]
    InvalidOneTimePassword,
    #[error("The TOTP has already been setup")]
    AlreadySetup,
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn totp_setup_confirm_authenticated(
    cmds: &AuthenticatedCmds,
    one_time_password: String,
) -> Result<(), TotpSetupConfirmAuthenticatedError> {
    let req = authenticated_cmds::latest::totp_setup_confirm::Req { one_time_password };
    let rep = cmds.send(req).await?;
    match rep {
        authenticated_cmds::latest::totp_setup_confirm::Rep::Ok => Ok(()),
        authenticated_cmds::latest::totp_setup_confirm::Rep::InvalidOneTimePassword => {
            Err(TotpSetupConfirmAuthenticatedError::InvalidOneTimePassword)
        }
        authenticated_cmds::latest::totp_setup_confirm::Rep::AlreadySetup => {
            Err(TotpSetupConfirmAuthenticatedError::AlreadySetup)
        }
        authenticated_cmds::latest::totp_setup_confirm::Rep::UnknownStatus {
            unknown_status,
            ..
        } => Err(TotpSetupConfirmAuthenticatedError::Internal(
            anyhow::anyhow!("Unknown error status `{}` from server", unknown_status),
        )),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TotpSetupConfirmAnonymousError {
    #[error("The one-time-password is invalid")]
    InvalidOneTimePassword,
    #[error("The authentication token is invalid")]
    BadToken,
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn totp_setup_confirm_anonymous(
    cmds: &AnonymousCmds,
    user_id: UserID,
    token: AccessToken,
    one_time_password: String,
) -> Result<(), TotpSetupConfirmAnonymousError> {
    let req = anonymous_cmds::latest::totp_setup_confirm::Req {
        user_id,
        token,
        one_time_password,
    };
    let rep = cmds.send(req).await?;
    match rep {
        anonymous_cmds::latest::totp_setup_confirm::Rep::Ok => Ok(()),
        anonymous_cmds::latest::totp_setup_confirm::Rep::InvalidOneTimePassword => {
            Err(TotpSetupConfirmAnonymousError::InvalidOneTimePassword)
        }
        anonymous_cmds::latest::totp_setup_confirm::Rep::BadToken => {
            Err(TotpSetupConfirmAnonymousError::BadToken)
        }
        anonymous_cmds::latest::totp_setup_confirm::Rep::UnknownStatus {
            unknown_status, ..
        } => Err(TotpSetupConfirmAnonymousError::Internal(anyhow::anyhow!(
            "Unknown error status `{}` from server",
            unknown_status
        ))),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TotpCreateOpaqueKeyError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn totp_create_opaque_key(
    cmds: &AuthenticatedCmds,
) -> Result<(TOTPOpaqueKeyID, SecretKey), TotpCreateOpaqueKeyError> {
    let req = authenticated_cmds::latest::totp_create_opaque_key::Req;
    let rep = cmds.send(req).await?;
    match rep {
        authenticated_cmds::latest::totp_create_opaque_key::Rep::Ok {
            opaque_key_id,
            opaque_key,
        } => Ok((opaque_key_id, opaque_key)),
        authenticated_cmds::latest::totp_create_opaque_key::Rep::UnknownStatus {
            unknown_status,
            ..
        } => Err(TotpCreateOpaqueKeyError::Internal(anyhow::anyhow!(
            "Unknown error status `{}` from server",
            unknown_status
        ))),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TotpFetchOpaqueKeyError {
    #[error("The one-time password is invalid")]
    InvalidOneTimePassword,
    #[error("Too many attempts, must wait until {wait_until}")]
    Throttled { wait_until: DateTime },
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn totp_fetch_opaque_key(
    cmds: &AnonymousCmds,
    user_id: UserID,
    opaque_key_id: TOTPOpaqueKeyID,
    one_time_password: String,
) -> Result<SecretKey, TotpFetchOpaqueKeyError> {
    let req = anonymous_cmds::latest::totp_fetch_opaque_key::Req {
        user_id,
        opaque_key_id,
        one_time_password,
    };
    let rep = cmds.send(req).await?;
    match rep {
        anonymous_cmds::latest::totp_fetch_opaque_key::Rep::Ok { opaque_key } => Ok(opaque_key),
        anonymous_cmds::latest::totp_fetch_opaque_key::Rep::InvalidOneTimePassword => {
            Err(TotpFetchOpaqueKeyError::InvalidOneTimePassword)
        }
        anonymous_cmds::latest::totp_fetch_opaque_key::Rep::Throttled { wait_until } => {
            Err(TotpFetchOpaqueKeyError::Throttled { wait_until })
        }
        anonymous_cmds::latest::totp_fetch_opaque_key::Rep::UnknownStatus {
            unknown_status,
            ..
        } => Err(TotpFetchOpaqueKeyError::Internal(anyhow::anyhow!(
            "Unknown error status `{}` from server",
            unknown_status
        ))),
    }
}

#[cfg(test)]
#[path = "../tests/unit/mod.rs"]
#[allow(clippy::unwrap_used)]
mod tests;

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{AnonymousServerCmds, ConnectionError};
use libparsec_types::prelude::*;

#[derive(Debug, thiserror::Error)]
pub(super) enum RetrieveAuthMethodMasterSecretFromPasswordError {
    #[error("Server provided an invalid password algorithm config: {0}")]
    BadPasswordAlgorithm(CryptoError),
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(super) async fn retrieve_auth_method_master_secret_from_password(
    cmds: &AnonymousServerCmds,
    email: &EmailAddress,
    password: &Password,
) -> Result<KeyDerivation, RetrieveAuthMethodMasterSecretFromPasswordError> {
    // The password algorithm configuration is obtained from the server
    // to know how to turn the password into `auth_method_master_secret`.

    let untrusted_password_algorithm = {
        use libparsec_protocol::anonymous_server_cmds::latest::auth_method_password_get_algorithm::{Rep, Req};

        let req = Req {
            email: email.clone(),
        };
        let rep = cmds.send(req).await?;

        match rep {
            Rep::Ok { password_algorithm } => password_algorithm,
            bad_rep @ Rep::UnknownStatus { .. } => {
                return Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into());
            }
        }
    };

    let password_algorithm = untrusted_password_algorithm
        .validate(&email.to_string())
        .map_err(RetrieveAuthMethodMasterSecretFromPasswordError::BadPasswordAlgorithm)?;

    let auth_method_master_secret = password_algorithm
        .compute_key_derivation(password)
        .map_err(RetrieveAuthMethodMasterSecretFromPasswordError::BadPasswordAlgorithm)?;

    Ok(auth_method_master_secret)
}

pub(super) struct AuthMethodDerivedKeys {
    pub id: AccountAuthMethodID,
    pub mac_key: SecretKey,
    pub secret_key: SecretKey,
}

pub(super) fn derive_auth_method_keys(master_secret: &KeyDerivation) -> AuthMethodDerivedKeys {
    // Derive from the master secret the data used to authenticate with the server.
    let id = AccountAuthMethodID::from(
        master_secret.derive_uuid_from_uuid(AUTH_METHOD_ID_DERIVATION_UUID),
    );
    let mac_key = master_secret.derive_secret_key_from_uuid(AUTH_METHOD_MAC_KEY_DERIVATION_UUID);
    let secret_key =
        master_secret.derive_secret_key_from_uuid(AUTH_METHOD_SECRET_KEY_DERIVATION_UUID);

    AuthMethodDerivedKeys {
        id,
        mac_key,
        secret_key,
    }
}

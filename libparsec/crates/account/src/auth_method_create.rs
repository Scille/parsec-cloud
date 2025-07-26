// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use crate::{derive_auth_method_keys, Account, AccountAuthMethodStrategy};

#[derive(Debug, thiserror::Error)]
pub enum AccountAuthMethodCreateError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(super) async fn account_auth_method_create(
    account: &Account,
    auth_method_strategy: AccountAuthMethodStrategy<'_>,
) -> Result<(), AccountAuthMethodCreateError> {
    let (auth_method_password_algorithm, auth_method_keys) = match auth_method_strategy {
        AccountAuthMethodStrategy::MasterSecret(auth_method_master_secret) => {
            let auth_method_keys = derive_auth_method_keys(auth_method_master_secret);
            (None, auth_method_keys)
        }

        AccountAuthMethodStrategy::Password(password) => {
            let auth_method_password_algorithm = PasswordAlgorithm::generate_argon2id(
                PasswordAlgorithmSaltStrategy::DerivedFromEmail {
                    email: account.human_handle.email().as_ref(),
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
    };

    let vault_key = SecretKey::generate();
    let vault_key_access = AccountVaultKeyAccess { vault_key };
    let vault_key_access_bytes = vault_key_access.dump_and_encrypt(&auth_method_keys.secret_key);

    use libparsec_protocol::authenticated_account_cmds::latest::auth_method_create::{Rep, Req};

    let req = Req {
        auth_method_password_algorithm,
        auth_method_mac_key: auth_method_keys.mac_key,
        auth_method_id: auth_method_keys.id,
        vault_key_access: vault_key_access_bytes.into(),
    };
    let rep = account.cmds.send(req).await?;

    match rep {
        Rep::Ok => Ok(()),
        bad_rep @ (
            // Auth method ID is an UUID derived from the master secret, so it should
            // statically never collide with an existing one.
            Rep::AuthMethodIdAlreadyExists
            | Rep::UnknownStatus { .. }
        ) => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }
}

#[cfg(test)]
#[path = "../tests/unit/auth_method_create.rs"]
mod tests;

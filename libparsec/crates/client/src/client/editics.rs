// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_protocol::authenticated_cmds::latest::editics_join_session::{Rep, Req};
use libparsec_types::anyhow;
use libparsec_types::uuid::Uuid;
use libparsec_types::Bytes;
use libparsec_types::VlobID;

use crate::EncrytionUsage;

#[derive(Debug, thiserror::Error)]
pub enum ClientEditicsGetSessionKeyError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
}

pub async fn join_session(
    client: &super::Client,
    workspace_id: VlobID,
    file_id: VlobID,
) -> Result<Bytes, ClientEditicsGetSessionKeyError> {
    let session_key = Uuid::new_v4();

    let (encrypted_session_key, key_index) = client
        .certificates_ops
        .encrypt_for_realm(
            EncrytionUsage::File(file_id),
            workspace_id,
            session_key.as_bytes(),
        )
        .await
        .map_err(|e| ClientEditicsGetSessionKeyError::Internal(e.into()))?;

    let request = Req {
        encrypted_session_key: Bytes::from(encrypted_session_key.clone()),
        key_index,
        file_id,
        workspace_id,
    };
    let rep = client.cmds.send(request).await?;
    let (encrypted, key_index) = match rep {
        Rep::Ok {
            encrypted_session_key,
            key_index,
        } => (encrypted_session_key, key_index),
        _ => {
            todo!()
        }
    };

    // decrypt session key

    let decrypted: Vec<_> = if encrypted == encrypted_session_key {
        session_key.as_bytes().into()
    } else {
        client
            .certificates_ops
            .decrypt_opaque_data_for_realm(
                EncrytionUsage::File(file_id),
                workspace_id,
                key_index,
                &encrypted,
            )
            .await
            .map_err(|e| ClientEditicsGetSessionKeyError::Internal(e.into()))?
    };

    Ok(Bytes::from(decrypted))
}

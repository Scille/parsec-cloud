// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_protocol::authenticated_cmds::latest::editics_join_session::{Rep, Req};
use libparsec_types::anyhow;
use libparsec_types::uuid::Uuid;
use libparsec_types::Bytes;
use libparsec_types::VlobID;

#[derive(Debug, thiserror::Error)]
pub enum ClientEditicsJoinSessionError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
}

pub async fn join_session(
    client: &super::Client,
    workspace_id: VlobID,
    file_id: VlobID,
) -> Result<Bytes, ClientEditicsJoinSessionError> {
    let session_key = Uuid::new_v4();
    // TODO encrypt with workspace key
    let encrypted_session_key = Bytes::from(session_key.to_string());

    let request = Req {
        encrypted_session_key,
        file_id,
        workspace_id,
    };
    let rep = client.cmds.send(request).await?;
    match rep {
        Rep::Ok {
            encrypted_session_key,
        } => Ok(encrypted_session_key),
        _ => {
            todo!()
        }
    }
}

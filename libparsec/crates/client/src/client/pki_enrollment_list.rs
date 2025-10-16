// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

use libparsec_client_connection::{
    protocol::authenticated_cmds, AuthenticatedCmds, ConnectionError,
};

/*
 * list_enrollments
 */

#[derive(Debug, thiserror::Error)]
pub enum PkiEnrollmentListError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Not allowed to list enrollments")]
    AuthorNotAllowed,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub use authenticated_cmds::latest::pki_enrollment_list::PkiEnrollmentListItem;

pub async fn list_enrollments(
    cmds: &AuthenticatedCmds,
) -> Result<Vec<PkiEnrollmentListItem>, PkiEnrollmentListError> {
    use authenticated_cmds::latest::pki_enrollment_list::{Rep, Req};

    let rep = cmds.send(Req).await?;

    match rep {
        Rep::Ok { enrollments } => Ok(enrollments),
        Rep::AuthorNotAllowed => Err(PkiEnrollmentListError::AuthorNotAllowed),
        rep @ Rep::UnknownStatus { .. } => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", rep).into())
        }
    }
}

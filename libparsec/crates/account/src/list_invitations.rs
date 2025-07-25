// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use super::Account;

#[derive(Debug, thiserror::Error)]
pub enum AccountListInvitationsError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(super) async fn account_list_invitations(
    account: &Account,
) -> Result<Vec<ParsecInvitationAddr>, AccountListInvitationsError> {
    use libparsec_protocol::authenticated_account_cmds::latest::invite_self_list::{Rep, Req};

    let req = Req;
    let rep = account.cmds.send(req).await?;

    let invitations = match rep {
        Rep::Ok { invitations } => invitations,
        bad_rep @ Rep::UnknownStatus { .. } => {
            return Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into());
        }
    };

    Ok(invitations
        .into_iter()
        .map(|(organization_id, token, invitation_type)| {
            ParsecInvitationAddr::new(
                account.cmds.addr().to_owned(),
                organization_id,
                invitation_type,
                token,
            )
        })
        .collect())
}

#[cfg(test)]
#[path = "../tests/unit/list_invitations.rs"]
mod tests;

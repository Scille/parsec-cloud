// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use super::Account;

#[derive(Debug, thiserror::Error)]
pub enum AccountListOrganizationsError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub type AccountOrganizationsActiveUser =
    libparsec_protocol::authenticated_account_cmds::latest::organization_self_list::ActiveUser;
pub type AccountOrganizationsRevokedUser =
    libparsec_protocol::authenticated_account_cmds::latest::organization_self_list::RevokedUser;
pub type AccountOrganizationsOrganizationConfig = libparsec_protocol::authenticated_account_cmds::latest::organization_self_list::OrganizationConfig;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountOrganizations {
    pub active: Vec<AccountOrganizationsActiveUser>,
    pub revoked: Vec<AccountOrganizationsRevokedUser>,
}

pub(super) async fn account_list_organizations(
    account: &Account,
) -> Result<AccountOrganizations, AccountListOrganizationsError> {
    use libparsec_protocol::authenticated_account_cmds::latest::organization_self_list::{
        Rep, Req,
    };

    let req = Req;
    let rep = account.cmds.send(req).await?;

    let (active, revoked) = match rep {
        Rep::Ok { active, revoked } => (active, revoked),
        bad_rep @ Rep::UnknownStatus { .. } => {
            return Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into());
        }
    };

    Ok(AccountOrganizations { active, revoked })
}

#[cfg(test)]
#[path = "../tests/unit/list_organizations.rs"]
mod tests;

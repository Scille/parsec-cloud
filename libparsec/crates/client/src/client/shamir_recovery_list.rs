// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

pub use crate::certif::{
    CertifListShamirRecoveryError as ClientListShamirRecoveryError, OtherShamirRecoveryInfo,
    SelfShamirRecoveryInfo,
};
use libparsec_types::prelude::*;

use super::Client;

pub async fn get_self_shamir_recovery(
    client: &Client,
) -> Result<SelfShamirRecoveryInfo, ClientListShamirRecoveryError> {
    client.certificates_ops.get_self_shamir_recovery().await
}

pub async fn list_shamir_recoveries_for_others(
    client: &Client,
) -> Result<Vec<OtherShamirRecoveryInfo>, ClientListShamirRecoveryError> {
    client
        .certificates_ops
        .list_shamir_recoveries_for_others()
        .await
}

pub async fn get_shamir_recovery_share_data(
    client: &Client,
    user_id: UserID,
) -> Result<ShamirRecoveryShareData, ClientListShamirRecoveryError> {
    client
        .certificates_ops
        .get_shamir_recovery_share_data(user_id)
        .await
}

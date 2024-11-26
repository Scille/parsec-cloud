// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

pub use crate::certif::{
    CertifListShamirRecoveryError as ClientListShamirRecoveryError, OtherShamirRecoveryInfo,
    SelfShamirRecoveryInfo,
};

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

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

pub use crate::certif::{
    CertifGetSelfShamirRecoveryError as ClientGetSelfShamirRecoveryError,
    CertifListShamirRecoveriesForOthersError as ClientListShamirRecoveriesForOthersError,
    OtherShamirRecoveryInfo, SelfShamirRecoveryInfo,
};

use super::Client;

pub async fn get_self_shamir_recovery(
    client: &Client,
) -> Result<SelfShamirRecoveryInfo, ClientGetSelfShamirRecoveryError> {
    client.certificates_ops.get_self_shamir_recovery().await
}

pub async fn list_shamir_recoveries_for_others(
    client: &Client,
) -> Result<Vec<OtherShamirRecoveryInfo>, ClientListShamirRecoveriesForOthersError> {
    client
        .certificates_ops
        .list_shamir_recoveries_for_others()
        .await
}

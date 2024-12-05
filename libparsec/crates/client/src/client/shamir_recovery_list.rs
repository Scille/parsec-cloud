// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

pub use crate::certif::{
    CertifGetSelfShamirRecoveryError as ClientGetSelfShamirRecoveryError,
    CertifGetShamirRecoveryShareDataError as ClientGetShamirRecoveryShareDataError,
    CertifListShamirRecoveriesForOthersError as ClientListShamirRecoveriesForOthersError,
    OtherShamirRecoveryInfo, SelfShamirRecoveryInfo,
};
use libparsec_types::prelude::*;

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

pub async fn get_shamir_recovery_share_data(
    client: &Client,
    user_id: UserID,
) -> Result<ShamirRecoveryShareData, ClientGetShamirRecoveryShareDataError> {
    client
        .certificates_ops
        .get_shamir_recovery_share_data(user_id)
        .await
}

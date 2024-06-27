// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::HashMap;

pub use crate::certif::CertifShamirError as ClientShamirError;
use libparsec_types::UserID;

use super::Client;

pub async fn shamir_setup_create(
    client_ops: &Client,
    share_recipients: HashMap<UserID, u8>,
    threshold: u8,
) -> Result<(), ClientShamirError> {
    let outcome = client_ops
        .certificates_ops
        .shamir_setup_create(
            client_ops.device_id(),
            client_ops.user_id(),
            share_recipients,
            threshold,
        )
        .await?;

    todo!()
}

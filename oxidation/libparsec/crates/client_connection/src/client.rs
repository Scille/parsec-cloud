// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use libparsec_crypto::SigningKey;
use libparsec_types::{BackendOrganizationAddr, DeviceID};

use crate::AuthenticatedCmds;

// TODO: This is just for poc purpose and will change later on
pub fn generate_client(
    signing_key: SigningKey,
    device_id: DeviceID,
    root_url: BackendOrganizationAddr,
) -> AuthenticatedCmds {
    let client = reqwest::ClientBuilder::new()
        .build()
        .expect("cannot build client");
    AuthenticatedCmds::new(client, root_url, device_id, signing_key)
        .expect("failed to build auth cmds client")
}

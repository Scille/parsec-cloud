// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use libparsec_crypto::SigningKey;
use libparsec_types::{
    BackendAnonymousAddr, BackendInvitationAddr, BackendOrganizationAddr, DeviceID,
};

use crate::{AnonymousCmds, AuthenticatedCmds, InvitedCmds};

// TODO: This is just for poc purpose and will change later on
pub fn generate_authenticated_client(
    signing_key: SigningKey,
    device_id: DeviceID,
    url: BackendOrganizationAddr,
) -> AuthenticatedCmds {
    let client = reqwest::ClientBuilder::new()
        .build()
        .expect("Cannot build client");
    AuthenticatedCmds::new(client, url, device_id, signing_key)
        .expect("Failed to build Authenticated client")
}

pub fn generate_invited_client(url: BackendInvitationAddr) -> InvitedCmds {
    let client = reqwest::ClientBuilder::new()
        .build()
        .expect("Cannot build client");
    InvitedCmds::new(client, url).expect("Failed to build InvitedCmds client")
}

pub fn generate_anonymous_client(url: BackendAnonymousAddr) -> AnonymousCmds {
    let client = reqwest::ClientBuilder::new()
        .build()
        .expect("Cannot build client");
    AnonymousCmds::new(client, url).expect("Failed to build AnonymousCmds client")
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use libparsec_crypto::SigningKey;
use libparsec_platform_http_proxy::ProxyConfig;
use libparsec_types::{
    BackendAnonymousAddr, BackendInvitationAddr, BackendOrganizationAddr, DeviceID,
};

use crate::{AnonymousCmds, AuthenticatedCmds, InvitedCmds};

// TODO: This is just for poc purpose and will change later on
pub fn generate_authenticated_client(
    signing_key: SigningKey,
    device_id: DeviceID,
    url: BackendOrganizationAddr,
    config: ProxyConfig,
) -> reqwest::Result<AuthenticatedCmds> {
    build_http_client(config)
        .map(|client| AuthenticatedCmds::new(client, url, device_id, signing_key))
}

pub fn generate_invited_client(
    url: BackendInvitationAddr,
    config: ProxyConfig,
) -> reqwest::Result<InvitedCmds> {
    build_http_client(config).map(|client| InvitedCmds::new(client, url))
}

pub fn generate_anonymous_client(
    url: BackendAnonymousAddr,
    config: ProxyConfig,
) -> reqwest::Result<AnonymousCmds> {
    build_http_client(config).map(|client| AnonymousCmds::new(client, url))
}

fn build_http_client(config: ProxyConfig) -> reqwest::Result<reqwest::Client> {
    let builder = reqwest::ClientBuilder::default();

    let builder = config.configure_http_client(builder)?;

    builder.build()
}

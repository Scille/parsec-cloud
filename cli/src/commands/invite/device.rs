// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use anyhow::Context;
use libparsec::{InvitationType, ParsecInvitationAddr};

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {}
);

crate::build_main_with_client!(main, invite_device);

pub async fn invite_device(_args: Args, client: &StartedClient) -> anyhow::Result<()> {
    log::trace!("Inviting a device");

    let mut handle = start_spinner("Creating device invitation".into());

    let (token, _email_sent_status) = client
        .new_device_invitation(false)
        .await
        .context("Server refused to create device invitation")?;

    let url = ParsecInvitationAddr::new(
        client.organization_addr(),
        client.organization_id().clone(),
        InvitationType::Device,
        token,
    )
    .to_url();

    handle.stop_with_message(format!(
        "Invitation token: {YELLOW}{token}{RESET}\nInvitation URL: {YELLOW}{url}{RESET}"
    ));

    Ok(())
}

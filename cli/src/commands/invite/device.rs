// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
use std::fmt::Write;

use anyhow::Context;
use libparsec::{InvitationType, ParsecInvitationAddr};

use crate::{ui::compat::InvitationLink, utils::StartedClient};

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {}
);

crate::build_main_with_client!(main, invite_device);

pub async fn invite_device(
    ui: crate::Ui,
    _args: Args,
    client: &StartedClient,
) -> anyhow::Result<()> {
    log::trace!("Inviting a device");

    let handle = ui.with_spinner(|_, out| write!(out, "Creating device invitation"))?;

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
    .to_http_redirection_url();

    handle.stop();

    ui.data_print(&InvitationLink { token, url })?;

    Ok(())
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
use std::fmt::Write;

use crate::{ui::compat::InviteItemDisplay, utils::*};

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {}
);

crate::build_main_with_client!(main, list_invite);

pub async fn list_invite(ui: crate::Ui, _args: Args, client: &StartedClient) -> anyhow::Result<()> {
    log::trace!("Listing invitations");
    poll_server_for_new_certificates(&ui, client).await?;

    let handle = ui.with_spinner(|_, out| write!(out, "Listing invitations"))?;

    let users = client.list_users(false, None, None).await?;

    let invitations = client
        .list_invitations()
        .await?
        .into_iter()
        .map(|item| InviteItemDisplay(item, &users))
        .collect::<Vec<_>>();

    if invitations.is_empty() {
        handle.stop_with(|_, out| write!(out, "No invitation."))?;
    } else {
        handle.stop_with(|_, out| write!(out, "{} invitations found.", invitations.len()))?;
        ui.data_print(&invitations.as_slice())?;
    }

    Ok(())
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
use std::fmt::Write;

use libparsec::{RealmRole, UserID};

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, workspace, password_stdin]
    pub struct Args {
        /// The user ID to share the workspace with
        #[arg(short, long, value_parser = UserID::from_hex, value_hint = clap::ValueHint::Other)]
        user: UserID,
        /// Role (owner/manager/contributor/reader)
        #[arg(short, long, value_hint = clap::ValueHint::Other)]
        role: RealmRole,
    }
);

crate::build_main_with_client!(main, share_workspace);

pub async fn share_workspace(
    ui: crate::Ui,
    args: Args,
    client: &StartedClient,
) -> anyhow::Result<()> {
    let Args {
        workspace: wid,
        user,
        role,
        ..
    } = args;
    log::trace!("Sharing workspace {wid} to {user} with role {role}");

    let handle = ui.with_spinner(|_, out| write!(out, "Sharing workspace"))?;

    client.share_workspace(wid, user, Some(role)).await?;

    handle.stop_with(|_, out| write!(out, "Workspace has been shared"))?;

    client.stop().await;

    Ok(())
}

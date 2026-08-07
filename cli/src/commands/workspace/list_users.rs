// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
use std::io::Write;

use crate::{
    ui::{compat::WorkspaceUserAccessInfoDisplay, Color},
    utils::*,
};

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, workspace, password_stdin]
    pub struct Args {
    }
);

crate::build_main_with_client!(main, list_users_workspace);

pub async fn list_users_workspace(
    ui: crate::Ui,
    args: Args,
    client: &StartedClient,
) -> anyhow::Result<()> {
    let Args { workspace: wid, .. } = args;

    log::trace!("Listing users in the workspace");

    poll_server_for_new_certificates(&ui, client).await?;
    client.refresh_workspaces_list().await?;
    let users = client
        .list_workspace_users(wid)
        .await?
        .into_iter()
        .map(WorkspaceUserAccessInfoDisplay)
        .collect::<Vec<_>>();

    if users.is_empty() {
        ui.with_message(|_, out| writeln!(out, "No user has access to that workspace"))?;
    } else {
        ui.with_message(|fmt, out| {
            writeln!(
                out,
                "Workspace {wid} is shared with {count} user(s)",
                count = fmt.wrap_in_color(Color::Green, users.len())
            )
        })?;

        ui.data_print(&users.as_slice())?;
    }

    Ok(())
}

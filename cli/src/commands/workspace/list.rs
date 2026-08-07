// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
use std::io::Write;

use crate::{
    ui::{compat::WorkspaceInfoDisplay, Color},
    utils::*,
};

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {}
);

crate::build_main_with_client!(main, list_workspace);

pub async fn list_workspace(
    ui: crate::Ui,
    _args: Args,
    client: &StartedClient,
) -> anyhow::Result<()> {
    log::trace!("Listing workspaces");

    poll_server_for_new_certificates(&ui, client).await?;
    client.refresh_workspaces_list().await?;
    let workspaces = client
        .list_workspaces()
        .await
        .into_iter()
        .map(WorkspaceInfoDisplay)
        .collect::<Vec<_>>();

    if workspaces.is_empty() {
        println!("No workspaces found");
    } else {
        ui.with_message(|fmt, out| {
            writeln!(
                out,
                "Found {count} workspace(s)",
                count = fmt.wrap_in_color(Color::Green, workspaces.len())
            )
        })?;

        ui.data_print(&workspaces.as_slice())?;
    }

    Ok(())
}

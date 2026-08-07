// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
use std::fmt::Write;

use libparsec::EntryName;

use crate::{ui::Color, utils::*};

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {
        /// New workspace name
        #[arg(value_hint = clap::ValueHint::Other)]
        name: EntryName,
    }
);

crate::build_main_with_client!(main, create_workspace);

pub async fn create_workspace(
    ui: crate::Ui,
    args: Args,
    client: &StartedClient,
) -> anyhow::Result<()> {
    let Args { name, .. } = args;
    log::trace!("Creating workspace {name}");

    let handle = ui.with_spinner(|_, out| write!(out, "Creating workspace"))?;

    let id = client.create_workspace(name).await?.simple();
    client.ensure_workspaces_bootstrapped().await?;

    handle.stop_with(|fmt, out| {
        write!(
            out,
            "Workspace has been created with id: {id}",
            id = fmt.wrap_in_color(Color::Yellow, id)
        )
    })?;

    client.stop().await;

    Ok(())
}

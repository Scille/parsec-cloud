// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec::EntryName;

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = client_opts]
    pub struct Args {
        /// New workspace name
        name: EntryName,
    }
);

crate::build_main_with_client!(main, create_workspace);

pub async fn create_workspace(args: Args, client: &StartedClient) -> anyhow::Result<()> {
    let Args { name, .. } = args;
    log::trace!("Creating workspace {name}");

    let mut handle = start_spinner("Creating workspace".into());

    let id = client.create_workspace(name).await?.simple();
    client.ensure_workspaces_bootstrapped().await?;

    handle.stop_with_message(format!(
        "Workspace has been created with id: {YELLOW}{id}{RESET}"
    ));

    client.stop().await;

    Ok(())
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec::EntryName;

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {
        /// New workspace name
        name: EntryName,
    }
);

pub async fn main(args: Args) -> anyhow::Result<()> {
    let Args {
        name,
        device,
        config_dir,
        password_stdin,
    } = args;
    log::trace!(
        "Creating workspace {name} (confdir={}, device={})",
        config_dir.display(),
        device.as_deref().unwrap_or("N/A")
    );

    let client = load_client(&config_dir, device, password_stdin).await?;
    let mut handle = start_spinner("Creating workspace".into());

    let id = client.create_workspace(name).await?.simple();
    client.ensure_workspaces_bootstrapped().await?;

    handle.stop_with_message(format!(
        "Workspace has been created with id: {YELLOW}{id}{RESET}"
    ));

    client.stop().await;

    Ok(())
}

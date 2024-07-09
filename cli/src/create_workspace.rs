// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec::EntryName;

use crate::utils::*;

#[derive(clap::Args)]
pub struct CreateWorkspace {
    #[clap(flatten)]
    config: ConfigWithDeviceSharedOpts,
    /// New workspace name
    #[arg(short, long)]
    name: EntryName,
}

pub async fn create_workspace(create_workspace: CreateWorkspace) -> anyhow::Result<()> {
    let CreateWorkspace {
        config: ConfigWithDeviceSharedOpts { config_dir, device },
        name,
    } = create_workspace;

    load_client_and_run(config_dir, device, |client| async move {
        let mut handle = start_spinner("Creating workspace".into());

        let id = client.create_workspace(name).await?.simple();
        client.ensure_workspaces_bootstrapped().await?;

        handle.stop_with_message(format!(
            "Workspace has been created with id: {YELLOW}{id}{RESET}"
        ));

        client.stop().await;

        Ok(())
    })
    .await
}

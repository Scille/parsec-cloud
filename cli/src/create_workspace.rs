// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec::EntryName;

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct CreateWorkspace {
        /// New workspace name
        #[arg(short, long)]
        name: EntryName,
    }
);

pub async fn create_workspace(create_workspace: CreateWorkspace) -> anyhow::Result<()> {
    let CreateWorkspace {
        name,
        device,
        config_dir,
        password_stdin,
    } = create_workspace;
    log::trace!(
        "Creating workspace {name} (confdir={}, device={})",
        config_dir.display(),
        device.as_deref().unwrap_or("N/A")
    );

    load_client_and_run(&config_dir, device, password_stdin, |client| async move {
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

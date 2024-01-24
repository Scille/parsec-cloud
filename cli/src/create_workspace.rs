// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use clap::Args;
use std::path::PathBuf;

use libparsec::{get_default_config_dir, EntryName};

use crate::utils::*;

#[derive(Args)]
pub struct CreateWorkspace {
    /// Parsec config directory
    #[arg(short, long, default_value_os_t = get_default_config_dir())]
    config_dir: PathBuf,
    /// Device slughash
    #[arg(short, long)]
    device: Option<String>,
    /// New workspace name
    #[arg(short, long)]
    name: EntryName,
}

pub async fn create_workspace(create_workspace: CreateWorkspace) -> anyhow::Result<()> {
    let CreateWorkspace {
        config_dir,
        device,
        name,
    } = create_workspace;

    load_client_and_run(config_dir, device, |client| async move {
        let handle = start_spinner("Creating workspace");

        let id = client.create_workspace(name).await?.simple();

        handle.done();

        println!("Workspace has been created with id: {YELLOW}{id}{RESET}");

        client.stop().await;

        Ok(())
    })
    .await
}

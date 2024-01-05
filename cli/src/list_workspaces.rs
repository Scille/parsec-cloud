// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use clap::Args;
use std::path::PathBuf;

use libparsec::get_default_config_dir;

use crate::utils::*;

#[derive(Args)]
pub struct ListWorkspaces {
    /// Parsec config directory
    #[arg(short, long, default_value_os_t = get_default_config_dir())]
    config_dir: PathBuf,
    /// Device slughash
    #[arg(short, long)]
    device: Option<String>,
}

pub async fn list_workspaces(list_workspaces: ListWorkspaces) -> anyhow::Result<()> {
    let ListWorkspaces { config_dir, device } = list_workspaces;

    load_client_and_run(config_dir, device, |client| async move {
        let workspaces = client.user_ops.list_workspaces();

        if workspaces.is_empty() {
            println!("No workspaces found");
        } else {
            let n = workspaces.len();
            println!("Found {GREEN}{n}{RESET} workspace(s)");

            for (id, name) in workspaces {
                let id = id.hex();
                println!("{YELLOW}{id}{RESET} - {name}");
            }
        }

        client.stop().await;

        Ok(())
    })
    .await
}

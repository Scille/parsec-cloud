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
        client.poll_server_for_new_certificates().await?;
        client.refresh_workspaces_list().await?;
        let workspaces = client.list_workspaces().await;

        if workspaces.is_empty() {
            println!("No workspaces found");
        } else {
            let n = workspaces.len();
            println!("Found {GREEN}{n}{RESET} workspace(s)");

            for ws in workspaces {
                let id = ws.id.hex();
                let name = ws.name;
                let role = ws.self_current_role;
                println!("{YELLOW}{id}{RESET} - {name}: {role}");
            }
        }

        client.stop().await;

        Ok(())
    })
    .await
}

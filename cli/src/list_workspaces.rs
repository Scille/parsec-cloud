// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::utils::*;

#[derive(clap::Parser)]
pub struct ListWorkspaces {
    #[clap(flatten)]
    config: ConfigWithDeviceSharedOpts,
}

pub async fn list_workspaces(list_workspaces: ListWorkspaces) -> anyhow::Result<()> {
    let ListWorkspaces {
        config: ConfigWithDeviceSharedOpts { config_dir, device },
    } = list_workspaces;

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
                let name = ws.current_name;
                let role = ws.current_self_role;
                println!("{YELLOW}{id}{RESET} - {name}: {role}");
            }
        }

        client.stop().await;

        Ok(())
    })
    .await
}

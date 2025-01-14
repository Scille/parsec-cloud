// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {}
);

crate::build_main_with_client!(main, list_workspace);

pub async fn list_workspace(_args: Args, client: &StartedClient) -> anyhow::Result<()> {
    log::trace!("Listing workspaces");

    poll_server_for_new_certificates(client).await?;
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

    Ok(())
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, workspace, password_stdin]
    pub struct Args {
    }
);

crate::build_main_with_client!(main, list_users_workspace);

pub async fn list_users_workspace(args: Args, client: &StartedClient) -> anyhow::Result<()> {
    let Args { workspace: wid, .. } = args;

    log::trace!("Listing users in the workspace");

    poll_server_for_new_certificates(client).await?;
    client.refresh_workspaces_list().await?;
    let users = client.list_workspace_users(wid).await?;

    if users.is_empty() {
        println!("No user has access to that workspace");
    } else {
        let n = users.len();
        println!("Workspace {wid} is shared with {GREEN}{n}{RESET} user(s)");

        for user in users {
            let id = user.user_id;
            let name = user.human_handle.label();
            let email = user.human_handle.email();
            let role = user.current_role;
            let profile = user.current_profile;

            println!("{BULLET_CHAR} User {YELLOW}{id}{RESET} ({YELLOW}{profile}{RESET}) - {GREEN}{name}{RESET} ({email}) has role {GREEN}{role}{RESET}");
        }
    }

    Ok(())
}

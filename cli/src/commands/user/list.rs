// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {
        /// Skip revoked users
        #[arg(short, long, default_value_t)]
        skip_revoked: bool,
    }
);

crate::build_main_with_client!(main, list_user);

pub async fn list_user(args: Args, client: &StartedClient) -> anyhow::Result<()> {
    let Args { skip_revoked, .. } = args;
    log::trace!("Listing users (skip_revoked={skip_revoked})");

    client.poll_server_for_new_certificates().await?;
    let users = client.list_users(skip_revoked, None, None).await?;

    if users.is_empty() {
        println!("No users found");
    } else {
        let n = users.len();
        println!("Found {GREEN}{n}{RESET} user(s):");
        for info in users {
            let id = info.id;
            let human_handle = info.human_handle;
            let profile = info.current_profile.to_string();
            if let (Some(revoked_on), Some(revoked_by)) = (info.revoked_on, info.revoked_by) {
                println!("{YELLOW}{id}{RESET} - {human_handle}: {profile} | revoked on {revoked_on} by {revoked_by}");
            } else {
                println!("{YELLOW}{id}{RESET} - {human_handle}: {profile}");
            }
        }
    }

    Ok(())
}

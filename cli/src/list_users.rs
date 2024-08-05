// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct ListUsers {
        /// Skip revoked users
        #[arg(short, long, default_value_t)]
        skip_revoked: bool,
    }
);

pub async fn list_users(list_users: ListUsers) -> anyhow::Result<()> {
    let ListUsers {
        skip_revoked,
        device,
        config_dir,
        password_stdin,
    } = list_users;
    log::trace!(
        "Listing users (confdir={}, device={}, skip_revoked={skip_revoked})",
        config_dir.display(),
        device.as_deref().unwrap_or("N/A")
    );

    load_client_and_run(&config_dir, device, password_stdin, |client| async move {
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

        client.stop().await;

        Ok(())
    })
    .await
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use clap::Args;
use std::path::PathBuf;

use libparsec::get_default_config_dir;

use crate::utils::*;

#[derive(Args)]
pub struct ListUsers {
    /// Parsec config directory
    #[arg(short, long, default_value_os_t = get_default_config_dir())]
    config_dir: PathBuf,
    /// Device slughash
    #[arg(short, long)]
    device: Option<String>,
    /// Skip revoked users
    #[arg(short, long, default_value_t)]
    skip_revoked: bool,
}

pub async fn list_users(list_users: ListUsers) -> anyhow::Result<()> {
    let ListUsers {
        config_dir,
        device,
        skip_revoked,
    } = list_users;

    load_client_and_run(config_dir, device, |client| async move {
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

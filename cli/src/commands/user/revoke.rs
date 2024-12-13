// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use anyhow::anyhow;

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {
        /// Email of the user to revoke
        email: String,
    }
);

crate::build_main_with_client!(main, revoke_user);

pub async fn revoke_user(args: Args, client: &StartedClient) -> anyhow::Result<()> {
    let Args { email, .. } = args;
    client.poll_server_for_new_certificates().await?;
    let users = client.list_users(true, None, None).await?;
    let to_revoke = users
        .iter()
        .find(|info| info.human_handle.email() == email)
        .ok_or(anyhow!("User not found"))?;
    client.revoke_user(to_revoke.id).await?;

    println!("User {email} has been revoked");

    Ok(())
}

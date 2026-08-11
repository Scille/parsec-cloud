// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
use std::io::Write;

use anyhow::anyhow;
use libparsec::EmailAddress;

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {
        /// Email of the user to revoke
        #[arg(value_hint = clap::ValueHint::EmailAddress)]
        email: EmailAddress,
    }
);

crate::build_main_with_client!(main, revoke_user);

pub async fn revoke_user(ui: crate::Ui, args: Args, client: &StartedClient) -> anyhow::Result<()> {
    let Args { email, .. } = args;
    poll_server_for_new_certificates(&ui, client).await?;
    let users = client.list_users(true, None, None).await?;
    let to_revoke = users
        .iter()
        .find(|info| info.human_handle.email() == &email)
        .ok_or(anyhow!("User not found"))?;
    client.revoke_user(to_revoke.id).await?;

    ui.with_message(|_, out| writeln!(out, "User {email} has been revoked"))?;

    Ok(())
}

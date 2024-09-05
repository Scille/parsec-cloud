// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec::{RealmRole, UserID};

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, workspace, password_stdin]
    pub struct Args {
        /// Recipient id
        #[arg(short, long, value_parser = UserID::from_hex)]
        user_id: UserID,
        /// Role (owner/manager/contributor/reader)
        #[arg(short, long)]
        role: RealmRole,
    }
);

pub async fn main(args: Args) -> anyhow::Result<()> {
    let Args {
        workspace: wid,
        user_id,
        role,
        device,
        config_dir,
        password_stdin,
    } = args;
    log::trace!(
        "Sharing workspace {wid} to {user_id} with role {role} (confdir={}, device={})",
        config_dir.display(),
        device.as_deref().unwrap_or("N/A")
    );

    let client = load_client(&config_dir, device, password_stdin).await?;
    let mut handle = start_spinner("Sharing workspace".into());

    client.share_workspace(wid, user_id, Some(role)).await?;

    handle.stop_with_message("Workspace has been shared".into());

    client.stop().await;

    Ok(())
}

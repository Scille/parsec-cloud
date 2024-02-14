// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use clap::Args;
use std::path::PathBuf;

use libparsec::{get_default_config_dir, RealmRole, UserID, VlobID};

use crate::utils::*;

#[derive(Args)]
pub struct ShareWorkspace {
    /// Parsec config directory
    #[arg(short, long, default_value_os_t = get_default_config_dir())]
    config_dir: PathBuf,
    /// Device slughash
    #[arg(short, long)]
    device: Option<String>,
    /// Workspace id
    #[arg(short, long, value_parser = VlobID::from_hex)]
    workspace_id: VlobID,
    /// Recipient id
    #[arg(short, long)]
    user_id: UserID,
    /// Role (owner/manager/contributor/reader)
    #[arg(short, long)]
    role: RealmRole,
}

pub async fn share_workspace(share_workspace: ShareWorkspace) -> anyhow::Result<()> {
    let ShareWorkspace {
        config_dir,
        device,
        workspace_id,
        user_id,
        role,
    } = share_workspace;

    load_client_and_run(config_dir, device, |client| async move {
        let mut handle = start_spinner("Sharing workspace".into());

        client
            .share_workspace(workspace_id, user_id, Some(role))
            .await?;

        handle.stop_with_message("Workspace has been shared".into());

        client.stop().await;

        Ok(())
    })
    .await
}

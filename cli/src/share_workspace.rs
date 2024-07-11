// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec::{RealmRole, UserID, VlobID};

use crate::utils::*;

#[derive(clap::Parser)]
pub struct ShareWorkspace {
    #[clap(flatten)]
    config: ConfigWithDeviceSharedOpts,
    /// Workspace id
    #[arg(short, long, value_parser = VlobID::from_hex)]
    workspace_id: VlobID,
    /// Recipient id
    #[arg(short, long, value_parser = UserID::from_hex)]
    user_id: UserID,
    /// Role (owner/manager/contributor/reader)
    #[arg(short, long)]
    role: RealmRole,
}

pub async fn share_workspace(share_workspace: ShareWorkspace) -> anyhow::Result<()> {
    let ShareWorkspace {
        config: ConfigWithDeviceSharedOpts { config_dir, device },
        workspace_id,
        user_id,
        role,
    } = share_workspace;
    log::trace!(
        "Sharing workspace {workspace_id} to {user_id} with role {role} (confdir={}, device={})",
        config_dir.display(),
        device.as_deref().unwrap_or("N/A")
    );

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

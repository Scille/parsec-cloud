// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_platform_async::lock::MutexGuard as AsyncMutexGuard;
use libparsec_types::prelude::*;

use super::Client;
use crate::{
    monitors::{start_workspace_inbound_sync_monitor, start_workspace_outbound_sync_monitor},
    workspace::WorkspaceExternalInfo,
    WorkspaceOps,
};

#[derive(Debug, thiserror::Error)]
pub enum ClientStartWorkspaceError {
    #[error("Workspace not found")]
    WorkspaceNotFound,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn start_workspace(
    client: &Client,
    realm_id: VlobID,
) -> Result<Arc<WorkspaceOps>, ClientStartWorkspaceError> {
    // 1) Take the update lock to guarantee the list of started workspace won't change in our back

    let mut workspaces = client.workspaces.lock().await;

    // 2) Check if the workspace is not already started

    let found = workspaces
        .iter()
        .find(|workspace_ops| workspace_ops.realm_id() == realm_id);
    if let Some(workspace_ops) = found {
        return Ok(workspace_ops.to_owned());
    }

    // 3) Actually start the workspace

    let user_manifest = client.user_ops.get_user_manifest();
    let found = user_manifest
        .local_workspaces
        .iter()
        .enumerate()
        .find(|(_, w)| w.id == realm_id);
    let (workspace_index, entry) = match found {
        Some((workspace_index, entry)) => (workspace_index, entry),
        None => return Err(ClientStartWorkspaceError::WorkspaceNotFound),
    };
    let total_workspaces = user_manifest.local_workspaces.len();

    let workspace_ops = Arc::new(
        WorkspaceOps::start(
            client.config.clone(),
            client.device.clone(),
            client.cmds.clone(),
            client.certificates_ops.clone(),
            client.event_bus.clone(),
            realm_id,
            WorkspaceExternalInfo {
                entry: entry.to_owned(),
                workspace_index,
                total_workspaces,
            },
        )
        .await?,
    );

    workspaces.push(workspace_ops.clone());

    // 4) Finally start the monitors

    if client.config.with_monitors {
        let inbound_sync_monitor =
            start_workspace_inbound_sync_monitor(workspace_ops.clone(), client.event_bus.clone())
                .await;
        let outbound_sync_monitor = if entry.role.can_write() {
            let outbound_sync_monitor = start_workspace_outbound_sync_monitor(
                workspace_ops.clone(),
                client.event_bus.clone(),
                client.device.clone(),
            )
            .await;
            Some(outbound_sync_monitor)
        } else {
            None
        };

        let mut monitors = client.monitors.lock().expect("Mutex is poisoned");
        monitors.push(inbound_sync_monitor);
        if let Some(outbound_sync_monitor) = outbound_sync_monitor {
            monitors.push(outbound_sync_monitor);
        }
    }

    Ok(workspace_ops)
}

pub async fn stop_workspace(client: &Client, realm_id: VlobID) {
    let mut workspaces = client.workspaces.lock().await;

    stop_workspace_internal(client, &mut workspaces, realm_id).await
}

pub(super) async fn stop_workspace_internal(
    client: &Client,
    running_workspaces: &mut AsyncMutexGuard<'_, Vec<Arc<WorkspaceOps>>>,
    realm_id: VlobID,
) {
    let found = running_workspaces
        .iter()
        .position(|ops| ops.realm_id() == realm_id);
    let workspace_ops = match found {
        Some(index) => running_workspaces.swap_remove(index),
        // The workspace is not running, go idempotent
        None => return,
    };

    // The workspace is running, first stop it related monitors...

    loop {
        // Must be release the synchronous lock on monitors before the asynchronous stop.
        // Workspace's monitors are only added when the workspace is started, hence
        // it's okay ot release the lock multiple times here.
        let found = {
            let mut monitors = client.monitors.lock().expect("Mutex is poisoned");
            monitors
                .iter()
                .position(|monitor| monitor.workspace_id == Some(realm_id))
                .map(|index| monitors.swap_remove(index))
        };
        match found {
            Some(workspace_monitor) => {
                if let Err(error) = workspace_monitor.stop().await {
                    // TODO: use event bug to log here !
                    log::warn!("Cannot properly stop workspace monitor: {}", error);
                }
            }
            // All cleared !
            None => break,
        }
    }

    // ...then the ops component.

    if let Err(error) = workspace_ops.stop().await {
        // TODO: use event bug to log here !
        log::warn!("Cannot properly stop workspace ops: {}", error);
    }
}

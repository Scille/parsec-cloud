// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_types::prelude::*;

use crate::{ClientGetOutboundSyncBacklogError, WorkspaceOps};

use super::Client;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientGetOutboundSyncBacklogItem {
    pub realm_id: VlobID,
    pub pending_entries: u64,
    pub pending_bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ClientGetOutboundSyncBacklog {
    pub total_pending_entries_for_started_workspaces: u64,
    pub total_pending_bytes_for_started_workspaces: u64,
    pub per_workspace: Vec<ClientGetOutboundSyncBacklogItem>,
}

pub async fn get_outbound_sync_backlog(
    client: &Client,
) -> Result<ClientGetOutboundSyncBacklog, ClientGetOutboundSyncBacklogError> {
    let workspaces: Vec<Arc<WorkspaceOps>> = {
        let workspaces = client.workspaces.lock().await;
        workspaces.clone()
    };

    let mut backlog = ClientGetOutboundSyncBacklog {
        per_workspace: Vec::with_capacity(workspaces.len()),
        ..Default::default()
    };

    for workspace in workspaces.iter() {
        let workspace_backlog = workspace.get_outbound_sync_backlog().await?;
        backlog.total_pending_entries_for_started_workspaces += workspace_backlog.pending_entries;
        backlog.total_pending_bytes_for_started_workspaces += workspace_backlog.pending_bytes;
        backlog
            .per_workspace
            .push(ClientGetOutboundSyncBacklogItem {
                realm_id: workspace.realm_id(),
                pending_entries: workspace_backlog.pending_entries,
                pending_bytes: workspace_backlog.pending_bytes,
            });
    }

    Ok(backlog)
}

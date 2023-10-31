// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#![allow(dead_code)]

use std::{
    ops::DerefMut,
    sync::{Arc, Mutex},
};

use libparsec_platform_async::lock::{Mutex as AsyncMutex, MutexGuard as AsyncMutexGuard};
use libparsec_types::prelude::*;

use crate::{
    event_bus::EventBus,
    monitors::{inbound_sync_monitor_factory, outbound_sync_monitor_factory, Monitor},
    workspace_ops::WorkspaceOps,
};

/// A running workspace is composed of two things:
/// - a workspace ops that provide all the operations to access&modify the workspace
/// - multiple monitors that react to external events to operate on the workspace
struct RunningWorkspace {
    ops: Arc<WorkspaceOps>,
    inbound_sync: Monitor,
    outbound_sync: Monitor,
}

/// Workspace ops can be added or removed during the client lifetime (typically when
/// the user get access to or is removed from a workspace).
///
/// The trick is starting/stopping WorkspaceOps is an asynchronous operation, and
/// we want other workspace ops to be accessible in the meantime.
/// We achieve this with a read-write lock:
/// - only one write access at a time, which corresponds to the workspace ops start/stop
/// - read access correspond to accessing an already started workspace ops, and can be
///   done at any time, even when a write is currently executing
///
/// The structure encapsulates the lock logic to access or modify the list of
/// workspace ops available.
pub struct RunningWorkspaces {
    /// The refresh lock must be held whenever the item list is modified
    refresh_lock: AsyncMutex<()>,
    items: Mutex<Vec<RunningWorkspace>>,
}

pub struct WorkspacesForUpdateGuard<'a> {
    refresh_guard: AsyncMutexGuard<'a, ()>,
    items: &'a Mutex<Vec<RunningWorkspace>>,
}

impl<'a> WorkspacesForUpdateGuard<'a> {
    pub fn get(&self, realm_id: VlobID) -> Option<Arc<WorkspaceOps>> {
        let items = self.items.lock().expect("Mutex is poisoned");
        items.iter().find_map(|x| {
            if x.ops.realm_id() == realm_id {
                Some(x.ops.clone())
            } else {
                None
            }
        })
    }

    pub fn list(&self) -> Vec<Arc<WorkspaceOps>> {
        let items = self.items.lock().expect("Mutex is poisoned");
        items.iter().map(|x| x.ops.clone()).collect()
    }

    pub async fn start_monitors_and_register(
        &self,
        event_bus: EventBus,
        device: Arc<LocalDevice>,
        ops: Arc<WorkspaceOps>,
    ) {
        let inbound_sync = inbound_sync_monitor_factory(event_bus.clone(), ops.clone()).await;
        let outbound_sync = outbound_sync_monitor_factory(event_bus, device, ops.clone()).await;

        let mut items = self.items.lock().expect("Mutex is poisoned");
        items.push(RunningWorkspace {
            ops,
            inbound_sync,
            outbound_sync,
        })
    }

    async fn stop_workspace_monitors(started_workspace: RunningWorkspace) -> Arc<WorkspaceOps> {
        let RunningWorkspace {
            ops,
            mut inbound_sync,
            mut outbound_sync,
        } = started_workspace;

        // Monitors must be stopped before returning the workspace ops, this is to
        // ensure the latter won't get used while it is stopping
        libparsec_platform_async::future::zip(inbound_sync.stop(), outbound_sync.stop()).await;

        ops
    }

    pub async fn stop_monitors_and_unregister(
        &self,
        realm_id: VlobID,
    ) -> Option<Arc<WorkspaceOps>> {
        let started_workspace = {
            let mut items = self.items.lock().expect("Mutex is poisoned");
            let pos = match items.iter().position(|x| x.ops.realm_id() == realm_id) {
                Some(pos) => pos,
                // Nothing to stop :/
                None => return None,
            };
            items.swap_remove(pos)
        };

        let ops = Self::stop_workspace_monitors(started_workspace).await;
        Some(ops)
    }

    pub async fn stop_monitors_and_unregister_all(&self) -> Vec<Arc<WorkspaceOps>> {
        let items = {
            let mut items = self.items.lock().expect("Mutex is poisoned");
            std::mem::take(items.deref_mut())
        };

        let mut opses = Vec::with_capacity(items.len());
        for started_workspace in items {
            let ops = Self::stop_workspace_monitors(started_workspace).await;
            opses.push(ops);
        }

        opses
    }
}

impl RunningWorkspaces {
    pub fn new() -> Self {
        Self {
            refresh_lock: AsyncMutex::default(),
            items: Mutex::default(),
        }
    }

    pub fn get(&self, realm_id: VlobID) -> Option<Arc<WorkspaceOps>> {
        let items = self.items.lock().expect("Mutex is poisoned");
        items.iter().find_map(|x| {
            if x.ops.realm_id() == realm_id {
                Some(x.ops.clone())
            } else {
                None
            }
        })
    }

    pub async fn for_update(&self) -> WorkspacesForUpdateGuard {
        let refresh_guard = self.refresh_lock.lock().await;
        WorkspacesForUpdateGuard {
            refresh_guard,
            items: &self.items,
        }
    }
}

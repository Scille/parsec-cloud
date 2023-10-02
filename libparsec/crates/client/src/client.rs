// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#![allow(dead_code)]

use std::{
    fmt::Debug,
    ops::Deref,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};

use libparsec_client_connection::AuthenticatedCmds;
use libparsec_platform_async::lock::{Mutex as AsyncMutex, MutexGuard as AsyncMutexGuard};
use libparsec_types::prelude::*;

use crate::{
    certificates_monitor::CertificatesMonitor,
    certificates_ops::CertificatesOps,
    config::ClientConfig,
    connection_monitor::ConnectionMonitor,
    event_bus::EventBus,
    user_ops::UserOps,
    workspace_ops::{UserDependantConfig, WorkspaceOps},
};

#[derive(Debug, thiserror::Error)]
pub enum ClientStartWorkspaceError {
    #[error("Cannot start workspace: no access")]
    NoAccess,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum ClientStopWorkspaceError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug)]
pub struct WorkspaceInfo {
    pub id: VlobID,
    pub name: EntryName,
    pub self_role: RealmRole,
}

mod workspaces_ops {
    // Rust allows access of private field of structures defined in the same module.
    // So we define this stuff in it own sub-module to prevent messing with the internals.

    use super::*;

    /// Workspace ops can be added or removed during the client lifetime (typically when
    /// the user get access to or is removed from a workspace).
    ///
    /// The trick is starting/stopping WorkspaceOps is asynchronous, and we want a read
    /// write lock (only one write access at a time, and read access at any time, even
    /// when a write is currently executing).
    ///
    /// The structure encapsulates the lock logic to access or modify the list of
    /// workspace ops available.
    pub struct WorkspaceOpsStore {
        /// The refresh lock must be held whenever the item list is modified
        refresh_lock: AsyncMutex<()>,
        items: Mutex<Vec<Arc<WorkspaceOps>>>,
    }

    pub struct WorkspacesForUpdateGuard<'a> {
        refresh_guard: AsyncMutexGuard<'a, ()>,
        pub items: &'a Mutex<Vec<Arc<WorkspaceOps>>>,
    }

    impl WorkspaceOpsStore {
        pub fn new() -> Self {
            Self {
                refresh_lock: AsyncMutex::default(),
                items: Mutex::default(),
            }
        }

        pub fn get(&self, realm_id: VlobID) -> Option<Arc<WorkspaceOps>> {
            let opses = self.items.lock().expect("Mutex is poisoned");
            opses.iter().find(|ops| ops.realm_id() == realm_id).cloned()
        }

        pub fn list(&self) -> Vec<Arc<WorkspaceOps>> {
            let opses = self.items.lock().expect("Mutex is poisoned");
            opses.to_owned()
        }

        pub async fn for_update(&self) -> WorkspacesForUpdateGuard {
            let refresh_guard = self.refresh_lock.lock().await;
            // let items = self.items.lock().expect("Mutex is poisoned");
            WorkspacesForUpdateGuard {
                refresh_guard,
                items: &self.items,
            }
        }
    }
}
use workspaces_ops::WorkspaceOpsStore;

// Should not be `Clone` given it manages underlying resources !
pub struct Client {
    stopped: AtomicBool,
    pub(crate) config: Arc<ClientConfig>,
    pub(crate) device: Arc<LocalDevice>,
    pub(crate) event_bus: EventBus,
    pub(crate) cmds: Arc<AuthenticatedCmds>,
    pub certificates_ops: Arc<CertificatesOps>,
    pub user_ops: UserOps,
    pub(crate) workspaces_ops: WorkspaceOpsStore,
    connection_monitor: ConnectionMonitor,
    certificates_monitor: CertificatesMonitor,
}

impl Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client")
            .field("device", &self.device.device_id)
            .finish()
    }
}

impl Client {
    pub fn organization_id(&self) -> &OrganizationID {
        self.device.organization_id()
    }

    pub fn device_slug(&self) -> String {
        self.device.slug()
    }

    pub fn device_slughash(&self) -> String {
        self.device.slughash()
    }

    pub fn device_id(&self) -> &DeviceID {
        &self.device.device_id
    }

    pub fn device_label(&self) -> Option<&DeviceLabel> {
        self.device.device_label.as_ref()
    }

    pub fn human_handle(&self) -> Option<&HumanHandle> {
        self.device.human_handle.as_ref()
    }

    pub async fn profile(&self) -> anyhow::Result<UserProfile> {
        self.certificates_ops.get_current_self_profile().await
    }

    pub async fn start(
        config: Arc<ClientConfig>,
        event_bus: EventBus,
        device: Arc<LocalDevice>,
    ) -> Result<Self, anyhow::Error> {
        // TODO: error handling
        let cmds = Arc::new(AuthenticatedCmds::new(
            &config.config_dir,
            device.clone(),
            config.proxy.clone(),
        )?);

        // TODO: error handling
        let certificates_ops = Arc::new(
            CertificatesOps::start(
                config.clone(),
                device.clone(),
                event_bus.clone(),
                cmds.clone(),
            )
            .await?,
        );
        let user_ops = UserOps::start(
            config.clone(),
            device.clone(),
            cmds.clone(),
            certificates_ops.clone(),
            event_bus.clone(),
        )
        .await?;

        let certifs_monitor =
            CertificatesMonitor::start(certificates_ops.clone(), event_bus.clone()).await;
        // Start the connection monitors last, as it send events to others
        let connection_monitor = ConnectionMonitor::start(cmds.clone(), event_bus.clone()).await;

        let client = Self {
            stopped: AtomicBool::new(false),
            config,
            device,
            event_bus,
            cmds,
            certificates_ops,
            user_ops,
            workspaces_ops: WorkspaceOpsStore::new(),
            connection_monitor,
            certificates_monitor: certifs_monitor,
        };

        Ok(client)
    }

    /// Stop the underlying storage (and flush whatever data is not yet on disk)
    ///
    /// Once stopped, it can still theoretically be used (i.e. `stop` doesn't
    /// consume `self`), but will do nothing but return stopped error.
    pub async fn stop(&self) {
        if self.stopped.load(Ordering::Relaxed) {
            return;
        }

        let updater = self.workspaces_ops.for_update().await;
        let workspaces_ops = {
            let mut workspaces_ops = updater.items.lock().expect("Mutex is poisoned");
            std::mem::take(&mut *workspaces_ops)
        };
        for workspace_ops in workspaces_ops {
            let outcome = workspace_ops.stop().await;
            // If a WorkspaceOps fails to be cleanly stopped we still have to close the
            // other ones
            if let Err(error) = outcome {
                // TODO: use event bug to log here !
                log::warn!(
                    "Cannot properly stop workspace ops {}: {}",
                    workspace_ops.realm_id(),
                    error
                );
            }
        }

        self.user_ops.stop().await;
        self.certificates_ops.stop().await;

        self.stopped.store(true, Ordering::Relaxed);
    }

    pub async fn start_workspace(
        &self,
        realm_id: VlobID,
    ) -> Result<Arc<WorkspaceOps>, ClientStartWorkspaceError> {
        // 1. Take the update lock, which guarantee the list of workspace won't change in our back

        let updater = self.workspaces_ops.for_update().await;

        // 2. Check if the workspace is not already started

        {
            let items = updater.items.lock().expect("Mutex is poisoned");
            for ops in items.iter() {
                if ops.realm_id() == realm_id {
                    // Workspace already started, just return it
                    return Ok(ops.to_owned());
                }
            }
        }

        // 3. Actually start the workspace

        let user_role = {
            let available_realms = self
                .certificates_ops
                .get_current_self_realms_roles()
                .await?;
            match available_realms.get(&realm_id) {
                // Never had access, or no longer have access
                None | Some(None) => return Err(ClientStartWorkspaceError::NoAccess),
                // Currently have access
                Some(Some(role)) => *role,
            }
        };

        let user_manifest = self.user_ops.get_user_manifest();
        let (realm_key, workspace_label) = match user_manifest.get_workspace_entry(realm_id) {
            Some(entry) => (entry.key.clone(), entry.name.clone()),
            // We have access to the realm, but our user manifest doesn't mention it :/
            // Two reasons for that:
            // - the sharing granted message hasn't been processed yet
            // - the granted message was corrupted
            //
            // In any way, we cannot start the workspace for the moment.
            None => return Err(ClientStartWorkspaceError::NoAccess),
        };

        let new_workspace_ops = Arc::new(
            WorkspaceOps::start(
                self.config.clone(),
                self.device.clone(),
                self.cmds.clone(),
                self.certificates_ops.clone(),
                self.event_bus.clone(),
                realm_id,
                UserDependantConfig {
                    realm_key,
                    user_role,
                    workspace_name: workspace_label,
                },
            )
            .await?,
        );

        // TODO: should start the related workspace monitor here

        let mut workspaces_started = updater.items.lock().expect("Mutex is poisoned");
        workspaces_started.push(new_workspace_ops.clone());

        Ok(new_workspace_ops)
    }

    pub async fn stop_workspace(&self, realm_id: VlobID) -> anyhow::Result<()> {
        // 1. Take the update lock, which guarantee the list of workspace won't change in our back

        let updater = self.workspaces_ops.for_update().await;

        // 2. Check if the workspace is not already started

        let ops = {
            let mut guard = updater.items.lock().expect("Mutex is poisoned");
            let maybe_started = guard.iter().position(|ops| ops.realm_id() == realm_id);
            match maybe_started {
                // The workspace ops is not started, go idempotent
                None => return Ok(()),
                // Must stop the workspace ops
                Some(index) => guard.swap_remove(index),
            }
        };

        // 3. Actual stop

        // TODO: stop the related workspace monitor ?
        ops.stop()
            .await
            .map_err(|e| e.context("Cannot stop workspace ops"))
    }

    /// This function should typically be called everytime we receive a workspace-related
    /// change from the server (e.g. sharing add/removed).
    pub(crate) async fn refresh_user_dependant_config_in_workspaces(&self) -> anyhow::Result<()> {
        // 1. Take the update lock, which guarantee the list of workspace won't change in our back

        let updater = self.workspaces_ops.for_update().await;

        // 2. For each existing workspace ops, retrieve and update it key (retrieved from
        // user manifest) and user current role (retrieved from the certificates)

        let workspace_opses = {
            let guard = updater.items.lock().expect("Mutex is poisoned");
            guard.deref().clone()
        };

        let available_realms = self
            .certificates_ops
            .get_current_self_realms_roles()
            .await?;
        let user_manifest = self.user_ops.get_user_manifest();

        let mut still_started_workspace_opses = Vec::with_capacity(workspace_opses.len());
        for workspace_ops in workspace_opses {
            let maybe_entry = user_manifest.get_workspace_entry(workspace_ops.realm_id());
            let maybe_role = available_realms
                .get(&workspace_ops.realm_id())
                .cloned()
                .flatten();

            match (maybe_entry, maybe_role) {
                (Some(entry), Some(role)) => {
                    workspace_ops.update_user_dependant_config(|config| {
                        config.realm_key = entry.key.to_owned();
                        config.workspace_name = entry.name.to_owned();
                        config.user_role = role;
                    });
                    still_started_workspace_opses.push(workspace_ops);
                }
                // We no longer have what it takes to run this workspace !
                _ => {
                    // TODO: log error !
                    let _ = workspace_ops.stop().await;
                    // TODO: stop the related workspace monitor ?
                }
            }
        }

        // 3. Finally refresh `workspace_opses` list to remove the now stopped ones
        // (remember: `updater` lock guarantees the list hasn't concurrently changed)
        let mut guard = updater.items.lock().expect("Mutex is poisoned");
        *guard = still_started_workspace_opses;

        Ok(())
    }

    pub async fn list_workspaces(&self) -> anyhow::Result<Vec<WorkspaceInfo>> {
        let realms_roles = self
            .certificates_ops
            .get_current_self_realms_roles()
            .await
            .map_err(|e| e.context("Cannot retrieve self realms roles"))?;
        let workspaces_entries = self.user_ops.list_workspaces();

        // Only keep the workspaces that are actually accessible:
        // - an entry is present in the user manifest
        // - we currently have access to the related realm
        let infos = workspaces_entries
            .into_iter()
            .filter_map(|(id, name)| {
                let self_role = match realms_roles.get(&id).cloned() {
                    // Currently have access to the realm
                    Some(Some(role)) => role,
                    // No longer have access to the realm
                    Some(None) => return None,
                    // There is two reason for having an entry but no certificates:
                    // 1. user manifest's data are invalid :(
                    // 2. we have just created workspace and haven't synced it yet
                    //
                    // Here we always consider we are in case 2 (hence our role is
                    // always OWNER): case 1 is highly unlikely and returning the wrong
                    // role won't do much harm anyway (as server enforce access).
                    None => RealmRole::Owner,
                };
                Some(WorkspaceInfo {
                    id,
                    name,
                    self_role,
                })
            })
            .collect();

        Ok(infos)
    }

    pub async fn new_user_invitation(
        &self,
        claimer_email: String,
        send_email: bool,
    ) -> Result<
        (InvitationToken, crate::invite::InvitationEmailSentStatus),
        crate::invite::NewUserInvitationError,
    > {
        crate::invite::new_user_invitation(&self.cmds, claimer_email, send_email).await
    }

    pub async fn new_device_invitation(
        &self,
        send_email: bool,
    ) -> Result<
        (InvitationToken, crate::invite::InvitationEmailSentStatus),
        crate::invite::NewDeviceInvitationError,
    > {
        crate::invite::new_device_invitation(&self.cmds, send_email).await
    }

    pub async fn delete_invitation(
        &self,
        token: InvitationToken,
    ) -> Result<(), crate::invite::DeleteInvitationError> {
        crate::invite::delete_invitation(&self.cmds, token).await
    }

    pub async fn list_invitations(
        &self,
    ) -> Result<Vec<crate::invite::InviteListItem>, crate::invite::ListInvitationsError> {
        crate::invite::list_invitations(&self.cmds).await
    }

    pub fn start_user_invitation_greet(
        &self,
        token: InvitationToken,
    ) -> crate::invite::UserGreetInitialCtx {
        crate::invite::UserGreetInitialCtx::new(
            self.device.clone(),
            self.cmds.clone(),
            self.event_bus.clone(),
            token,
        )
    }

    pub fn start_device_invitation_greet(
        &self,
        token: InvitationToken,
    ) -> crate::invite::DeviceGreetInitialCtx {
        crate::invite::DeviceGreetInitialCtx::new(
            self.device.clone(),
            self.cmds.clone(),
            self.event_bus.clone(),
            token,
        )
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        if !self.stopped.load(Ordering::Relaxed) {
            log::error!("Client dropped without prior stop !");
        }
    }
}

#[cfg(test)]
#[path = "../tests/unit/client.rs"]
#[allow(clippy::unwrap_used)]
mod tests;

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#![allow(dead_code)]

use std::{
    fmt::Debug,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use libparsec_client_connection::AuthenticatedCmds;
use libparsec_types::prelude::*;

use crate::{
    certificates_monitor::CertificatesMonitor,
    certificates_ops::CertificatesOps,
    config::ClientConfig,
    connection_monitor::ConnectionMonitor,
    event_bus::EventBus,
    messages_monitor::MessagesMonitor,
    running_workspace::RunningWorkspaces,
    user_ops::UserOps,
    user_sync_monitor::UserSyncMonitor,
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
    pub self_current_role: RealmRole,
    pub is_started: bool,
}

// Should not be `Clone` given it manages underlying resources !
pub struct Client {
    stopped: AtomicBool,
    pub(crate) config: Arc<ClientConfig>,
    pub(crate) device: Arc<LocalDevice>,
    pub(crate) event_bus: EventBus,
    pub(crate) cmds: Arc<AuthenticatedCmds>,
    pub certificates_ops: Arc<CertificatesOps>,
    pub user_ops: Arc<UserOps>,
    pub(crate) running_workspaces: RunningWorkspaces,
    connection_monitor: ConnectionMonitor,
    certificates_monitor: CertificatesMonitor,
    messages_monitor: MessagesMonitor,
    user_sync_monitor: UserSyncMonitor,
}

impl Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client")
            .field("device", &self.device.device_id)
            .finish()
    }
}

impl Client {
    pub fn organization_addr(&self) -> &BackendOrganizationAddr {
        &self.device.organization_addr
    }

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

    pub fn device_label(&self) -> &DeviceLabel {
        &self.device.device_label
    }

    pub fn human_handle(&self) -> &HumanHandle {
        &self.device.human_handle
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
        let user_ops = Arc::new(
            UserOps::start(
                config.clone(),
                device.clone(),
                cmds.clone(),
                certificates_ops.clone(),
                event_bus.clone(),
            )
            .await?,
        );

        let certificates_monitor =
            CertificatesMonitor::start(certificates_ops.clone(), event_bus.clone()).await;
        let messages_monitor = MessagesMonitor::start(user_ops.clone(), event_bus.clone()).await;
        let user_sync_monitor = UserSyncMonitor::start(user_ops.clone(), event_bus.clone()).await;
        // Start the connection monitors last, as it send the initial event that wakeup to others
        let connection_monitor = ConnectionMonitor::start(cmds.clone(), event_bus.clone()).await;

        let client = Self {
            stopped: AtomicBool::new(false),
            config,
            device,
            event_bus,
            cmds,
            certificates_ops,
            user_ops,
            running_workspaces: RunningWorkspaces::new(),
            connection_monitor,
            certificates_monitor,
            messages_monitor,
            user_sync_monitor,
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

        let updater = self.running_workspaces.for_update().await;
        for workspace_ops in updater.stop_monitors_and_unregister_all().await {
            let outcome = workspace_ops.stop().await;

            // If a workspace ops fails to be cleanly stopped we still have to close the other ones
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
        // 1. Take the update lock to guarantee the list of started workspace won't change in our back

        let updater = self.running_workspaces.for_update().await;

        // 2. Check if the workspace is not already started

        if let Some(workspace_ops) = updater.get(realm_id) {
            return Ok(workspace_ops);
        }

        // 3. Actually start the workspace

        // The workspace ops needs a couple of data that may change at anytime (they are
        // then bundled together as the `UserDependantConfig`).
        // Updating those data in the workspace opses require holding the workspace
        // opses store update lock, hence we are protected here against concurrency.

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

        let workspace_ops = Arc::new(
            WorkspaceOps::start(
                self.config.clone(),
                self.device.clone(),
                self.cmds.clone(),
                self.certificates_ops.clone(),
                self.event_bus.clone(),
                realm_id,
                UserDependantConfig {
                    realm_key: Arc::new(realm_key),
                    user_role,
                    workspace_name: workspace_label,
                },
            )
            .await?,
        );

        updater
            .start_monitors_and_register(
                self.event_bus.clone(),
                self.device.clone(),
                workspace_ops.clone(),
            )
            .await;

        Ok(workspace_ops)
    }

    pub async fn stop_workspace(&self, realm_id: VlobID) -> anyhow::Result<()> {
        // 1. Take the update lock to guarantee the list of started workspaces won't change in our back

        let updater = self.running_workspaces.for_update().await;

        // 2. Actual stop

        let workspace_ops = match updater.stop_monitors_and_unregister(realm_id).await {
            Some(workspace_ops) => workspace_ops,
            // The workspace ops is not started, go idempotent
            None => return Ok(()),
        };

        workspace_ops
            .stop()
            .await
            .map_err(|e| e.context("Cannot stop workspace ops"))
    }

    /// This function should typically be called everytime we receive a workspace-related
    /// change from the server (e.g. sharing add/removed).
    pub(crate) async fn refresh_user_dependant_config_in_workspaces(&self) -> anyhow::Result<()> {
        // 1. Take the update lock to guarantee the list of started workspace won't change in our back

        let updater = self.running_workspaces.for_update().await;

        // 2. For each started workspace, retrieve and update its key (retrieved from
        // user manifest) and user current role (retrieved from the certificates).
        // And if the key or current role are no longer available, stop the workspace.

        let available_realms = self
            .certificates_ops
            .get_current_self_realms_roles()
            .await?;
        let user_manifest = self.user_ops.get_user_manifest();

        for workspace_ops in updater.list() {
            let maybe_entry = user_manifest.get_workspace_entry(workspace_ops.realm_id());
            let maybe_role = available_realms
                .get(&workspace_ops.realm_id())
                .cloned()
                .flatten();

            match (maybe_entry, maybe_role) {
                (Some(entry), Some(role)) => {
                    workspace_ops.update_user_dependant_config(|config| {
                        config.realm_key = Arc::new(entry.key.to_owned());
                        config.workspace_name = entry.name.to_owned();
                        config.user_role = role;
                    });
                }
                // We no longer have what it takes to run this workspace, so stop it !
                _ => {
                    updater
                        .stop_monitors_and_unregister(workspace_ops.realm_id())
                        .await;
                    let outcome = workspace_ops.stop().await;

                    // If a WorkspaceOps fails to be cleanly stopped we still have to close the other ones
                    if let Err(error) = outcome {
                        // TODO: use event bug to log here !
                        log::warn!(
                            "Cannot properly stop workspace ops {}: {}",
                            workspace_ops.realm_id(),
                            error
                        );
                    }
                }
            }
        }

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
                let self_current_role = match realms_roles.get(&id).cloned() {
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
                    self_current_role,
                    is_started: self.running_workspaces.get(id).is_some(),
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

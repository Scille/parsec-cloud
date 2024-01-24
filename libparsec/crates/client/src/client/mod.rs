// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#![allow(dead_code)]

mod workspace_bootstrap;
mod workspace_create;
mod workspace_list;
mod workspace_refresh_list;
mod workspace_rename;
mod workspace_rotate_key;
mod workspace_share;
mod workspace_start;

use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

pub use self::{
    workspace_bootstrap::ClientEnsureWorkspacesBootstrappedError,
    workspace_create::ClientCreateWorkspaceError, workspace_list::WorkspaceInfo,
    workspace_refresh_list::RefreshWorkspacesListError,
    workspace_rename::ClientRenameWorkspaceError,
    workspace_rotate_key::ClientRotateWorkspaceKeyError,
    workspace_share::ClientShareWorkspaceError, workspace_start::ClientStartWorkspaceError,
};
use crate::{
    certif::{CertifOps, CertifPollServerError},
    config::ClientConfig,
    event_bus::EventBus,
    monitors::{
        start_certif_poll_monitor, start_connection_monitor, start_user_sync_monitor,
        start_workspaces_boostrap_monitor, Monitor,
    },
    user::UserOps,
};
use libparsec_client_connection::AuthenticatedCmds;
use libparsec_platform_async::lock::Mutex as AsyncMutex;
use libparsec_types::prelude::*;

// Re-exposed for public API
pub use crate::certif::{
    CertifGetCurrentSelfProfileError as ClientGetCurrentSelfProfileError,
    CertifGetUserDeviceError as ClientGetUserDeviceError,
    CertifListUserDevicesError as ClientListUserDevicesError,
    CertifListUsersError as ClientListUsersError,
    CertifListWorkspaceUsersError as ClientListWorkspaceUsersError, DeviceInfo, UserInfo,
    WorkspaceUserAccessInfo,
};
pub use crate::invite::{
    CancelInvitationError as ClientCancelInvitationError, DeviceGreetInitialCtx,
    InvitationEmailSentStatus, InviteListItem, ListInvitationsError as ClientListInvitationsError,
    NewDeviceInvitationError as ClientNewDeviceInvitationError,
    NewUserInvitationError as ClientNewUserInvitationError, UserGreetInitialCtx,
};
pub use crate::workspace::WorkspaceOps;

// Should not be `Clone` given it manages underlying resources !
pub struct Client {
    pub(crate) config: Arc<ClientConfig>,
    pub(crate) device: Arc<LocalDevice>,
    pub(crate) event_bus: EventBus,
    pub(crate) cmds: Arc<AuthenticatedCmds>,
    certificates_ops: Arc<CertifOps>,
    user_ops: Arc<UserOps>,
    /// Workspace ops gets added or removed from this list when they are started or stopped.
    ///
    /// The trick is starting/stopping WorkspaceOps is an asynchronous operation, and
    /// we want other workspace ops to be accessible in the meantime.
    /// We achieve this with a read-write lock:
    /// - only one write access at a time, which corresponds to the workspace ops start/stop
    /// - read access correspond to accessing an already started workspace ops, and can be
    ///   done at any time, even when a write is currently executing
    workspaces: AsyncMutex<Vec<Arc<WorkspaceOps>>>,
    monitors: Mutex<Vec<Monitor>>,
}

impl Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client")
            .field("device", &self.device.device_id)
            .finish()
    }
}

impl Client {
    pub async fn start(
        config: Arc<ClientConfig>,
        event_bus: EventBus,
        device: Arc<LocalDevice>,
    ) -> Result<Arc<Self>, anyhow::Error> {
        let with_monitors = config.with_monitors;

        // TODO: error handling
        let cmds = Arc::new(AuthenticatedCmds::new(
            &config.config_dir,
            device.clone(),
            config.proxy.clone(),
        )?);

        // TODO: error handling
        let certificates_ops = Arc::new(
            CertifOps::start(
                config.clone(),
                device.clone(),
                event_bus.clone(),
                cmds.clone(),
            )
            .await?,
        );
        let user_ops = Arc::new(
            UserOps::start(
                &config,
                device.clone(),
                cmds.clone(),
                certificates_ops.clone(),
                event_bus.clone(),
            )
            .await?,
        );

        let client = Arc::new(Self {
            config,
            device,
            event_bus,
            cmds,
            certificates_ops,
            user_ops,
            workspaces: AsyncMutex::default(),
            monitors: Mutex::default(),
        });

        // Start the common monitors

        if with_monitors {
            let workspaces_boostrap_monitor =
                start_workspaces_boostrap_monitor(client.event_bus.clone(), client.clone()).await;

            let user_sync_monitor =
                start_user_sync_monitor(client.user_ops.clone(), client.event_bus.clone()).await;

            let certif_poll_monitor = start_certif_poll_monitor(
                client.certificates_ops.clone(),
                client.event_bus.clone(),
            )
            .await;

            // Start the connection monitors last, as it send the initial event that wakeup to others

            let connection_monitor =
                start_connection_monitor(client.cmds.clone(), client.event_bus.clone()).await;

            let mut monitors = client.monitors.lock().expect("Mutex is poisoned");
            monitors.push(workspaces_boostrap_monitor);
            monitors.push(user_sync_monitor);
            monitors.push(certif_poll_monitor);
            monitors.push(connection_monitor);
        }

        Ok(client)
    }

    /// Stop the monitors and the underlying storage (and flush whatever data is not yet on disk).
    ///
    /// Once stopped, it can still theoretically be used (i.e. `stop` doesn't
    /// consume `self`), but will do nothing but return stopped error.
    pub async fn stop(&self) {
        // 1) Take the lock on the workspaces to prevent any attempt of concurrently
        // starting a new workspace.

        let mut workspaces = self.workspaces.lock().await;

        // 2) First stop the running workspaces (this also stop their monitors and
        // any mountpoint they expose).

        for workspace_ops in workspaces.drain(..) {
            if let Err(error) = workspace_ops.stop().await {
                // TODO: use event bug to log here !
                log::warn!(
                    "Cannot properly stop workspace ops {}: {}",
                    workspace_ops.realm_id(),
                    error
                );
            }
        }

        // 3) Now stop the remaining monitors (which depend on user/certificates ops)

        let monitors = {
            let mut guard = self.monitors.lock().expect("Mutex is poisoned");
            std::mem::take(&mut *guard)
        };
        for monitor in monitors {
            let outcome = monitor.stop().await;
            if let Err(error) = outcome {
                // TODO: use event bug to log here !
                log::warn!("Cannot properly stop monitor: {}", error);
            }
        }

        // 4) Next is the user ops...

        if let Err(error) = self.user_ops.stop().await {
            // TODO: use event bug to log here !
            log::warn!("Cannot properly stop user ops: {}", error);
        }

        // 5) ...and finally the certificates ops as it is the one everything is based on

        if let Err(error) = self.certificates_ops.stop().await {
            // TODO: use event bug to log here !
            log::warn!("Cannot properly stop certificates ops: {}", error);
        }
    }

    /*
     * Public interface
     */

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

    pub async fn get_current_self_profile(
        &self,
    ) -> Result<UserProfile, ClientGetCurrentSelfProfileError> {
        self.certificates_ops.get_current_self_profile().await
    }

    /// List all users.
    pub async fn list_users(
        &self,
        skip_revoked: bool,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> Result<Vec<UserInfo>, ClientListUsersError> {
        self.certificates_ops
            .list_users(skip_revoked, offset, limit)
            .await
    }

    /// List all devices for a given user.
    pub async fn list_user_devices(
        &self,
        user_id: UserID,
    ) -> Result<Vec<DeviceInfo>, ClientListUserDevicesError> {
        self.certificates_ops.list_user_devices(user_id).await
    }

    pub async fn get_user_device(
        &self,
        device_id: DeviceID,
    ) -> Result<(UserInfo, DeviceInfo), ClientGetUserDeviceError> {
        self.certificates_ops.get_user_device(device_id).await
    }

    /// List all users currently having access to a given realm.
    pub async fn list_workspace_users(
        &self,
        realm_id: VlobID,
    ) -> Result<Vec<WorkspaceUserAccessInfo>, ClientListWorkspaceUsersError> {
        self.certificates_ops
            .list_workspace_users(realm_id)
            .await
            .map_err(|err| err.into())
    }

    /// List workspaces available to the current user.
    ///
    /// This is done according to the user manifest cache, hence some very
    /// recent remote changes may not be visible.
    pub async fn list_workspaces(&self) -> Vec<WorkspaceInfo> {
        workspace_list::list_workspaces(self).await
    }

    /// Create a new workspace.
    ///
    /// A new workspace starts its life locally (hence this function can be called while
    /// offline), and is later bootstrapped to be accessible to other users/devices (this
    /// is typically done by a monitor reacting to an event from this function).
    pub async fn create_workspace(
        &self,
        name: EntryName,
    ) -> Result<VlobID, ClientCreateWorkspaceError> {
        workspace_create::create_workspace(self, name).await
    }

    /// Rename an existing workspace, this function requires to be online.
    pub async fn rename_workspace(
        &self,
        realm_id: VlobID,
        new_name: EntryName,
    ) -> Result<(), ClientRenameWorkspaceError> {
        workspace_rename::rename_workspace(self, realm_id, new_name).await
    }

    pub async fn share_workspace(
        &self,
        realm_id: VlobID,
        recipient: UserID,
        role: Option<RealmRole>,
    ) -> Result<(), ClientShareWorkspaceError> {
        workspace_share::share_workspace(self, realm_id, recipient, role).await
    }

    /// Ensure all workspaces are bootstrapped.
    ///
    /// ⚠️ This function doesn't refresh the workspace list to reflect the bootstraps.
    /// This is because it is expected to be called from a monitor (hence the refresh
    /// will be triggered by new certificates events from the server).
    /// When using this function outside of a monitor, you should use
    /// `poll_server_for_new_certificates` then `refresh_workspaces_list` instead:
    ///
    /// ```rust
    /// client.create_workspace("wksp1".parse().unwrap()).await?;
    /// // Workspace is local-only and need to be bootstrapped
    /// client.ensure_workspaces_boostrap().await?;
    /// // Workspace has been bootstrapped, but our client is not aware of the new
    /// // certificates (and its workspace list local cache is out-of-date).
    /// // We could wait for the monitors to catch up, or else do:
    /// client.poll_server_for_new_certificates().await?;
    /// client.refresh_workspaces_list().await?;
    /// // Now the monitors have no remaining job to do \o/
    /// ```
    ///
    /// Given a workspace can be created while offline, it always starts its life locally.
    ///
    /// The bootstrap is the process of creating it the server, and configure it further
    /// so that it can be made accessible to other users/devices (i.e. in Parsec v3 sharing
    /// is only possible on a workspace with a key rotation).
    ///
    /// In practice the workspace boostrap is composed of three steps:
    /// 1. Upload the initial user role certificate. This step correspond to the actual
    ///    creation the realm on the server.
    /// 2. Do the initial key rotation. In Parsec v3+ sharing a workspace involve encrypting
    ///    the keys bundle access for the recipient.
    /// 3. Upload the initial realm name certificate. This step is last as it uses the
    ///    last key to encrypt the realm name.
    ///
    /// Note there is no global atomicity guarantee for those steps:
    /// - Given each step is idempotent, the idea is to repeatedly call them until
    ///   bootstrape is no longer needed.
    /// - A realm created before Parsec v3 will not have step 2 and 3 (and may nevertheless
    ///   be shared !) until an OWNER using a Parsec v3 client calls this function.
    ///
    /// This method is typically used by a monitor.
    pub(crate) async fn ensure_workspaces_bootstrapped(
        &self,
    ) -> Result<(), ClientEnsureWorkspacesBootstrappedError> {
        workspace_bootstrap::ensure_workspaces_bootstrapped(self).await
    }

    /// Force a poll of the server to fetch new certificates.
    ///
    /// This method is typically used by a monitor.
    pub async fn poll_server_for_new_certificates(&self) -> Result<(), CertifPollServerError> {
        self.certificates_ops
            .poll_server_for_new_certificates(None)
            .await
    }

    /// Refresh the workspace list cache by taking into account the certificates that
    /// have been newly fetched.
    ///
    /// This also refreshes the name of the workspaces that have been renamed. Be aware
    /// do so potentially involves server accesses (to fetch the keys bundle).
    ///
    /// This method is typically used by a monitor.
    pub(crate) async fn refresh_workspaces_list(&self) -> Result<(), RefreshWorkspacesListError> {
        workspace_refresh_list::refresh_workspaces_list(self).await
    }

    /// This function deals with subsequent (i.e. post bootstrap) key rotations.
    ///
    /// This is something that cannot be handled by `ensure_workspaces_minimal_sync` given,
    /// unlike workspace rename, key rotation is something meaningless to do while offline:
    /// - Workspace key are only used for encrypting uploaded data,
    /// - Key rotation must be done is a strict order given they contain an index,
    pub(crate) async fn rotate_workspace_key(
        &self,
        realm_id: VlobID,
    ) -> Result<(), ClientRotateWorkspaceKeyError> {
        workspace_rotate_key::rotate_workspace_key(self, realm_id).await
    }

    pub async fn start_workspace(
        &self,
        realm_id: VlobID,
    ) -> Result<Arc<WorkspaceOps>, ClientStartWorkspaceError> {
        workspace_start::start_workspace(self, realm_id).await
    }

    pub async fn stop_workspace(&self, realm_id: VlobID) {
        workspace_start::stop_workspace(self, realm_id).await
    }

    pub async fn new_user_invitation(
        &self,
        claimer_email: String,
        send_email: bool,
    ) -> Result<(InvitationToken, InvitationEmailSentStatus), ClientNewUserInvitationError> {
        crate::invite::new_user_invitation(&self.cmds, claimer_email, send_email).await
    }

    pub async fn new_device_invitation(
        &self,
        send_email: bool,
    ) -> Result<(InvitationToken, InvitationEmailSentStatus), ClientNewDeviceInvitationError> {
        crate::invite::new_device_invitation(&self.cmds, send_email).await
    }

    pub async fn cancel_invitation(
        &self,
        token: InvitationToken,
    ) -> Result<(), ClientCancelInvitationError> {
        crate::invite::cancel_invitation(&self.cmds, token).await
    }

    pub async fn list_invitations(
        &self,
    ) -> Result<Vec<InviteListItem>, ClientListInvitationsError> {
        crate::invite::list_invitations(&self.cmds).await
    }

    pub fn start_user_invitation_greet(&self, token: InvitationToken) -> UserGreetInitialCtx {
        UserGreetInitialCtx::new(
            self.device.clone(),
            self.cmds.clone(),
            self.event_bus.clone(),
            token,
        )
    }

    pub fn start_device_invitation_greet(&self, token: InvitationToken) -> DeviceGreetInitialCtx {
        DeviceGreetInitialCtx::new(
            self.device.clone(),
            self.cmds.clone(),
            self.event_bus.clone(),
            token,
        )
    }
}

#[cfg(test)]
#[path = "../../tests/unit/client/mod.rs"]
#[allow(clippy::unwrap_used)]
mod tests;

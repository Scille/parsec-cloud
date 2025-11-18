// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#![allow(dead_code)]

mod list_frozen_users;
mod organization_info;
mod pki_enrollment_accept;
mod pki_enrollment_finalize;
mod pki_enrollment_info;
mod pki_enrollment_list;
mod pki_enrollment_reject;
mod pki_enrollment_submit;
mod pki_get_addr;
mod recovery_device;
mod shamir_recovery_delete;
mod shamir_recovery_list;
mod shamir_recovery_setup;
mod start_invitation_greet;
mod tos;
mod user_revoke;
mod user_update_profile;
mod workspace_bootstrap;
mod workspace_create;
mod workspace_list;
mod workspace_needs;
mod workspace_refresh_list;
mod workspace_rename;
mod workspace_share;
mod workspace_start;

use std::{
    collections::HashMap,
    fmt::Debug,
    num::NonZeroU8,
    sync::{Arc, Mutex},
};

pub use self::{
    list_frozen_users::ClientListFrozenUsersError,
    pki_enrollment_accept::PkiEnrollmentAcceptError,
    pki_enrollment_finalize::{finalize as pki_enrollment_finalize, PkiEnrollmentFinalizeError},
    pki_enrollment_info::{info as pki_enrollment_info, PKIInfoItem, PkiEnrollmentInfoError},
    pki_enrollment_list::{InvalidityReason, PkiEnrollmentListError, PkiEnrollmentListItem},
    pki_enrollment_reject::PkiEnrollmentRejectError,
    pki_enrollment_submit::{pki_enrollment_submit, PkiEnrollmentSubmitError},
    start_invitation_greet::ClientStartShamirRecoveryInvitationGreetError,
    tos::{ClientAcceptTosError, ClientGetTosError, Tos},
    workspace_bootstrap::ClientEnsureWorkspacesBootstrappedError,
    workspace_create::ClientCreateWorkspaceError,
    workspace_list::WorkspaceInfo,
    workspace_needs::ClientProcessWorkspacesNeedsError,
    workspace_refresh_list::ClientRefreshWorkspacesListError,
    workspace_rename::ClientRenameWorkspaceError,
    workspace_share::ClientShareWorkspaceError,
    workspace_start::ClientStartWorkspaceError,
};
use crate::{
    certif::{CertifPollServerError, CertificateOps},
    config::{ClientConfig, ServerConfig},
    event_bus::EventBus,
    monitors::{
        start_certif_poll_monitor, start_connection_monitor, start_server_config_monitor,
        start_workspaces_bootstrap_monitor, start_workspaces_process_needs_monitor,
        start_workspaces_refresh_list_monitor, Monitor,
    },
    user::UserOps,
    workspace_history::{WorkspaceHistoryOps, WorkspaceHistoryOpsStartError},
};
use libparsec_client_connection::AuthenticatedCmds;
use libparsec_platform_async::lock::Mutex as AsyncMutex;

use libparsec_types::prelude::*;
pub use organization_info::{
    ClientGetOrganizationBootstrapDateError, ClientOrganizationInfoError, OrganizationInfo,
};
pub use recovery_device::{
    import_recovery_device, register_new_device, ClientExportRecoveryDeviceError,
    ImportRecoveryDeviceError, RegisterNewDeviceError,
};

// Re-exposed for public API
pub use crate::certif::{
    CertifForgetAllCertificatesError as ClientForgetAllCertificatesError,
    CertifGetCurrentSelfProfileError as ClientGetCurrentSelfProfileError,
    CertifGetUserDeviceError as ClientGetUserDeviceError,
    CertifGetUserInfoError as ClientGetUserInfoError,
    CertifListUserDevicesError as ClientListUserDevicesError,
    CertifListUsersError as ClientListUsersError,
    CertifListWorkspaceUsersError as ClientListWorkspaceUsersError,
    CertifRevokeUserError as ClientRevokeUserError,
    CertifSetupShamirRecoveryError as ClientSetupShamirRecoveryError,
    CertifUpdateUserProfileError as ClientUserUpdateProfileError, DeviceInfo, UserInfo,
    WorkspaceUserAccessInfo,
};
pub use crate::invite::{
    CancelInvitationError as ClientCancelInvitationError, DeviceGreetInitialCtx,
    InvitationEmailSentStatus, InviteListItem, ListInvitationsError as ClientListInvitationsError,
    NewDeviceInvitationError as ClientNewDeviceInvitationError,
    NewShamirRecoveryInvitationError as ClientNewShamirRecoveryInvitationError,
    NewUserInvitationError as ClientNewUserInvitationError, ShamirRecoveryGreetInitialCtx,
    UserGreetInitialCtx,
};
pub use crate::workspace::WorkspaceOps;
pub use shamir_recovery_delete::ClientDeleteShamirRecoveryError;
pub use shamir_recovery_list::{
    ClientGetSelfShamirRecoveryError, ClientListShamirRecoveriesForOthersError,
    OtherShamirRecoveryInfo, SelfShamirRecoveryInfo,
};

pub type ClientStartWorkspaceHistoryError = WorkspaceHistoryOpsStartError;

// Should not be `Clone` given it manages underlying resources !
pub struct Client {
    pub(crate) config: Arc<ClientConfig>,
    pub(crate) server_config: Mutex<ServerConfig>,
    pub(crate) device: Arc<LocalDevice>,
    pub(crate) event_bus: EventBus,
    pub(crate) cmds: Arc<AuthenticatedCmds>,
    certificates_ops: Arc<CertificateOps>,
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
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
            CertificateOps::start(
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
            server_config: Mutex::new(ServerConfig::default()),
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
            let workspaces_bootstrap_monitor =
                start_workspaces_bootstrap_monitor(client.event_bus.clone(), client.clone()).await;
            let workspaces_process_needs_monitor =
                start_workspaces_process_needs_monitor(client.event_bus.clone(), client.clone())
                    .await;

            let workspaces_refresh_list_monitor =
                start_workspaces_refresh_list_monitor(client.event_bus.clone(), client.clone())
                    .await;

            // TODO: rework user manifest sync once needed
            //
            // User manifest currently doesn't need to be synced (i.e. we only
            // use local user manifest, and rebuild it from scratch on every new
            // client).
            //
            // The current user manifest sync system relies on realm to store the
            // data, which is no longer allowed for users with OUTSIDER profile.
            //
            // Hence enabling the user sync monitor should have no effect, but it's
            // safer to disable it entirely.

            // let user_sync_monitor =
            //     start_user_sync_monitor(client.user_ops.clone(), client.event_bus.clone()).await;

            let certif_poll_monitor = start_certif_poll_monitor(
                client.certificates_ops.clone(),
                client.event_bus.clone(),
            )
            .await;

            let server_config_monitor =
                start_server_config_monitor(client.clone(), client.event_bus.clone()).await;

            // Start the connection monitors last, as it send the initial event that wakeup to others

            let connection_monitor =
                start_connection_monitor(client.cmds.clone(), client.event_bus.clone()).await;

            let mut monitors = client.monitors.lock().expect("Mutex is poisoned");
            monitors.push(workspaces_bootstrap_monitor);
            monitors.push(workspaces_process_needs_monitor);
            monitors.push(workspaces_refresh_list_monitor);
            // monitors.push(user_sync_monitor);
            monitors.push(certif_poll_monitor);
            monitors.push(server_config_monitor);
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

        // 2) First stop the running workspaces (this also stop their monitors).

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
                log::warn!("Cannot properly stop monitor: {error}");
            }
        }

        // 4) Next is the user ops...

        if let Err(error) = self.user_ops.stop().await {
            // TODO: use event bug to log here !
            log::warn!("Cannot properly stop user ops: {error}");
        }

        // 5) ...and finally the certificates ops as it is the one everything is based on

        if let Err(error) = self.certificates_ops.stop().await {
            // TODO: use event bug to log here !
            log::warn!("Cannot properly stop certificates ops: {error}");
        }
    }

    /*
     * Public interface
     */

    pub fn config(&self) -> &ClientConfig {
        &self.config
    }

    pub fn server_config(&self) -> ServerConfig {
        self.server_config
            .lock()
            .expect("Mutex is poisoned")
            .clone()
    }

    pub fn is_server_online(&self) -> bool {
        self.event_bus.is_server_online()
    }

    pub fn is_organization_expired(&self) -> bool {
        self.event_bus.is_organization_expired()
    }

    pub fn must_accept_tos(&self) -> bool {
        self.event_bus.must_accept_tos()
    }

    pub fn organization_addr(&self) -> &ParsecOrganizationAddr {
        &self.device.organization_addr
    }

    pub fn organization_id(&self) -> &OrganizationID {
        self.device.organization_id()
    }

    pub fn device_id(&self) -> DeviceID {
        self.device.device_id
    }

    pub fn user_id(&self) -> UserID {
        self.device.user_id
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

    pub async fn revoke_user(&self, user: UserID) -> Result<(), ClientRevokeUserError> {
        user_revoke::revoke_user(self, user).await
    }

    /// Get user info.
    pub async fn get_user_info(&self, user_id: UserID) -> Result<UserInfo, ClientGetUserInfoError> {
        self.certificates_ops.get_user_info(user_id).await
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

    /// List all frozen users.
    pub async fn list_frozen_users(&self) -> Result<Vec<UserID>, ClientListFrozenUsersError> {
        list_frozen_users::list_frozen_users(&self.cmds).await
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
        self.certificates_ops.list_workspace_users(realm_id).await
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
    ///
    /// If the workspace is not bootstrapped, this function will try to
    /// bootstrap it.
    pub async fn rename_workspace(
        &self,
        realm_id: VlobID,
        new_name: EntryName,
    ) -> Result<(), ClientRenameWorkspaceError> {
        workspace_rename::rename_workspace(self, realm_id, new_name).await
    }

    /// Share the workspace with another user, this function requires to be online.
    ///
    /// If the workspace is not bootstrapped, this function will try to bootstrap
    /// it (this may fail if the current user does not have the OWNER role).
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
    /// client.ensure_workspaces_bootstrap().await?;
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
    /// In practice the workspace bootstrap is composed of three steps:
    /// 1. Upload the initial user role certificate. This step correspond to the actual
    ///    creation the realm on the server.
    /// 2. Do the initial key rotation. In Parsec v3+ sharing a workspace involve encrypting
    ///    the keys bundle access for the recipient.
    /// 3. Upload the initial realm name certificate. This step is last as it uses the
    ///    last key to encrypt the realm name.
    ///
    /// Note there is no global atomicity guarantee for those steps:
    /// - Given each step is idempotent, the idea is to repeatedly call them until
    ///   bootstrap is no longer needed.
    /// - A realm created before Parsec v3 will not have step 2 and 3 (and may nevertheless
    ///   be shared !) until an OWNER using a Parsec v3 client calls this function.
    ///
    /// This method is typically used by a monitor.
    pub async fn ensure_workspaces_bootstrapped(
        &self,
    ) -> Result<(), ClientEnsureWorkspacesBootstrappedError> {
        workspace_bootstrap::ensure_workspaces_bootstrapped(self).await
    }

    /// Force a poll of the server to fetch new certificates.
    ///
    /// Returns the number of new certificates fetched.
    ///
    /// This method is typically used by a monitor.
    pub async fn poll_server_for_new_certificates(&self) -> Result<usize, CertifPollServerError> {
        self.certificates_ops
            .poll_server_for_new_certificates(None)
            .await
    }

    /// Forget all certificates from the local database, this is not needed under normal circumstances.
    ///
    /// Clearing the certificates might be useful in case the server database got rolled back
    /// to a previous state, resulting in the local database containing certificates that are no
    /// longer valid.
    ///
    /// Note that this scenario is technically similar to a server compromise, so this
    /// operation should only result from a manual user action (e.g. CLI command).
    pub async fn forget_all_certificates(&self) -> Result<(), ClientForgetAllCertificatesError> {
        self.certificates_ops.forget_all_certificates().await
    }

    /// Refresh the workspace list cache by taking into account the certificates that
    /// have been newly fetched.
    ///
    /// This also refreshes the name of the workspaces that have been renamed. Be aware
    /// do so potentially involves server accesses (to fetch the keys bundle).
    ///
    /// This method is typically used by a monitor.
    pub async fn refresh_workspaces_list(&self) -> Result<(), ClientRefreshWorkspacesListError> {
        workspace_refresh_list::refresh_workspaces_list(self).await
    }

    /// Workspace needs correspond to:
    /// - The workspace has been unshared with someone, a new key rotation is needed
    /// - A user which is part of the workspace has been revoked, unsharing is needed
    ///
    /// This method looks into the local certificates to determine the needs of each workspace
    /// where the current user has OWNER profile, then proceed to do the requested operations.
    ///
    /// ⚠️ This method doesn't poll for the new certificates that have been created by
    /// those operations.
    /// This is because it is expected to be called from a monitor (hence the refresh
    /// will be triggered by new certificates events from the server).
    ///
    /// This method is typically used by a monitor.
    pub(crate) async fn process_workspaces_needs(
        &self,
    ) -> Result<(), ClientProcessWorkspacesNeedsError> {
        workspace_needs::process_workspaces_needs(self).await
    }

    /// Refresh the server configuration, typically after the server have sent a
    /// `ServerConfig` SSE event.
    ///
    /// This method is typically used by a monitor.
    pub(crate) fn update_server_config(&self, updater: impl FnOnce(&mut ServerConfig)) {
        let mut guard = self.server_config.lock().expect("Mutex is poisoned");
        updater(&mut guard);
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

    pub async fn start_workspace_history(
        &self,
        realm_id: VlobID,
    ) -> Result<Arc<WorkspaceHistoryOps>, ClientStartWorkspaceHistoryError> {
        WorkspaceHistoryOps::start_with_server_access(
            self.config.clone(),
            self.cmds.clone(),
            self.certificates_ops.clone(),
            self.organization_id().to_owned(),
            realm_id,
        )
        .await
        .map(Arc::new)
    }

    pub async fn new_user_invitation(
        &self,
        claimer_email: EmailAddress,
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

    pub async fn new_shamir_recovery_invitation(
        &self,
        user_id: UserID,
        send_email: bool,
    ) -> Result<(InvitationToken, InvitationEmailSentStatus), ClientNewShamirRecoveryInvitationError>
    {
        crate::invite::new_shamir_recovery_invitation(&self.cmds, user_id, send_email).await
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
        start_invitation_greet::start_user_invitation_greet(self, token)
    }

    pub fn start_device_invitation_greet(&self, token: InvitationToken) -> DeviceGreetInitialCtx {
        start_invitation_greet::start_device_invitation_greet(self, token)
    }

    pub async fn start_shamir_recovery_invitation_greet(
        &self,
        token: InvitationToken,
    ) -> Result<ShamirRecoveryGreetInitialCtx, ClientStartShamirRecoveryInvitationGreetError> {
        start_invitation_greet::start_shamir_recovery_invitation_greet(self, token).await
    }

    pub async fn get_self_shamir_recovery(
        &self,
    ) -> Result<SelfShamirRecoveryInfo, ClientGetSelfShamirRecoveryError> {
        shamir_recovery_list::get_self_shamir_recovery(self).await
    }

    pub async fn list_shamir_recoveries_for_others(
        &self,
    ) -> Result<Vec<OtherShamirRecoveryInfo>, ClientListShamirRecoveriesForOthersError> {
        shamir_recovery_list::list_shamir_recoveries_for_others(self).await
    }

    pub async fn setup_shamir_recovery(
        &self,
        per_recipient_shares: HashMap<UserID, NonZeroU8>,
        threshold: NonZeroU8,
    ) -> Result<(), ClientSetupShamirRecoveryError> {
        shamir_recovery_setup::setup_shamir_recovery(self, per_recipient_shares, threshold).await
    }

    pub async fn delete_shamir_recovery(&self) -> Result<(), ClientDeleteShamirRecoveryError> {
        shamir_recovery_delete::delete_shamir_recovery(self).await
    }

    pub async fn get_tos(&self) -> Result<Tos, ClientGetTosError> {
        tos::get_tos(self).await
    }

    pub async fn accept_tos(&self, tos_updated_on: DateTime) -> Result<(), ClientAcceptTosError> {
        tos::accept_tos(self, tos_updated_on).await
    }

    pub async fn export_recovery_device(
        &self,
        device_label: DeviceLabel,
    ) -> Result<(SecretKeyPassphrase, Vec<u8>), ClientExportRecoveryDeviceError> {
        recovery_device::export_recovery_device(self, device_label).await
    }

    pub async fn update_user_profile(
        &self,
        user_id: UserID,
        new_profile: UserProfile,
    ) -> Result<(), ClientUserUpdateProfileError> {
        user_update_profile::update_profile(self, user_id, new_profile).await
    }

    pub async fn organization_info(&self) -> Result<OrganizationInfo, ClientOrganizationInfoError> {
        organization_info::organization_info(self).await
    }

    pub async fn get_organization_bootstrap_date(
        &self,
    ) -> Result<DateTime, ClientGetOrganizationBootstrapDateError> {
        organization_info::get_organization_bootstrap_date(self).await
    }

    pub async fn pki_get_addr(&self) -> ParsecPkiEnrollmentAddr {
        pki_get_addr::get_addr(self).await
    }

    /// List pending PKI enrollments (requests to join an organization)
    pub async fn pki_list_enrollments(
        &self,
    ) -> Result<Vec<PkiEnrollmentListItem>, PkiEnrollmentListError> {
        pki_enrollment_list::list_enrollments(&self.cmds).await
    }

    pub async fn pki_enrollment_reject(
        &self,
        enrollment_id: PKIEnrollmentID,
    ) -> Result<(), PkiEnrollmentRejectError> {
        pki_enrollment_reject::reject(&self.cmds, enrollment_id).await
    }

    pub async fn pki_enrollment_accept(
        &self,
        profile: UserProfile,
        enrollment_id: PKIEnrollmentID,
        human_handle: &HumanHandle,
        cert_ref: &X509CertificateReference,
        submit_payload: &PkiEnrollmentSubmitPayload,
    ) -> Result<(), PkiEnrollmentAcceptError> {
        pki_enrollment_accept::accept(
            self,
            profile,
            enrollment_id,
            human_handle,
            cert_ref,
            submit_payload,
        )
        .await
    }
}

#[cfg(test)]
#[path = "../../tests/unit/client/mod.rs"]
#[allow(clippy::unwrap_used)]
mod tests;

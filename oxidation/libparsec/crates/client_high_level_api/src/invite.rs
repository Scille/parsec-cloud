// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

pub use libparsec_client_types::AvailableDevice;
pub use libparsec_crypto::SequesterVerifyKeyDer;
pub use libparsec_protocol::authenticated_cmds::v3::invite_list::InviteListItem;
pub use libparsec_types::{BackendInvitationAddr, InvitationToken, SASCode, UserProfile};
pub use libparsec_types::{BackendOrganizationBootstrapAddr, DeviceLabel, HumanHandle};

use crate::{ClientHandle, DeviceAccessParams};

//
// Organization bootstrap
//

pub enum OrganizationBootstrapError {
    // TODO: the error that are common to all function that communicate with
    // the server should be implemented by a macro

    // Recognized non-ok statuses
    InviteNotFound,
    InviteAlreadyUsed,
    RefusedByServer,        // Unrecognized non-ok status
    CannotUnderstandServer, // protocol error
    ServerNotReachable,     // network error

    DeviceSavingFailure,
}

#[allow(unused)]
pub async fn bootstrap_organization(
    addr: BackendOrganizationBootstrapAddr,
    human_handle: HumanHandle,
    device_label: DeviceLabel,
    sequester_authority_verify_key: Option<SequesterVerifyKeyDer>,
    save_device_params: DeviceAccessParams,
    login_after_bootstrap: bool, // Useful to avoid having the user enter his smartcard PIN twice
) -> Result<(AvailableDevice, Option<ClientHandle>), OrganizationBootstrapError> {
    unimplemented!();
}

//
// Greeter invite
//

pub enum ClientInviteListError {
    // TODO: common server errors
}
pub async fn client_invite_list() -> Result<Vec<InviteListItem>, ClientInviteListError> {
    unimplemented!()
}

pub enum ClientInviteNewError {
    // TODO: common server errors
}
#[allow(unused)]
pub async fn client_invite_new_user_invitation(
    client: ClientHandle,
    email: &str,
    send_email: bool,
) -> Result<InviteListItem, ClientInviteNewError> {
    unimplemented!();
}
#[allow(unused)]
pub async fn client_invite_new_device_invitation(
    client: ClientHandle,
) -> Result<InviteListItem, ClientInviteNewError> {
    unimplemented!();
}

pub enum ClientInviteDeleteError {
    // TODO: common server errors
    InviteNotFound,
    InviteAlreadyDeleted,
}
#[allow(unused)]
pub async fn client_invite_delete_invitation(
    client: ClientHandle,
    token: InvitationToken,
) -> Result<InviteListItem, ClientInviteDeleteError> {
    unimplemented!();
}
pub type InviteGreeterHandle = u32;

pub struct InviteGreeterUserInProgress1Ctx {
    pub handle: InviteGreeterHandle,
    pub greeter_sas: SASCode,
}
pub struct InviteGreeterDeviceInProgress1Ctx {
    pub handle: InviteGreeterHandle,
    pub greeter_sas: SASCode,
}

pub struct InviteGreeterUserInProgress2Ctx {
    pub handle: InviteGreeterHandle,
    pub claimer_sas: SASCode,
    // 3 bad codes + the good one, in a random order
    pub claimer_sas_choices: [SASCode; 4],
}
pub struct InviteGreeterDeviceInProgress2Ctx {
    pub handle: InviteGreeterHandle,
    pub claimer_sas: SASCode,
    // 3 bad codes + the good one, in a random order
    pub claimer_sas_choices: [SASCode; 4],
}

pub struct InviteGreeterUserInProgress3Ctx {
    pub handle: InviteGreeterHandle,
}
pub struct InviteGreeterDeviceInProgress3Ctx {
    pub handle: InviteGreeterHandle,
}

pub struct InviteGreeterUserInProgress4Ctx {
    pub handle: InviteGreeterHandle,
    pub requested_device_label: Option<DeviceLabel>,
    pub requested_human_handle: Option<HumanHandle>,
}
pub struct InviteGreeterDeviceInProgress4Ctx {
    pub handle: InviteGreeterHandle,
    pub requested_device_label: Option<DeviceLabel>,
}

pub enum ClientInviteStartGreetingError {
    InvalidType, // Invitation is user when greeting a device and vice-versa
}
#[allow(unused)]
pub async fn client_invite_start_greeting_user(
    client: ClientHandle,
    token: InvitationToken,
) -> Result<InviteGreeterUserInProgress1Ctx, ClientInviteStartGreetingError> {
    unimplemented!();
}

#[allow(unused)]
pub async fn client_invite_start_greeting_device(
    client: ClientHandle,
    token: InvitationToken,
) -> Result<InviteGreeterDeviceInProgress1Ctx, ClientInviteStartGreetingError> {
    unimplemented!();
}

pub enum InviteGreeterDoWaitPeerError {
    InvalidHandle,
}
#[allow(unused)]
pub async fn invite_greeter_user_do_wait_peer(
    claimer: InviteGreeterUserInProgress1Ctx,
) -> Result<InviteGreeterUserInProgress2Ctx, InviteGreeterDoWaitPeerError> {
    unimplemented!();
}
#[allow(unused)]
pub async fn invite_greeter_device_do_wait_peer(
    claimer: InviteGreeterDeviceInProgress1Ctx,
) -> Result<InviteGreeterDeviceInProgress2Ctx, InviteGreeterDoWaitPeerError> {
    unimplemented!();
}

pub enum InviteGreeterDoSignifyTrustError {
    InvalidHandle,
}
#[allow(unused)]
pub async fn invite_greeter_user_do_signify_trust(
    claimer: InviteGreeterUserInProgress2Ctx,
) -> Result<InviteGreeterUserInProgress3Ctx, InviteGreeterDoSignifyTrustError> {
    unimplemented!();
}
#[allow(unused)]
pub async fn invite_greeter_device_do_signify_trust(
    claimer: InviteGreeterDeviceInProgress2Ctx,
) -> Result<InviteGreeterDeviceInProgress3Ctx, InviteGreeterDoSignifyTrustError> {
    unimplemented!();
}

pub enum InviteGreeterDoGetClaimRequestsError {
    InvalidHandle,
}
#[allow(unused)]
pub async fn invite_greeter_user_do_get_claim_requests(
    claimer: InviteGreeterUserInProgress3Ctx,
) -> Result<InviteGreeterUserInProgress4Ctx, InviteGreeterDoGetClaimRequestsError> {
    unimplemented!();
}
#[allow(unused)]
pub async fn invite_greeter_device_do_get_claim_requests(
    claimer: InviteGreeterDeviceInProgress3Ctx,
) -> Result<InviteGreeterDeviceInProgress4Ctx, InviteGreeterDoGetClaimRequestsError> {
    unimplemented!();
}

pub enum InviteGreeterDoCreateError {
    InvalidHandle,
}
#[allow(unused)]
pub async fn invite_greeter_user_do_create(
    claimer: InviteGreeterUserInProgress4Ctx,
    device_label: Option<DeviceLabel>,
    human_handle: Option<HumanHandle>,
    profile: UserProfile,
) -> Result<(), InviteGreeterDoCreateError> {
    unimplemented!();
}
#[allow(unused)]
pub async fn invite_greeter_device_do_create(
    claimer: InviteGreeterDeviceInProgress4Ctx,
    device_label: Option<DeviceLabel>,
) -> Result<(), InviteGreeterDoCreateError> {
    unimplemented!();
}

//
// Claimer invite
//

// Invitation claimer & greeter works with a state machine.
// We keep the state machine context in libparsec and expose only a handle.
// This is less elegant than exposing structures (the caller can use a function
// that is not allowed for the current state), but is simple enough.

pub type InviteClaimerHandle = u32;

pub enum InviteClaimerInitialCtx {
    User(InviteClaimerUserInitialCtx),
    Device(InviteClaimerDeviceInitialCtx),
}

pub struct InviteClaimerUserInitialCtx {
    pub handle: InviteClaimerHandle,
}
pub struct InviteClaimerDeviceInitialCtx {
    pub handle: InviteClaimerHandle,
}

pub struct InviteClaimerUserInProgress1Ctx {
    pub handle: InviteClaimerHandle,
    pub greeter_sas: SASCode,
    // 3 bad codes + the good one, in a random order
    pub greeter_sas_choices: [SASCode; 4],
}
pub struct InviteClaimerDeviceInProgress1Ctx {
    pub handle: InviteClaimerHandle,
    pub greeter_sas: SASCode,
    // 3 bad codes + the good one, in a random order
    pub greeter_sas_choices: [SASCode; 4],
}

pub struct InviteClaimerUserInProgress2Ctx {
    pub handle: InviteClaimerHandle,
    pub claimer_sas: SASCode,
}
pub struct InviteClaimerDeviceInProgress2Ctx {
    pub handle: InviteClaimerHandle,
    pub claimer_sas: SASCode,
}

pub struct InviteClaimerUserInProgress3Ctx {
    pub handle: InviteClaimerHandle,
}
pub struct InviteClaimerDeviceInProgress3Ctx {
    pub handle: InviteClaimerHandle,
}

pub enum InviteClaimerRetrieveInfoError {
    // TODO: common server errors
}
#[allow(unused)]
pub async fn invite_claimer_retrieve_info(
    addr: BackendInvitationAddr,
) -> Result<InviteClaimerInitialCtx, InviteClaimerRetrieveInfoError> {
    unimplemented!();
}

pub enum InviteClaimerAbortError {
    InvalidHandle,
}
#[allow(unused)]
pub async fn invite_claimer_abort(
    claimer: InviteClaimerHandle,
) -> Result<(), InviteClaimerAbortError> {
    unimplemented!();
}

// We want to be able to cancel this function (given the wait can take forever
// if peer never connects).
// The current policy is "drop to cancel" (we will re-evaluate this once we hit
// a usecase that doesn't work with this). On the binding side, we should provide
// a way to mark a function cancellable (or maybe consider all functions are ?).
// In this case, the returned promise in Js should have a cancel method that will
// do the drop of the rust future.
pub enum InviteClaimerDoWaitPeerError {
    InvalidHandle,
}
#[allow(unused)]
pub async fn invite_claimer_user_do_wait_peer(
    claimer: InviteClaimerUserInitialCtx,
) -> Result<InviteClaimerUserInProgress1Ctx, InviteClaimerDoWaitPeerError> {
    unimplemented!();
}
#[allow(unused)]
pub async fn invite_claimer_device_do_wait_peer(
    claimer: InviteClaimerDeviceInitialCtx,
) -> Result<InviteClaimerDeviceInProgress1Ctx, InviteClaimerDoWaitPeerError> {
    unimplemented!();
}

pub enum InviteClaimerDoSignifyTrustError {
    InvalidHandle,
}
#[allow(unused)]
pub async fn invite_claimer_user_do_signify_trust(
    claimer: InviteClaimerUserInProgress1Ctx,
) -> Result<InviteClaimerUserInProgress2Ctx, InviteClaimerDoSignifyTrustError> {
    unimplemented!();
}
#[allow(unused)]
pub async fn invite_claimer_device_do_signify_trust(
    claimer: InviteClaimerDeviceInProgress1Ctx,
) -> Result<InviteClaimerDeviceInProgress2Ctx, InviteClaimerDoSignifyTrustError> {
    unimplemented!();
}

pub enum InviteClaimerDoWaitPeerTrustError {
    InvalidHandle,
}
#[allow(unused)]
pub async fn invite_claimer_user_do_wait_peer_trust(
    claimer: InviteClaimerUserInProgress2Ctx,
) -> Result<InviteClaimerUserInProgress3Ctx, InviteClaimerDoWaitPeerTrustError> {
    unimplemented!();
}
#[allow(unused)]
pub async fn invite_claimer_device_do_wait_peer_trust(
    claimer: InviteClaimerDeviceInProgress2Ctx,
) -> Result<InviteClaimerDeviceInProgress3Ctx, InviteClaimerDoWaitPeerTrustError> {
    unimplemented!();
}

pub enum InviteClaimerDoClaimError {
    InvalidHandle,
}
#[allow(unused)]
pub async fn invite_claimer_user_do_claim(
    claimer: InviteClaimerUserInProgress3Ctx,
    requested_device_label: Option<DeviceLabel>,
    requester_human_handle: Option<HumanHandle>,
    save_device_params: DeviceAccessParams,
    login_after_claim: bool,
) -> Result<(AvailableDevice, Option<ClientHandle>), InviteClaimerDoClaimError> {
    unimplemented!();
}
#[allow(unused)]
pub async fn invite_claimer_device_do_claim(
    claimer: InviteClaimerDeviceInProgress3Ctx,
    requester_device_label: Option<DeviceLabel>,
    save_device_params: DeviceAccessParams,
    login_after_claim: bool,
) -> Result<(AvailableDevice, Option<ClientHandle>), InviteClaimerDoClaimError> {
    unimplemented!();
}

//
// PKI enrollment
//

// TODO
// client_pki_enrollement_list_submitted_requests
// client_pki_enrollment_accepter_list_submitted_from_server
// client_pki_enrollment_accepter_accept
// client_pki_enrollment_accepter_reject
// pki_enrollment_submitter_new
// pki_enrollment_submitter_submit
// pki_enrollment_submitter_list_from_disk
// pki_enrollment_submitter_poll
// pki_enrollment_submitter_remove_from_disk
// pki_enrollment_submitter_finalize

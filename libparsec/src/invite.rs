// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use libparsec_client::EventBus;
pub use libparsec_crypto::SequesterVerifyKeyDer;
pub use libparsec_protocol::authenticated_cmds::v3::invite_list::InviteListItem;
pub use libparsec_types::{
    AvailableDevice, BackendInvitationAddr, BackendOrganizationBootstrapAddr, DeviceLabel,
    HumanHandle, InvitationToken, SASCode, UserProfile,
};

use crate::{ClientConfig, ClientEvent, ClientHandle, DeviceAccessParams};

pub use libparsec_client::invite::NewUserInvitationError;

/*
 * Bootstrap organization
 */

pub async fn bootstrap_organization(
    config: ClientConfig,

    // Access to the event bus is done through this callback.
    // Ad-hoc code should be added to the binding system to handle this (hence
    // why this is passed as a parameter instead of as part of `ClientConfig`:
    // we can have a simple `if func_name == "client_login"` that does a special
    // cooking of it last param.
    #[cfg(not(target_arch = "wasm32"))] on_event_callback: Box<dyn FnMut(ClientEvent) + Send>,
    // On web we run on the JS runtime which is monothreaded, hence everything is !Send
    #[cfg(target_arch = "wasm32")] on_event_callback: Box<dyn FnMut(ClientEvent)>,

    bootstrap_organization_addr: &str,
    save_device_params: DeviceAccessParams,
) {
    let addr = bootstrap_organization_addr.parse().map_err(|e| {
        todo!();
    })?;
    let event_bus = EventBus::default();
    libparsec_client::invite::bootstrap_organization(
        config_dir,
        event_bus,
        addr,
        human_handle,
        device_label,
        sequester_authority_verify_key,
    )
    .await
}

/*
 * Greeter invite
 */

//  pub async fn client_invite_new_user_invitation(
//     client: ClientHandle,
//     claimer_email: String,
//     send_email: bool
// ) -> Result<, NewUserInvitationError> {
//     let runing_device = todo!();
//     libparsec_client::invite::new_user_invitation(, claimer_email, send_email)
//  }

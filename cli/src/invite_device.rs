// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec::{
    authenticated_cmds::latest::invite_new_device::{self, InviteNewDeviceRep},
    InvitationType, ParsecInvitationAddr,
};

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct InviteDevice {}
);

pub async fn invite_device(invite_device: InviteDevice) -> anyhow::Result<()> {
    let InviteDevice {
        device,
        config_dir,
        password_stdin,
    } = invite_device;
    log::trace!(
        "Inviting a device (confdir={}, device={})",
        config_dir.display(),
        device.as_deref().unwrap_or("N/A")
    );

    let (cmds, device) = load_cmds(&config_dir, device, password_stdin).await?;
    let mut handle = start_spinner("Creating device invitation".into());

    let rep = cmds
        .send(invite_new_device::Req { send_email: false })
        .await?;

    let url = match rep {
        InviteNewDeviceRep::Ok { token, .. } => ParsecInvitationAddr::new(
            device.organization_addr.clone(),
            device.organization_id().clone(),
            InvitationType::Device,
            token,
        )
        .to_url(),
        rep => {
            return Err(anyhow::anyhow!(
                "Server refused to create device invitation: {rep:?}"
            ));
        }
    };

    handle.stop_with_message(format!("Invitation URL: {YELLOW}{url}{RESET}"));

    Ok(())
}

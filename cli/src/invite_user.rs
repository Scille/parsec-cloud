// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec::{
    authenticated_cmds::latest::invite_new_user::{self, InviteNewUserRep},
    InvitationType, ParsecInvitationAddr,
};

use crate::utils::*;

#[derive(clap::Parser)]
pub struct InviteUser {
    #[clap(flatten)]
    config: ConfigWithDeviceSharedOpts,
    /// Claimer email (i.e.: The invitee)
    #[arg(short, long)]
    email: String,
    /// Send email to the invitee
    #[arg(short, long, default_value_t)]
    send_email: bool,
}

pub async fn invite_user(invite_user: InviteUser) -> anyhow::Result<()> {
    let InviteUser {
        config: ConfigWithDeviceSharedOpts { config_dir, device },
        email,
        send_email,
    } = invite_user;

    load_cmds_and_run(config_dir, device, |cmds, device| async move {
        let mut handle = start_spinner("Creating user invitation".into());

        let rep = cmds
            .send(invite_new_user::Req {
                claimer_email: email,
                send_email,
            })
            .await?;

        let url = match rep {
            InviteNewUserRep::Ok { token, .. } => ParsecInvitationAddr::new(
                device.organization_addr.clone(),
                device.organization_id().clone(),
                InvitationType::Device,
                token,
            )
            .to_url(),
            rep => {
                return Err(anyhow::anyhow!(
                    "Server refused to create user invitation: {rep:?}"
                ));
            }
        };

        handle.stop_with_message(format!("Invitation URL: {YELLOW}{url}{RESET}"));

        Ok(())
    })
    .await
}

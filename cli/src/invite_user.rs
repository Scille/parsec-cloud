// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use clap::Args;
use std::path::PathBuf;

use libparsec::{
    authenticated_cmds::latest::invite_new::{self, InviteNewRep, UserOrDevice},
    get_default_config_dir, BackendInvitationAddr, InvitationType,
};

use crate::utils::*;

#[derive(Args)]
pub struct InviteUser {
    /// Parsec config directory
    #[arg(short, long, default_value_os_t = get_default_config_dir())]
    config_dir: PathBuf,
    /// Greeter device slughash
    #[arg(short, long)]
    device: Option<String>,
    /// Claimer email (i.e.: The invitee)
    #[arg(short, long)]
    email: String,
    /// Send email to the invitee
    #[arg(short, long, default_value_t)]
    send_email: bool,
}

pub async fn invite_user(invite_user: InviteUser) -> anyhow::Result<()> {
    load_device_and_run(
        invite_user.config_dir,
        invite_user.device,
        |cmds, device| async move {
            let claimer_email = invite_user.email;
            let handle = start_spinner("Creating user invitation");

            let rep = cmds
                .send(invite_new::Req(UserOrDevice::User {
                    claimer_email,
                    send_email: invite_user.send_email,
                }))
                .await?;

            let url = match rep {
                InviteNewRep::Ok { token, .. } => BackendInvitationAddr::new(
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

            handle.done();

            println!("Invitation URL: {YELLOW}{url}{RESET}");

            Ok(())
        },
    )
    .await
}

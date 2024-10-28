// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec::{
    authenticated_cmds::latest::invite_new_user::{self, InviteNewUserRep},
    InvitationType, ParsecInvitationAddr,
};

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {
        /// Claimer email (i.e.: The invitee)
        #[arg(short, long)]
        email: String,
        /// Send email to the invitee
        #[arg(short, long, default_value_t)]
        send_email: bool,
    }
);

pub async fn main(args: Args) -> anyhow::Result<()> {
    let Args {
        email,
        send_email,
        device,
        config_dir,
        password_stdin,
    } = args;
    log::trace!(
        "Inviting an user (confdir={}, device={})",
        config_dir.display(),
        device.as_deref().unwrap_or("N/A")
    );

    let (cmds, device) = load_cmds(&config_dir, device, password_stdin).await?;
    let mut handle = start_spinner("Creating user invitation".into());

    let rep = cmds
        .send(invite_new_user::Req {
            claimer_email: email,
            send_email,
        })
        .await?; // TODO: Handle connection error

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
}

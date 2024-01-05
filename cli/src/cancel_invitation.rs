// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use clap::Args;
use std::path::PathBuf;

use libparsec::{
    authenticated_cmds::latest::invite_delete::{self, InvitationDeletedReason, InviteDeleteRep},
    get_default_config_dir, InvitationToken,
};

use crate::utils::*;

#[derive(Args)]
pub struct CancelInvitation {
    /// Parsec config directory
    #[arg(short, long, default_value_os_t = get_default_config_dir())]
    config_dir: PathBuf,
    /// Device slughash
    #[arg(short, long)]
    device: Option<String>,
    /// Invitation token
    #[arg(short, long, value_parser = InvitationToken::from_hex)]
    token: InvitationToken,
}

pub async fn cancel_invitation(cancel_invitation: CancelInvitation) -> anyhow::Result<()> {
    let CancelInvitation {
        config_dir,
        device,
        token,
    } = cancel_invitation;

    load_cmds_and_run(config_dir, device, |cmds, _| async move {
        let handle = start_spinner("Deleting invitation");

        let rep = cmds
            .send(invite_delete::Req {
                token,
                reason: InvitationDeletedReason::Cancelled,
            })
            .await?;

        match rep {
            InviteDeleteRep::Ok => (),
            rep => {
                return Err(anyhow::anyhow!(
                    "Server error while cancelling invitation: {rep:?}"
                ));
            }
        };

        handle.done();

        println!("Invitation deleted");

        Ok(())
    })
    .await
}

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
    #[arg(short, long)]
    token: String,
}

pub async fn cancel_invitation(cancel_invitation: CancelInvitation) -> anyhow::Result<()> {
    load_cmds_and_run(
        cancel_invitation.config_dir,
        cancel_invitation.device,
        |cmds, _| async move {
            let token = InvitationToken::from_hex(&cancel_invitation.token)
                .map_err(|e| anyhow::anyhow!(e))?;

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
        },
    )
    .await
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use clap::Args;
use std::path::PathBuf;

use libparsec::{
    authenticated_cmds::latest::invite_cancel::{self, InviteCancelRep},
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
        let mut handle = start_spinner("Deleting invitation".into());

        let rep = cmds.send(invite_cancel::Req { token }).await?;

        match rep {
            InviteCancelRep::Ok => (),
            rep => {
                return Err(anyhow::anyhow!(
                    "Server error while cancelling invitation: {rep:?}"
                ));
            }
        };

        handle.stop_with_message("Invitation deleted".into());

        Ok(())
    })
    .await
}

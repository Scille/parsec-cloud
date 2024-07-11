// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec::{
    authenticated_cmds::latest::invite_cancel::{self, InviteCancelRep},
    InvitationToken,
};

use crate::utils::*;

#[derive(clap::Parser)]
pub struct CancelInvitation {
    #[clap(flatten)]
    config: ConfigWithDeviceSharedOpts,
    /// Invitation token
    #[arg(short, long, value_parser = InvitationToken::from_hex)]
    token: InvitationToken,
}

pub async fn cancel_invitation(cancel_invitation: CancelInvitation) -> anyhow::Result<()> {
    let CancelInvitation {
        config: ConfigWithDeviceSharedOpts { config_dir, device },
        token,
    } = cancel_invitation;
    log::trace!(
        "Cancelling invitation (confdir={}, device={})",
        config_dir.display(),
        device.as_deref().unwrap_or("N/A")
    );

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

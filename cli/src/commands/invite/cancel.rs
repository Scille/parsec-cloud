// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec::{
    authenticated_cmds::latest::invite_cancel::{self, InviteCancelRep},
    InvitationToken,
};

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {
        /// Invitation token
        #[arg(short, long, value_parser = InvitationToken::from_hex)]
        token: InvitationToken,
    }
);

pub async fn main(args: Args) -> anyhow::Result<()> {
    let Args {
        token,
        device,
        config_dir,
        password_stdin,
    } = args;
    log::trace!(
        "Cancelling invitation (confdir={}, device={})",
        config_dir.display(),
        device.as_deref().unwrap_or("N/A")
    );

    let (cmds, _) = load_cmds(&config_dir, device, password_stdin).await?;
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
}

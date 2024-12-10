// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use anyhow::Context;
use libparsec::InvitationToken;

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {
        /// Invitation token
        #[arg(short, long, value_parser = InvitationToken::from_hex)]
        token: InvitationToken,
    }
);

crate::build_main_with_client!(main, invite_cancel);

pub async fn invite_cancel(args: Args, client: &StartedClient) -> anyhow::Result<()> {
    let Args { token, .. } = args;
    log::trace!("Cancelling invitation");

    let mut handle = start_spinner("Deleting invitation".into());

    client
        .cancel_invitation(token)
        .await
        .context("Server refused to cancel invitation")?;

    handle.stop_with_message("Invitation deleted".into());

    Ok(())
}

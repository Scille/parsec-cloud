// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use anyhow::Context;
use libparsec::{EmailAddress, InvitationType, ParsecInvitationAddr};

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {
        /// Claimer email (i.e.: The invitee)
        email: EmailAddress,
        /// Send email to the invitee
        #[arg(short, long, default_value_t)]
        send_email: bool,
    }
);

crate::build_main_with_client!(main, invite_user);

pub async fn invite_user(args: Args, client: &StartedClient) -> anyhow::Result<()> {
    let Args {
        email, send_email, ..
    } = args;
    log::trace!("Inviting an user");

    let mut handle = start_spinner("Creating user invitation".into());

    let (token, _sent_email_status) = client
        .new_user_invitation(email, send_email)
        .await
        .context("Server refused to create user invitation")?;

    let addr = ParsecInvitationAddr::new(
        client.organization_addr().clone(),
        client.organization_id().clone(),
        InvitationType::User,
        token,
    );

    handle.stop_with_message(format!(
        "Invitation token: {YELLOW}{token}{RESET}\n\
        Invitation URL: {YELLOW}{}{RESET}",
        addr.to_http_redirection_url(),
    ));

    Ok(())
}

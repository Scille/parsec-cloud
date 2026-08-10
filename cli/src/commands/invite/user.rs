// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
use std::fmt::Write;

use anyhow::Context;
use libparsec::{EmailAddress, InvitationType, ParsecInvitationAddr};

use crate::{ui::compat::InvitationLink, utils::StartedClient};

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {
        /// Claimer email (i.e.: The invitee)
        #[arg(value_hint = clap::ValueHint::EmailAddress)]
        email: EmailAddress,
        /// Send email to the invitee
        #[arg(short, long, default_value_t)]
        send_email: bool,
    }
);

crate::build_main_with_client!(main, invite_user);

pub async fn invite_user(ui: crate::Ui, args: Args, client: &StartedClient) -> anyhow::Result<()> {
    let Args {
        email, send_email, ..
    } = args;
    log::trace!("Inviting an user");

    let handle = ui.with_spinner(|_, out| write!(out, "Creating user invitation"))?;

    let (token, _sent_email_status) = client
        .new_user_invitation(email, send_email)
        .await
        .context("Server refused to create user invitation")?;

    let url = ParsecInvitationAddr::new(
        client.organization_addr().clone(),
        client.organization_id().clone(),
        InvitationType::User,
        token,
    )
    .to_http_redirection_url();

    handle.stop();

    ui.data_print(&InvitationLink { token, url })?;

    Ok(())
}

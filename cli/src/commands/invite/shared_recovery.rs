// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
use std::{fmt::Write as _, io::Write as _};

use libparsec::{EmailAddress, InvitationEmailSentStatus, InvitationType, ParsecInvitationAddr};

use crate::{
    ui::compat::InvitationLink,
    utils::{poll_server_for_new_certificates, StartedClient},
};

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {
        /// Claimer email (i.e.: The invitee)
        #[arg(value_hint = clap::ValueHint::EmailAddress)]
        email: EmailAddress,
        /// Send email to the invitee
        #[arg(long, default_value_t)]
        send_email: bool,
    }
);

crate::build_main_with_client!(main, invite_shared_recovery);

pub async fn invite_shared_recovery(
    ui: crate::Ui,
    args: Args,
    client: &StartedClient,
) -> anyhow::Result<()> {
    let Args {
        email, send_email, ..
    } = args;
    log::trace!("Inviting an user to perform a shared recovery");

    poll_server_for_new_certificates(&ui, client).await?;

    let users = client.list_users(true, None, None).await?;
    let user_info = users
        .iter()
        .find(|u| u.human_handle.email() == &email)
        .ok_or_else(|| anyhow::anyhow!("User with email {} not found", email))?;

    let handle = ui.with_spinner(|_, out| write!(out, "Creating a shared recovery invitation"))?;
    let (url, email_sent_status, token) = match client
        .new_shamir_recovery_invitation(user_info.id, send_email)
        .await
    {
        Ok((token, email_sent_status)) => (
            ParsecInvitationAddr::new(
                client.organization_addr().clone(),
                client.organization_id().clone(),
                InvitationType::ShamirRecovery,
                token,
            )
            .to_http_redirection_url(),
            email_sent_status,
            token,
        ),
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Server refused to create shared recovery invitation: {e}"
            ));
        }
    };

    handle.stop();

    if send_email {
        match email_sent_status {
            InvitationEmailSentStatus::Success => {
                ui.with_message(|_, out| writeln!(out, "Invitation email sent to {email}"))?;
            }
            InvitationEmailSentStatus::RecipientRefused => {
                ui.with_message(|_, out| {
                    writeln!(
                        out,
                        "Invitation email not sent to {email} because the recipient was refused"
                    )
                })?;
            }
            InvitationEmailSentStatus::ServerUnavailable => {
                ui.with_message(|_, out| {
                    writeln!(
                        out,
                        "Invitation email not sent to {email} because the server is unavailable"
                    )
                })?;
            }
        }
    }

    ui.data_print(&InvitationLink { token, url })?;

    Ok(())
}

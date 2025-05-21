// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec::{EmailAddress, InvitationEmailSentStatus, InvitationType, ParsecInvitationAddr};

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {
        /// Claimer email (i.e.: The invitee)
        email: EmailAddress,
        /// Send email to the invitee
        #[arg(long, default_value_t)]
        send_email: bool,
    }
);

crate::build_main_with_client!(main, invite_shared_recovery);

pub async fn invite_shared_recovery(args: Args, client: &StartedClient) -> anyhow::Result<()> {
    let Args {
        email, send_email, ..
    } = args;
    log::trace!("Inviting an user to perform a shared recovery");

    poll_server_for_new_certificates(client).await?;

    let users = client.list_users(true, None, None).await?;
    let user_info = users
        .iter()
        .find(|u| u.human_handle.email() == &email)
        .ok_or_else(|| anyhow::anyhow!("User with email {} not found", email))?;

    let mut handle = start_spinner("Creating a shared recovery invitation".into());
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
            .to_url(),
            email_sent_status,
            token,
        ),
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Server refused to create shared recovery invitation: {e}"
            ));
        }
    };

    handle.stop_with_message(format!(
        "Invitation token: {YELLOW}{token}{RESET}\nInvitation URL: {YELLOW}{url}{RESET}"
    ));

    if send_email {
        match email_sent_status {
            InvitationEmailSentStatus::Success => {
                println!("Invitation email sent to {}", email);
            }
            InvitationEmailSentStatus::RecipientRefused => {
                println!(
                    "Invitation email not sent to {} because the recipient was refused",
                    email
                );
            }
            InvitationEmailSentStatus::ServerUnavailable => {
                println!(
                    "Invitation email not sent to {} because the server is unavailable",
                    email
                );
            }
        }
    }

    Ok(())
}

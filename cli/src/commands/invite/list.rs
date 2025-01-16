// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec::{authenticated_cmds::latest::invite_list::InviteListItem, InvitationStatus};

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {}
);

crate::build_main_with_client!(main, list_invite);

pub async fn list_invite(_args: Args, client: &StartedClient) -> anyhow::Result<()> {
    log::trace!("Listing invitations");
    poll_server_for_new_certificates(client).await?;

    let mut handle = start_spinner("Listing invitations".into());

    let invitations = client.list_invitations().await?;

    let users = client.list_users(false, None, None).await?;

    if invitations.is_empty() {
        handle.stop_with_message("No invitation.".into());
    } else {
        handle.stop_with_message(format!("{} invitations found.", invitations.len()));
        for invitation in invitations {
            let (token, status, display_type) = match invitation {
                InviteListItem::User {
                    claimer_email,
                    status,
                    token,
                    ..
                } => (token, status, format!("user (email={claimer_email})")),
                InviteListItem::Device { status, token, .. } => (token, status, "device".into()),
                InviteListItem::ShamirRecovery {
                    status,
                    token,
                    claimer_user_id,
                    ..
                } => {
                    let claimer_human_handle = users
                        .iter()
                        .find(|user| user.id == claimer_user_id)
                        .map(|user| format!("{}", user.human_handle))
                        .unwrap_or("N/A".to_string());
                    (
                        token,
                        status,
                        format!("shamir recovery ({claimer_human_handle})"),
                    )
                }
            };

            let token = token.hex();

            let display_status = match status {
                InvitationStatus::Idle => format!("{YELLOW}idle{RESET}"),
                InvitationStatus::Ready => format!("{GREEN}ready{RESET}"),
                InvitationStatus::Cancelled => format!("{RED}cancelled{RESET}"),
                InvitationStatus::Finished => format!("{GREEN}finished{RESET}"),
            };

            println!("{token}\t{display_status}\t{display_type}");
        }
    }

    Ok(())
}

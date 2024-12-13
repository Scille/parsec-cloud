// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec::{authenticated_cmds::latest::invite_list::InviteListItem, InvitationStatus};
use prettytable::{cell, row, Row, Table};

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {
        /// Display the output in CSV format
        #[arg(long)]
        csv: bool,
    }
);

crate::build_main_with_client!(main, list_invite);

pub async fn list_invite(args: Args, client: &StartedClient) -> anyhow::Result<()> {
    let Args { csv, .. } = args;
    log::trace!("Listing invitations");

    let mut handle = start_spinner("Listing invitations".into());

    let invitations = client.list_invitations().await?;

    let users = client.list_users(false, None, None).await?;

    let mut table = Table::new();
    table.set_titles(row!["Invitation token", "Status", "Type", "Email"]);

    if invitations.is_empty() {
        handle.stop_with_message("No invitation.".into());
    } else {
        handle.stop_with_message(format!("{} invitations found.", invitations.len()));

        for invitation in invitations {
            let (token, status, display_type, email) = match invitation {
                InviteListItem::User {
                    claimer_email,
                    status,
                    token,
                    ..
                } => (token, status, "user", Some(claimer_email)),
                InviteListItem::Device { status, token, .. } => (token, status, "device", None),
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
                    (token, status, "shamir recovery", Some(claimer_human_handle))
                }
            };

            let token = token.hex();

            let display_status = match status {
                InvitationStatus::Idle => cell!(Fy->"idle"),
                InvitationStatus::Ready => cell!(Fg->"ready"),
                InvitationStatus::Cancelled => cell!(Fr->"cancelled"),
                InvitationStatus::Finished => cell!(Fg->"finished"),
            };

            table.add_row(Row::new(vec![
                cell!(token),
                display_status,
                cell!(display_type),
                cell!(email.as_deref().unwrap_or("")),
            ]));
        }
    }

    if csv {
        drop(table.to_csv_writer(prettytable::csv::Writer::from_writer(
            std::io::stdout().lock(),
        ))?);
    } else if !table.is_empty() {
        table.printstd();
    }

    Ok(())
}

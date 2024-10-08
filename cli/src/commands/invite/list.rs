// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec::{
    authenticated_cmds::latest::invite_list::{self, InviteListItem, InviteListRep},
    InvitationStatus,
};

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {}
);

pub async fn main(args: Args) -> anyhow::Result<()> {
    let Args {
        device,
        config_dir,
        password_stdin,
    } = args;
    log::trace!(
        "Listing invitations (confdir={}, device={})",
        config_dir.display(),
        device.as_deref().unwrap_or("N/A")
    );

    let (cmds, _) = load_cmds(&config_dir, device, password_stdin).await?;
    let mut handle = start_spinner("Listing invitations".into());

    let rep = cmds.send(invite_list::Req).await?;

    let invitations = match rep {
        InviteListRep::Ok { invitations } => invitations,
        rep => {
            return Err(anyhow::anyhow!(
                "Server error while listing invitations: {rep:?}"
            ));
        }
    };

    if invitations.is_empty() {
        handle.stop_with_message("No invitation.".into());
    } else {
        for invitation in invitations {
            let (token, status, display_type) = match invitation {
                InviteListItem::User {
                    claimer_email,
                    status,
                    token,
                    ..
                } => (token, status, format!("user (email={claimer_email}")),
                InviteListItem::Device { status, token, .. } => (token, status, "device".into()),
            };

            let token = token.hex();

            let display_status = match status {
                InvitationStatus::Idle => format!("{YELLOW}idle{RESET}"),
                InvitationStatus::Ready => format!("{GREEN}ready{RESET}"),
                InvitationStatus::Cancelled => format!("{RED}cancelled{RESET}"),
                InvitationStatus::Finished => format!("{GREEN}finished{RESET}"),
            };

            handle.stop_with_message(format!("{token}\t{display_status}\t{display_type}"))
        }
    }

    Ok(())
}

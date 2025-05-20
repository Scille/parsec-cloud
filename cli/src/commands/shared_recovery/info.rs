use std::collections::HashMap;

use itertools::Itertools;

use crate::{build_main_with_client, utils::*};

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {
    }
);

build_main_with_client!(main, shared_recovery_info);

pub async fn shared_recovery_info(_args: Args, client: &StartedClient) -> anyhow::Result<()> {
    poll_server_for_new_certificates(client).await?;

    let info = client.get_self_shamir_recovery().await?;
    let users = client.list_users(false, None, None).await?;
    let users: HashMap<_, _> = users.iter().map(|info| (info.id, info)).collect();

    match info {
        libparsec_client::SelfShamirRecoveryInfo::Deleted {
            deleted_on,
            deleted_by,
            ..
        } => println!(
            "{RED}Deleted{RESET} shared recovery - deleted by {deleted_by} on {deleted_on}"
        ),
        libparsec_client::SelfShamirRecoveryInfo::SetupAllValid {
            per_recipient_shares,
            threshold,
            ..
        } => println!(
            "Shared recovery {GREEN}set up{RESET} with threshold {threshold}\n{}",
            per_recipient_shares
                .iter()
                .map(|(recipient, share)| {
                    // this means that a user disappeared completely, it should not happen
                    let user = &users
                        .get(recipient)
                        .expect("missing recipient")
                        .human_handle;
                    format!(
                        "{BULLET_CHAR} User {user} has {share} share{}",
                        maybe_plural(&share.get())
                    )
                })
                .join("\n")
        ),
        libparsec_client::SelfShamirRecoveryInfo::SetupWithRevokedRecipients {
            threshold,
            per_recipient_shares,
            revoked_recipients,
            ..
        } => println!(
            "Shared recovery {YELLOW}set up{RESET} - contains revoked recipient{maybe_plural}: {revoked} ({revoked_len} out of {total} total recipients, with threshold {threshold})",
            maybe_plural = maybe_plural(&(revoked_recipients.len() as u8)),
            revoked = revoked_recipients.iter().join(", "),
            revoked_len = revoked_recipients.len(),
            total = per_recipient_shares.len()
        ),
        libparsec_client::SelfShamirRecoveryInfo::SetupButUnusable {
            threshold,
            per_recipient_shares,
            revoked_recipients,
            ..
        } => println!(
            "{RED}Unusable{RESET} shared recovery - contains revoked recipient{maybe_plural}: {revoked} ({revoked_len} out of {total} total recipients, with threshold {threshold})",
            maybe_plural = maybe_plural(&(revoked_recipients.len() as u8)),
            revoked = revoked_recipients.iter().join(", "),
            revoked_len = revoked_recipients.len(),
            total = per_recipient_shares.len()
        ),
        libparsec_client::SelfShamirRecoveryInfo::NeverSetup => {
            println!("Shared recovery {RED}never setup{RESET}")
        }
    }

    Ok(())
}

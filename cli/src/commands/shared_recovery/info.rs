use itertools::Itertools;

use crate::{build_main_with_client, utils::*};

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {
    }
);

build_main_with_client!(main, list_shared_recovery);

pub async fn list_shared_recovery(_args: Args, client: &StartedClient) -> anyhow::Result<()> {
    {
        let mut spinner = start_spinner("Poll server for new certificates".into());
        client.poll_server_for_new_certificates().await?;
        spinner.stop_with_symbol(GREEN_CHECKMARK);
    }

    let info = client.get_self_shamir_recovery().await?;

    match info {
                libparsec_client::SelfShamirRecoveryInfo::Deleted {
                    deleted_on,
                    deleted_by,
                    ..
                } => println!("{RED}Deleted{RESET} shared recovery - deleted by {deleted_by} on {deleted_on}"),
                libparsec_client::SelfShamirRecoveryInfo::SetupAllValid {
                    per_recipient_shares, threshold, ..
                } => println!("Shared recovery {GREEN}set up{RESET} with threshold {threshold}\n{}", per_recipient_shares.iter().map(|(recipient, share)| {
                    format!("• User {recipient} has {share} share(s)") // TODO: special case if there is only on share
                }).join("\n")),
                libparsec_client::SelfShamirRecoveryInfo::SetupWithRevokedRecipients {
                    threshold,
                    per_recipient_shares,
                    revoked_recipients,
                    ..
                } => println!(
                    "Shared recovery {YELLOW}set up{RESET} - contains revoked recipients: {revoked} ({revoked_len} out of {total} total recipients, with threshold {threshold})",
                    revoked = revoked_recipients.iter().join(", "),
                    revoked_len = revoked_recipients.len(),
                    total = per_recipient_shares.len()),
                libparsec_client::SelfShamirRecoveryInfo::SetupButUnusable {
                    threshold,
                    per_recipient_shares,
                    revoked_recipients,
                    ..
                } => println!("{RED}Unusable{RESET} shared recovery - contains revoked recipients: {revoked} ({revoked_len} out of {total} total recipients, with threshold {threshold})",
                    revoked = revoked_recipients.iter().join(", "),
                    revoked_len = revoked_recipients.len(),
                    total = per_recipient_shares.len()),
                libparsec_client::SelfShamirRecoveryInfo::NeverSetup => println!("Shared recovery {RED}never setup{RESET}"),
            }

    Ok(())
}

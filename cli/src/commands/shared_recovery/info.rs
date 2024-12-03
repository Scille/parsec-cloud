use itertools::Itertools;

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {
    }
);

pub async fn main(args: Args) -> anyhow::Result<()> {
    let Args {
        password_stdin,
        device,
        config_dir,
    } = args;

    let client = load_client(&config_dir, device, password_stdin).await?;

    {
        let _spinner = start_spinner("Poll server for new certificates".into());
        client.poll_server_for_new_certificates().await?;
    }

    let info = client.get_self_shamir_recovery().await?;

    client.stop().await;

    match info {
                libparsec_client::SelfShamirRecoveryInfo::Deleted {
                    deleted_on,
                    deleted_by,
                    ..
                } => println!("{RED}Deleted{RESET} shared recovery - deleted by {deleted_by} on {deleted_on}"),
                libparsec_client::SelfShamirRecoveryInfo::SetupAllValid {
                    ..
                } => println!("Shared recovery {GREEN}set up{RESET}"),
                libparsec_client::SelfShamirRecoveryInfo::SetupWithRevokedRecipients {
                    threshold,
                    per_recipient_shares,
                    revoked_recipients,
                    ..
                } => println!("Shared recovery for {YELLOW}set up{RESET} - contains revoked recipients: {} ({} out of {} total recipients, with threshold {threshold})", revoked_recipients.iter().join(","), revoked_recipients.len(), per_recipient_shares.len()),
                libparsec_client::SelfShamirRecoveryInfo::SetupButUnusable {
                    threshold,
                    per_recipient_shares,
                    revoked_recipients,
                    ..
                } => println!("{RED}Unusable{RESET} shared recovery - contains revoked recipients: {} ({} out of {} total recipients, with threshold {threshold})", revoked_recipients.iter().join(","), revoked_recipients.len(), per_recipient_shares.len()),
                libparsec_client::SelfShamirRecoveryInfo::NeverSetup => println!("Shared recovery {GREEN}setup{RESET}"),
            }

    Ok(())
}

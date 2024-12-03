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

    let res = client.list_shamir_recoveries_for_others().await?;

    client.stop().await;

    if res.is_empty() {
        println!("No shared recovery found");
    } else {
        let n = res.len();
        println!("Found {GREEN}{n}{RESET} user(s):");
        for info in res {
            match info {
                libparsec_client::OtherShamirRecoveryInfo::Deleted {
                    user_id,
                    deleted_on,
                    deleted_by,
                    ..
                } => println!("Deleted shared recovery for {RED}{user_id}{RESET} - deleted by {deleted_by} on {deleted_on}"),
                libparsec_client::OtherShamirRecoveryInfo::SetupAllValid {
                    user_id,..
                } => println!("Shared recovery for {GREEN}{user_id}{RESET}"),
                libparsec_client::OtherShamirRecoveryInfo::SetupWithRevokedRecipients {
                    user_id,
                    threshold,
                    per_recipient_shares,
                    revoked_recipients,..
                } => println!("Shared recovery for {YELLOW}{user_id}{RESET} - contains revoked recipients: {} ({} out of {} total recipients, with threshold {threshold})", revoked_recipients.iter().join(","), revoked_recipients.len(), per_recipient_shares.len()),
                libparsec_client::OtherShamirRecoveryInfo::SetupButUnusable {
                    user_id,
                    threshold,
                    per_recipient_shares,
                    revoked_recipients,..
                } => println!("Unusable shared recovery for {RED}{user_id}{RED} - contains revoked recipients: {} ({} out of {} total recipients, with threshold {threshold})", revoked_recipients.iter().join(","), revoked_recipients.len(), per_recipient_shares.len()),
            }
        }
    }

    Ok(())
}

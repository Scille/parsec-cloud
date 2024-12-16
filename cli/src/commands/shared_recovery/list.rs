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
    {
        let mut spinner = start_spinner("Poll server for new certificates".into());
        client.poll_server_for_new_certificates().await?;
        spinner.stop_with_symbol(GREEN_CHECKMARK);
    }

    let res = client.list_shamir_recoveries_for_others().await?;
    let users = client.list_users(false, None, None).await?;
    let users: HashMap<_, _> = users.iter().map(|info| (info.id, info)).collect();

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
                } => println!("• Deleted shared recovery for {RED}{user_id}{RESET} - deleted by {deleted_by} on {deleted_on}"),
                libparsec_client::OtherShamirRecoveryInfo::SetupAllValid {
                    user_id, threshold, per_recipient_shares,..
                } => println!("Shared recovery for {GREEN}{}{RESET} with threshold {threshold}\n{}", users.get(&user_id).expect("missing author").human_handle, per_recipient_shares.iter().map(|(recipient, share)| {
                    // this means that a user disappeared completely, it should not happen
                    let user= &users.get(recipient).expect("missing recipient").human_handle;
                    format!("\t• User {user} has {share} share(s)", // TODO: special case if there is only on share
                )
                }).join("\n")),
                libparsec_client::OtherShamirRecoveryInfo::SetupWithRevokedRecipients {
                    user_id,
                    threshold,
                    per_recipient_shares,
                    revoked_recipients,..
                } => println!("• Shared recovery for {YELLOW}{user_id}{RESET} - contains revoked recipients: {revoked} ({revoked_len} out of {total} total recipients, with threshold {threshold})",
                    revoked = revoked_recipients.iter().join(", "),
                    revoked_len = revoked_recipients.len(),
                    total = per_recipient_shares.len()),
                libparsec_client::OtherShamirRecoveryInfo::SetupButUnusable {
                    user_id,
                    threshold,
                    per_recipient_shares,
                    revoked_recipients,..
                } => println!("• Unusable shared recovery for {RED}{user_id}{RESET} - contains revoked recipients: {revoked} ({revoked_len} out of {total} total recipients, with threshold {threshold})",
                    revoked = revoked_recipients.iter().join(", "),
                    revoked_len = revoked_recipients.len(),
                    total = per_recipient_shares.len()),
            }
        }
    }

    Ok(())
}

use std::{collections::HashMap, path::PathBuf};

use clap::Args;
use libparsec::{get_default_config_dir, UserID, UserProfile};

use crate::utils::{load_client_and_run, start_spinner};

#[derive(Args)]
pub struct ShamirSetupCreate {
    /// Parsec config directory
    #[arg(short, long, default_value_os_t = get_default_config_dir())]
    config_dir: PathBuf,
    /// Device ID
    #[arg(short, long)]
    device: Option<String>,
    /// Share recipients, if missing organization's admins will be used instead
    /// Author must not be included as recipient.
    /// User email is expected
    #[arg(short, long,  num_args = 1..)]
    recipients: Option<Vec<String>>,
    /// Share weights. Requires Share recipient list.
    /// Must have the same length as recipients.
    /// Defaults to one per recipient.
    #[arg(short, long, requires = "recipients",  num_args = 1..)]
    weights: Option<Vec<u8>>,
    /// Threshold number of shares required to proceed with recovery.
    /// Default to sum of weights. Must be lesser or equal to sum of weights.
    #[arg(short, long)]
    threshold: Option<u8>,
}

pub async fn shamir_setup_create(shamir_setup: ShamirSetupCreate) -> anyhow::Result<()> {
    let ShamirSetupCreate {
        config_dir,
        device,
        recipients,
        weights,
        threshold,
    } = shamir_setup;
    load_client_and_run(config_dir, device, |client| async move {
        let mut handle = start_spinner("Creating shamir setup".into());

        let users = client.list_users(true, None, None).await?;
        let recipients_ids: Vec<_> = if let Some(recipients) = recipients {
            let recipient_info: HashMap<_, _> = users
                .iter()
                .filter(|info| recipients.contains(&info.human_handle.email().to_owned()))
                .map(|info| (info.human_handle.email().to_owned(), info.id))
                .collect();
            if recipient_info.len() != recipients.len() {
                handle.stop_with_message("A user in missing".into());
                client.stop().await;
                return Ok(());
            }
            recipients
                .iter()
                .map(|human| *recipient_info.get(human).expect("human should be here"))
                .collect()
        } else {
            users
                .iter()
                .filter(|info| info.current_profile == UserProfile::Admin)
                .map(|info| info.id)
                .collect()
        };

        // TODO check that author is not in recipients

        let shares: HashMap<UserID, u8> = if let Some(weights) = weights {
            if recipients_ids.len() != weights.len() {
                handle.stop_with_message("incoherent weights count".into());
                client.stop().await;
                return Ok(());
            }

            recipients_ids
                .into_iter()
                .zip(weights.into_iter())
                .collect()
        } else {
            recipients_ids.into_iter().map(|r| (r, 1)).collect()
        };

        let t = shares.values().sum();
        if let Some(threshold) = threshold {
            if threshold > t {
                handle.stop_with_message("too big threshold".into());
                client.stop().await;
                return Ok(());
            }
        }

        client
            .shamir_setup_create(shares, threshold.unwrap_or(t))
            .await?;

        handle.stop_with_message("Shamir setup has been created".into());

        client.stop().await;

        Ok(())
    })
    .await
}

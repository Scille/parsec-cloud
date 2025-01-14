use std::{collections::HashMap, num::NonZeroU8};

use dialoguer::Input;
use libparsec::{UserID, UserProfile};

use crate::utils::{
    poll_server_for_new_certificates, start_spinner, StartedClient, GREEN_CHECKMARK,
};

// TODO: should provide the recipients and their share count as a single parameter
//       e.g. `--recipients=foo@example.com=2,bar@example.com=3`
crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {
        /// Share recipients, if missing organization's admins will be used instead
        /// Author must not be included as recipient.
        /// User email is expected.
        #[arg(short, long, num_args = 1..=255)]
        recipients: Option<Vec<String>>,
        /// Share weights. Requires Share recipient list.
        /// Must have the same length as recipients.
        /// Defaults to one per recipient.
        #[arg(short, long, requires = "recipients", num_args = 1..=255)]
        weights: Option<Vec<NonZeroU8>>,
        /// Threshold number of shares required to proceed with recovery.
        #[arg(short, long, requires = "recipients")]
        threshold: Option<NonZeroU8>,
    }
);

crate::build_main_with_client!(main, create_shared_recovery);

pub async fn create_shared_recovery(args: Args, client: &StartedClient) -> anyhow::Result<()> {
    let Args {
        recipients,
        weights,
        threshold,
        ..
    } = args;

    poll_server_for_new_certificates(client).await?;

    let mut handle = start_spinner("Creating shared recovery setup".into());
    let users = client.list_users(true, None, None).await?;
    let recipients_ids: Vec<_> = if let Some(recipients) = recipients {
        let recipient_info: HashMap<_, _> = users
            .iter()
            .filter(|info| recipients.contains(&info.human_handle.email().to_owned()))
            .map(|info| (info.human_handle.email().to_owned(), info.id))
            .collect();
        if recipient_info.len() != recipients.len() {
            return Err(anyhow::anyhow!("A user is missing"));
        }
        recipients
            .iter()
            .map(|human| *recipient_info.get(human).expect("human should be here"))
            .collect()
    } else {
        users
            .iter()
            .filter(|info| {
                info.current_profile == UserProfile::Admin && info.id != client.user_id()
            })
            .map(|info| info.id)
            .collect()
    };

    let per_recipient_shares: HashMap<UserID, NonZeroU8> = if let Some(weights) = weights {
        if recipients_ids.len() != weights.len() {
            return Err(anyhow::anyhow!("incoherent weights count"));
        }

        recipients_ids
            .into_iter()
            .zip(weights.into_iter())
            .collect()
    } else {
        let weight = NonZeroU8::new(1).expect("always valid");
        recipients_ids
            .into_iter()
            .map(|user_id| (user_id, weight))
            .collect()
    };
    // we must stop the handle here to avoid messing up with the threshold choice
    handle.stop_with_symbol("..."); // not green check mark because it's not finished
    let threshold = if let Some(t) = threshold {
        t
    } else {
        // note that this is a blocking call
        Input::<NonZeroU8>::new()
        .with_prompt(format!(
            "Choose a threshold between 1 and {}\nThe threshold is the minimum number of recipients that one must gather to recover the account",
            per_recipient_shares.len()
        )) .interact_text()?
    };
    let mut handle = start_spinner("Creating shared recovery setup".into());

    client
        .setup_shamir_recovery(per_recipient_shares, threshold)
        .await?;

    handle.stop_with_message(format!(
        "{GREEN_CHECKMARK} Shared recovery setup has been created"
    ));

    Ok(())
}

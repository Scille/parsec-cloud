use std::{collections::HashMap, fmt::Write as _, io::Write as _, num::NonZeroU8};

use dialoguer::{Confirm, Input};
use libparsec::{EmailAddress, UserID, UserProfile};

use crate::{
    ui::Color,
    utils::{
        maybe_plural, poll_server_for_new_certificates, StartedClient, BULLET_CHAR, CHECKMARK,
    },
};

const ONE: NonZeroU8 = NonZeroU8::new(1).expect("always valid");

#[derive(Debug, Clone, PartialEq)]
pub struct WeightedEmail {
    pub email: EmailAddress,
    pub weight: NonZeroU8,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum WeightedEmailParseError {
    #[error("Invalid email address")]
    InvalidEmail,
    #[error("Invalid weight")]
    InvalidWeight,
}

impl std::str::FromStr for WeightedEmail {
    type Err = WeightedEmailParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (email, weight) = match s.split_once(':') {
            Some((email, weight)) => (
                email,
                weight
                    .parse::<NonZeroU8>()
                    .map_err(|_| WeightedEmailParseError::InvalidWeight)?,
            ),
            None => (s, ONE), // default weight is 1
        };

        let email = email
            .parse::<EmailAddress>()
            .map_err(|_| WeightedEmailParseError::InvalidEmail)?;

        Ok(WeightedEmail { email, weight })
    }
}

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin, force]
    pub struct Args {
        /// Share recipients, with their weight.
        /// If missing organization's admins will be used instead.
        /// Author must not be included as recipient.
        /// email or email:weight format is expected (weight defaults to one if not provided).
        #[arg(long, num_args = 1..=255, value_hint = clap::ValueHint::EmailAddress)]
        recipients: Option<Vec<WeightedEmail>>,
        /// Threshold number of shares required to proceed with recovery.
        #[arg(short, long, requires = "recipients")]
        threshold: Option<NonZeroU8>,
    }
);

crate::build_main_with_client!(main, create_shared_recovery);

pub async fn create_shared_recovery(
    ui: crate::Ui,
    args: Args,
    client: &StartedClient,
) -> anyhow::Result<()> {
    let Args {
        recipients,
        threshold,
        force,
        ..
    } = args;

    poll_server_for_new_certificates(&ui, client).await?;

    let handle = ui.with_spinner(|_, out| write!(out, "Creating shared recovery setup"))?;
    let users = client.list_users(true, None, None).await?;

    let per_recipient_shares: HashMap<UserID, NonZeroU8> = if let Some(recipients) = recipients {
        let users_ids: HashMap<_, _> = users
            .iter()
            .map(|info| (info.human_handle.email().to_owned(), info.id))
            .collect();

        recipients
            .iter()
            .map(|recipient| {
                users_ids
                    .get(&recipient.email)
                    .copied()
                    .ok_or_else(|| {
                        anyhow::anyhow!("A user with email {} not found", recipient.email)
                    })
                    .map(|id| (id, recipient.weight))
            })
            .collect::<Result<HashMap<_, _>, _>>()?
    } else {
        let admins: HashMap<_, _> = users
            .iter()
            .filter(|info| {
                info.current_profile == UserProfile::Admin && info.id != client.user_id()
            })
            .map(|info| (info.id, ONE))
            .collect();
        anyhow::ensure!(!admins.is_empty(), "No default recipient available");
        admins
    };

    // we must stop the handle here to avoid messing up with the threshold choice
    handle.stop_with_symbol(|_, out| write!(out, "..."))?; // not green check mark because it's not finished
    let threshold = if let Some(t) = threshold {
        t
    } else {
        ui.with_message(|_, out| writeln!(out, "The threshold is the minimum number of shares that one must gather to recover the account"))?;
        // note that this is a blocking call
        Input::<NonZeroU8>::new()
            .with_prompt(format!(
                "Choose a threshold between 1 and {}",
                per_recipient_shares.len()
            ))
            .interact_text()?
    };

    ui.with_message(|_, out| {
        writeln!(out, "The following shared recovery setup with a threshold set to {threshold} will be created:")?;
        per_recipient_shares
            .iter()
            .try_for_each(|(recipient, share)| {
                let user = &users
                    .iter()
                    .find(|x| x.id == *recipient)
                    .expect("missing recipient")
                    .human_handle;
                writeln!(out, "{BULLET_CHAR} User {user} will have {share} share{}", maybe_plural(share.get()))
            })
    })?;

    if !force
        && !Confirm::new()
            .with_prompt("Do you want to proceed?")
            .interact()?
    {
        ui.with_message(|_, out| writeln!(out, "Shared recovery creation aborted."))?;
        return Ok(());
    }

    let handle = ui.with_spinner(|_, out| write!(out, "Creating shared recovery setup"))?;

    client
        .setup_shamir_recovery(per_recipient_shares, threshold)
        .await?;

    handle.stop_with(|fmt, out| {
        write!(
            out,
            "{} Shared recovery setup has been created",
            fmt.wrap_in_color(Color::Green, CHECKMARK)
        )
    })?;

    Ok(())
}

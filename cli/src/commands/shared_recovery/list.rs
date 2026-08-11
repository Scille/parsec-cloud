use std::{collections::HashMap, io::Write as _};

use itertools::Itertools;

use libparsec_client::OtherShamirRecoveryInfo;
use serde::ser::SerializeStruct;

use crate::{
    build_main_with_client,
    ui::{CLIDisplay, Color},
    utils::*,
};

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {
    }
);

build_main_with_client!(main, shared_recovery_list);

pub async fn shared_recovery_list(
    ui: crate::Ui,
    _args: Args,
    client: &StartedClient,
) -> anyhow::Result<()> {
    poll_server_for_new_certificates(&ui, client).await?;

    let users = client.list_users(false, None, None).await?;
    let users: HashMap<_, _> = users.iter().map(|info| (info.id, info)).collect();
    let res = client
        .list_shamir_recoveries_for_others()
        .await?
        .into_iter()
        .map(|v| OtherShamirRecoveryInfoDisplay {
            info: v,
            users: &users,
        })
        .collect::<Vec<_>>();

    if res.is_empty() {
        ui.with_message(|_, out| writeln!(out, "No shared recovery found"))?;
    } else {
        ui.with_message(|fmt, out| {
            writeln!(
                out,
                "Found {} user(s):",
                fmt.wrap_in_color(Color::Green, res.len())
            )
        })?;
        ui.data_print(&res.as_slice())?;
    }

    Ok(())
}

struct OtherShamirRecoveryInfoDisplay<'a> {
    info: libparsec_client::OtherShamirRecoveryInfo,
    users: &'a HashMap<libparsec_types::UserID, &'a libparsec_client::UserInfo>,
}

impl<'a> serde::Serialize for OtherShamirRecoveryInfoDisplay<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        const NAME: &str = "SelfShamirRecoveryInfo";
        const STATUS: &str = "status";
        const CREATED_ON: &str = "created_on";
        const CREATED_BY: &str = "created_by";
        const THRESHOLD: &str = "threshold";
        const PER_RECIPIENT_SHARES: &str = "per_recipient_shares";
        const DELETED_ON: &str = "deleted_on";
        const DELETED_BY: &str = "deleted_by";
        const REVOKED_RECIPIENTS: &str = "revoked_recipients";
        const USER_ID: &str = "user_id";
        let s = match &self.info {
            OtherShamirRecoveryInfo::Deleted {
                created_on,
                created_by,
                threshold,
                per_recipient_shares,
                deleted_on,
                deleted_by,
                user_id,
            } => {
                let mut s = serializer.serialize_struct(NAME, 8)?;
                s.serialize_field(STATUS, "deleted")?;
                s.serialize_field(CREATED_ON, created_on)?;
                s.serialize_field(CREATED_BY, created_by)?;
                s.serialize_field(THRESHOLD, threshold)?;
                s.serialize_field(PER_RECIPIENT_SHARES, per_recipient_shares)?;
                s.serialize_field(DELETED_ON, deleted_on)?;
                s.serialize_field(DELETED_BY, deleted_by)?;
                s.serialize_field(USER_ID, user_id)?;
                s
            }
            OtherShamirRecoveryInfo::SetupAllValid {
                created_on,
                created_by,
                threshold,
                per_recipient_shares,
                user_id,
            } => {
                let mut s = serializer.serialize_struct(NAME, 6)?;
                s.serialize_field(STATUS, "ok")?;
                s.serialize_field(CREATED_ON, created_on)?;
                s.serialize_field(CREATED_BY, created_by)?;
                s.serialize_field(THRESHOLD, threshold)?;
                s.serialize_field(PER_RECIPIENT_SHARES, per_recipient_shares)?;
                s.serialize_field(USER_ID, user_id)?;
                s
            }
            OtherShamirRecoveryInfo::SetupWithRevokedRecipients {
                created_on,
                created_by,
                threshold,
                per_recipient_shares,
                revoked_recipients,
                user_id,
            } => {
                let mut s = serializer.serialize_struct(NAME, 7)?;
                s.serialize_field(STATUS, "with-revoked-recipients")?;
                s.serialize_field(CREATED_ON, created_on)?;
                s.serialize_field(CREATED_BY, created_by)?;
                s.serialize_field(THRESHOLD, threshold)?;
                s.serialize_field(PER_RECIPIENT_SHARES, per_recipient_shares)?;
                s.serialize_field(REVOKED_RECIPIENTS, revoked_recipients)?;
                s.serialize_field(USER_ID, user_id)?;
                s
            }
            OtherShamirRecoveryInfo::SetupButUnusable {
                created_on,
                created_by,
                threshold,
                per_recipient_shares,
                revoked_recipients,
                user_id,
            } => {
                let mut s = serializer.serialize_struct(NAME, 7)?;
                s.serialize_field(STATUS, "unusable")?;
                s.serialize_field(CREATED_ON, created_on)?;
                s.serialize_field(CREATED_BY, created_by)?;
                s.serialize_field(THRESHOLD, threshold)?;
                s.serialize_field(PER_RECIPIENT_SHARES, per_recipient_shares)?;
                s.serialize_field(REVOKED_RECIPIENTS, revoked_recipients)?;
                s.serialize_field(USER_ID, user_id)?;
                s
            }
        };
        s.end()
    }
}

impl<'a> CLIDisplay for OtherShamirRecoveryInfoDisplay<'a> {
    fn plain_write<W: std::io::prelude::Write>(
        &self,
        fmt: &crate::ui::ColorFormatter,
        mut w: W,
    ) -> std::io::Result<()> {
        match &self.info {
                OtherShamirRecoveryInfo::Deleted {
                    user_id,
                    deleted_on,
                    deleted_by,
                    ..
                } => write!(
                    w,
                    "Deleted shared recovery for {user_id} - deleted by {deleted_by} on {deleted_on}",
                    user_id = fmt.wrap_in_color(Color::Red, user_id)
                ),
                OtherShamirRecoveryInfo::SetupAllValid {
                    user_id, threshold, per_recipient_shares,..
                } => {
                    let name = if let Some(user) = self.users.get(user_id) {
                        format_args!("{}", user.human_handle)
                    } else {
                        format_args!("uid={user_id}")
                    };
                    writeln!(
                        w,
                        "Shared recovery for {} with threshold {threshold}",
                        fmt.wrap_in_color(Color::Green, name)
                    )?;

                    per_recipient_shares.iter().try_for_each(|(recipient, share)| {
                        // this means that a user disappeared completely, it should not happen
                        let user = self.users.get(recipient);
                        let name = if let Some(user) = user {
                            format_args!("{}", user.human_handle)
                        } else {
                            format_args!("uid={recipient}")
                        };
                        writeln!(w,"\t- User {name} has {share} share{}",  maybe_plural(share.get()))
                    })
                },
                OtherShamirRecoveryInfo::SetupWithRevokedRecipients {
                    user_id,
                    threshold,
                    per_recipient_shares,
                    revoked_recipients,..
                } => write!(
                    w,
                    "Shared recovery for {user_id} - contains revoked recipient{maybe_plural}: {revoked} ({revoked_len} out of {total} total recipients, with threshold {threshold})",
                    user_id = fmt.wrap_in_color(Color::Yellow, user_id),
                    maybe_plural = maybe_plural(revoked_recipients.len() as u8),
                    revoked = revoked_recipients.iter().join(", "),
                    revoked_len = revoked_recipients.len(),
                    total = per_recipient_shares.len()
                ),
                OtherShamirRecoveryInfo::SetupButUnusable {
                    user_id,
                    threshold,
                    per_recipient_shares,
                    revoked_recipients,..
                } => write!(
                    w,
                    "Unusable shared recovery for {user_id} - contains revoked recipient{maybe_plural}: {revoked} ({revoked_len} out of {total} total recipients, with threshold {threshold})",
                    user_id = fmt.wrap_in_color(Color::Red, user_id),
                    maybe_plural = maybe_plural(revoked_recipients.len() as u8),
                    revoked = revoked_recipients.iter().join(", "),
                    revoked_len = revoked_recipients.len(),
                    total = per_recipient_shares.len()
                ),
            }
    }
}

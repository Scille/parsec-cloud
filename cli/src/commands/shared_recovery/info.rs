use std::collections::HashMap;

use itertools::Itertools;
use serde::ser::SerializeStruct;

use libparsec_client::SelfShamirRecoveryInfo;

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

build_main_with_client!(main, shared_recovery_info);

pub async fn shared_recovery_info(
    ui: crate::Ui,
    _args: Args,
    client: &StartedClient,
) -> anyhow::Result<()> {
    poll_server_for_new_certificates(&ui, client).await?;

    let info = client.get_self_shamir_recovery().await?;
    let users = client.list_users(false, None, None).await?;
    let users: HashMap<_, _> = users.iter().map(|info| (info.id, info)).collect();

    ui.data_print(&SelfShamirRecoveryInfoDisplay { info, users })?;

    Ok(())
}

struct SelfShamirRecoveryInfoDisplay<'a> {
    info: libparsec_client::SelfShamirRecoveryInfo,
    users: HashMap<libparsec_types::UserID, &'a libparsec_client::UserInfo>,
}

impl<'a> serde::Serialize for SelfShamirRecoveryInfoDisplay<'a> {
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
        let s = match &self.info {
            SelfShamirRecoveryInfo::NeverSetup => {
                let mut s = serializer.serialize_struct(NAME, 1)?;
                s.serialize_field(STATUS, "never-setup")?;
                s
            }
            SelfShamirRecoveryInfo::Deleted {
                created_on,
                created_by,
                threshold,
                per_recipient_shares,
                deleted_on,
                deleted_by,
            } => {
                let mut s = serializer.serialize_struct(NAME, 7)?;
                s.serialize_field(STATUS, "deleted")?;
                s.serialize_field(CREATED_ON, created_on)?;
                s.serialize_field(CREATED_BY, created_by)?;
                s.serialize_field(THRESHOLD, threshold)?;
                s.serialize_field(PER_RECIPIENT_SHARES, per_recipient_shares)?;
                s.serialize_field(DELETED_ON, deleted_on)?;
                s.serialize_field(DELETED_BY, deleted_by)?;
                s
            }
            SelfShamirRecoveryInfo::SetupAllValid {
                created_on,
                created_by,
                threshold,
                per_recipient_shares,
            } => {
                let mut s = serializer.serialize_struct(NAME, 5)?;
                s.serialize_field(STATUS, "ok")?;
                s.serialize_field(CREATED_ON, created_on)?;
                s.serialize_field(CREATED_BY, created_by)?;
                s.serialize_field(THRESHOLD, threshold)?;
                s.serialize_field(PER_RECIPIENT_SHARES, per_recipient_shares)?;
                s
            }
            SelfShamirRecoveryInfo::SetupWithRevokedRecipients {
                created_on,
                created_by,
                threshold,
                per_recipient_shares,
                revoked_recipients,
            } => {
                let mut s = serializer.serialize_struct(NAME, 6)?;
                s.serialize_field(STATUS, "with-revoked-recipients")?;
                s.serialize_field(CREATED_ON, created_on)?;
                s.serialize_field(CREATED_BY, created_by)?;
                s.serialize_field(THRESHOLD, threshold)?;
                s.serialize_field(PER_RECIPIENT_SHARES, per_recipient_shares)?;
                s.serialize_field(REVOKED_RECIPIENTS, revoked_recipients)?;
                s
            }
            SelfShamirRecoveryInfo::SetupButUnusable {
                created_on,
                created_by,
                threshold,
                per_recipient_shares,
                revoked_recipients,
            } => {
                let mut s = serializer.serialize_struct(NAME, 6)?;
                s.serialize_field(STATUS, "unusable")?;
                s.serialize_field(CREATED_ON, created_on)?;
                s.serialize_field(CREATED_BY, created_by)?;
                s.serialize_field(THRESHOLD, threshold)?;
                s.serialize_field(PER_RECIPIENT_SHARES, per_recipient_shares)?;
                s.serialize_field(REVOKED_RECIPIENTS, revoked_recipients)?;
                s
            }
        };
        s.end()
    }
}

impl<'a> CLIDisplay for SelfShamirRecoveryInfoDisplay<'a> {
    fn plain_write<W: std::io::prelude::Write>(
        &self,
        fmt: &crate::ui::ColorFormatter,
        mut w: W,
    ) -> std::io::Result<()> {
        match &self.info {
            SelfShamirRecoveryInfo::NeverSetup => write!(
                w,
                "Shared recovery {}",
                fmt.wrap_in_color(Color::Red, "never setup")
            ),
            SelfShamirRecoveryInfo::Deleted {
                deleted_on,
                deleted_by,
                ..
            } => write!(
                w,
                "{} shared recovery - deleted by {deleted_by} on {deleted_on}",
                fmt.wrap_in_color(Color::Red, "Deleted")
            ),
            SelfShamirRecoveryInfo::SetupAllValid {
                threshold,
                per_recipient_shares,
                ..
            } => {
                writeln!(w, "Shared recovery {} with threshold {threshold}", fmt.wrap_in_color(Color::Green, "set up"))?;

                per_recipient_shares.iter().try_for_each(|(recipient, share)| {
                    let user = self.users.get(recipient);
                    // this means that a user disappeared completely, it should not happen
                    let name = if let Some(user) = user {
                        format_args!("{}", user.human_handle)
                    } else {
                        format_args!("uid={recipient}")
                    };
                    writeln!(w, "{BULLET_CHAR} User {name} has {share} share{plural}", plural=maybe_plural(share.get()))
                })
            },
            SelfShamirRecoveryInfo::SetupWithRevokedRecipients {
                threshold,
                per_recipient_shares,
                revoked_recipients,
                ..
            } => write!(w,
                 "Shared recovery {} - contains revoked recipient{maybe_plural}: {revoked} ({revoked_len} ouf of {total} total recipients, with threshold {threshold})",
                 fmt.wrap_in_color(Color::Yellow, "set up"),
                 maybe_plural = maybe_plural(revoked_recipients.len() as u8),
                 revoked = revoked_recipients.iter().join(", "),
                 revoked_len = revoked_recipients.len(),
                 total = per_recipient_shares.len()
            ),
            SelfShamirRecoveryInfo::SetupButUnusable {
                threshold,
                per_recipient_shares,
                revoked_recipients,
                ..
            } => write!(
                w,
                "{} shared recovery - contains revoked recipient{maybe_plural}: {revoked} ({revoked_len} out of {total} total recipients, with threshold {threshold})",
                fmt.wrap_in_color(Color::Red, "Unusable"),
                maybe_plural = maybe_plural(revoked_recipients.len() as u8),
                revoked = revoked_recipients.iter().join(", "),
                revoked_len = revoked_recipients.len(),
                total = per_recipient_shares.len()
            ),
        }
    }
}

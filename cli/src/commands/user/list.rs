// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
use std::io::Write;

use serde::ser::SerializeStruct;

use crate::{
    ui::{CLIDisplay, Color},
    utils::*,
};

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {
        /// Skip revoked users
        #[arg(short, long, default_value_t)]
        skip_revoked: bool,
    }
);

crate::build_main_with_client!(main, list_user);

pub async fn list_user(ui: crate::Ui, args: Args, client: &StartedClient) -> anyhow::Result<()> {
    let Args { skip_revoked, .. } = args;
    log::trace!("Listing users (skip_revoked={skip_revoked})");

    poll_server_for_new_certificates(&ui, client).await?;
    let users = client
        .list_users(skip_revoked, None, None)
        .await?
        .into_iter()
        .map(UserInfoDisplay)
        .collect::<Vec<_>>();

    if users.is_empty() {
        ui.with_message(|_, out| writeln!(out, "No users found"))?;
    } else {
        ui.with_message(|fmt, out| {
            writeln!(
                out,
                "Found {} user(s):",
                fmt.wrap_in_color(Color::Green, users.len())
            )
        })?;
        ui.data_print(&users.as_slice())?;
    }

    Ok(())
}

struct UserInfoDisplay(libparsec_client::UserInfo);

impl serde::Serialize for UserInfoDisplay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("UserInfo", 7)?;
        s.serialize_field("id", &self.0.id)?;
        s.serialize_field("human_handle", &self.0.human_handle)?;
        s.serialize_field("current_profile", &self.0.current_profile)?;
        s.serialize_field("created_on", &self.0.created_on)?;
        s.serialize_field("created_by", &self.0.created_by)?;
        s.serialize_field("revoked_on", &self.0.revoked_on)?;
        s.serialize_field("revoked_by", &self.0.revoked_by)?;
        s.end()
    }
}

impl CLIDisplay for UserInfoDisplay {
    fn plain_write<W: Write>(
        &self,
        fmt: &crate::ui::ColorFormatter,
        mut w: W,
    ) -> std::io::Result<()> {
        let id = self.0.id;
        let human_handle = &self.0.human_handle;
        let profile = &self.0.current_profile;
        let prefix = format_args!(
            "{id} - {human_handle}: {profile}",
            id = fmt.wrap_in_color(Color::Yellow, id)
        );
        let zipped_revoke = self.0.revoked_on.zip(self.0.revoked_by);
        if let Some((revoked_on, revoked_by)) = zipped_revoke.as_ref() {
            write!(w, "{prefix} | revoked on {revoked_on} by {revoked_by}")
        } else {
            w.write_fmt(prefix)
        }
    }
}

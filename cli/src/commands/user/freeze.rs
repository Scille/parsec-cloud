// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
use std::fmt::Write as _;

use libparsec::{EmailAddress, OrganizationID, ParsecAddr, UserID};

use crate::{
    ui::{CLIDisplay, Color},
    utils::make_checkmark_symbol,
};
use serde::{Deserialize, Serialize};

crate::clap_parser_with_shared_opts_builder!(
    #[with = addr, token, organization]
    #[command(group(clap::ArgGroup::new("user_ref").required(true)))]
    pub struct Args {
        /// User ID (hex) of the user to reset TOTP for (mutually exclusive with --user-email)
        #[arg(long, group = "user_ref", value_parser = UserID::from_hex, value_hint = clap::ValueHint::Other,  conflicts_with = "user_email")]
        user_id: Option<UserID>,
        /// Email of the user to reset TOTP for (mutually exclusive with --user-id)
        #[arg(long, group = "user_ref", value_hint = clap::ValueHint::EmailAddress)]
        user_email: Option<EmailAddress>,
        /// Unfreeze the user
        #[arg(long)]
        unfreeze: bool,
    }
);

#[derive(Serialize, Deserialize)]
struct FreezeRepOk {
    user_id: String,
    user_email: String,
    user_name: String,
    frozen: bool,
}

#[derive(Deserialize)]
struct FreezeRepErr {
    detail: String,
}

async fn user_freeze_req(
    organization_id: &OrganizationID,
    addr: &ParsecAddr,
    administration_token: &str,
    user_id: Option<UserID>,
    user_email: Option<EmailAddress>,
    unfreeze: bool,
) -> anyhow::Result<FreezeRepOk> {
    let url = addr.to_http_url(Some(&format!(
        "/administration/organizations/{organization_id}/users/freeze"
    )));

    let mut body = serde_json::Map::new();
    if let Some(user_id) = user_id {
        body.insert("user_id".into(), serde_json::Value::String(user_id.hex()));
    } else if let Some(email) = user_email {
        body.insert(
            "user_email".into(),
            serde_json::Value::String(email.to_string()),
        );
    } else {
        return Err(anyhow::anyhow!("Missing user_id or user_email."));
    }

    body.insert("frozen".into(), serde_json::Value::Bool(!unfreeze));

    let client = libparsec_client_connection::build_client()?;
    let rep = client
        .post(url)
        .bearer_auth(administration_token)
        .json(&body)
        .send()
        .await?;

    match rep.status() {
        status if status.is_success() => Ok(rep.json::<FreezeRepOk>().await?),
        bad_status => match rep.json::<FreezeRepErr>().await {
            Ok(body) => Err(anyhow::anyhow!("{}", body.detail)),
            Err(err) => Err(anyhow::anyhow!(
                "Unexpected response from server, status={} ({})",
                bad_status,
                err
            )),
        },
    }
}

pub async fn main(ui: crate::Ui, args: Args) -> anyhow::Result<()> {
    let Args {
        organization,
        token,
        addr,
        user_id,
        user_email,
        unfreeze,
    } = args;

    let handle = ui.with_spinner(|_, out| {
        write!(
            out,
            "{} user",
            if unfreeze { "Unfreezing" } else { "Freezing" }
        )
    })?;

    let res = user_freeze_req(&organization, &addr, &token, user_id, user_email, unfreeze).await?;

    handle.stop_with_symbol(make_checkmark_symbol)?;

    ui.data_print(&res)?;

    Ok(())
}

impl CLIDisplay for FreezeRepOk {
    fn plain_write<W: std::io::prelude::Write>(
        &self,
        fmt: &crate::ui::ColorFormatter,
        mut w: W,
    ) -> std::io::Result<()> {
        write!(
            w,
            "User {} - {} <{}> is {}frozen",
            fmt.wrap_in_color(Color::Yellow, &self.user_id),
            self.user_name,
            self.user_email,
            if self.frozen { "" } else { "not " }
        )
    }
}

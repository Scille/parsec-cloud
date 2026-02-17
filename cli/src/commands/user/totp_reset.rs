// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec::{EmailAddress, OrganizationID, ParsecAddr, UserID};

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = addr, token, organization]
    #[command(group(clap::ArgGroup::new("user_ref").required(true)))]
    pub struct Args {
        /// User ID (hex) of the user to reset TOTP for (mutually exclusive with --user-email)
        #[arg(long, group = "user_ref", value_parser = UserID::from_hex)]
        user_id: Option<UserID>,
        /// Email of the user to reset TOTP for (mutually exclusive with --user-id)
        #[arg(long, group = "user_ref")]
        user_email: Option<EmailAddress>,
        /// Request the server to send the an email informing the user of the reset
        #[arg(long, default_value_t)]
        send_email: bool,
    }
);

#[derive(serde::Deserialize)]
struct ResetTotpRepOk {
    user_id: String,
    user_email: String,
    totp_reset_url: String,
    email_sent_status: String,
}

#[derive(serde::Deserialize)]
struct ResetTotpRepErr {
    detail: String,
}

async fn totp_reset_req(
    organization_id: &OrganizationID,
    addr: &ParsecAddr,
    administration_token: &str,
    user_id: Option<UserID>,
    user_email: Option<EmailAddress>,
    send_email: bool,
) -> anyhow::Result<ResetTotpRepOk> {
    let url = addr.to_http_url(Some(&format!(
        "/administration/organizations/{organization_id}/users/reset_totp"
    )));

    let mut body = serde_json::Map::new();
    if let Some(user_id) = user_id {
        body.insert("user_id".into(), serde_json::Value::String(user_id.hex()));
    }
    if let Some(email) = user_email {
        body.insert(
            "user_email".into(),
            serde_json::Value::String(email.to_string()),
        );
    }
    body.insert("send_email".into(), serde_json::Value::Bool(send_email));

    let client = libparsec_client_connection::build_client()?;
    let rep = client
        .post(url)
        .bearer_auth(administration_token)
        .json(&body)
        .send()
        .await?;

    match rep.status() {
        status if status.is_success() => Ok(rep.json::<ResetTotpRepOk>().await?),
        bad_status => match rep.json::<ResetTotpRepErr>().await {
            Ok(body) => Err(anyhow::anyhow!("{}", body.detail)),
            Err(err) => Err(anyhow::anyhow!(
                "Unexpected response from server, status={} ({})",
                bad_status,
                err
            )),
        },
    }
}

pub async fn main(args: Args) -> anyhow::Result<()> {
    let Args {
        organization,
        token,
        addr,
        user_id,
        user_email,
        send_email,
    } = args;

    let mut handle = start_spinner("Resetting TOTP".into());

    let res = totp_reset_req(
        &organization,
        &addr,
        &token,
        user_id,
        user_email,
        send_email,
    )
    .await?;

    let msg = match res.email_sent_status.as_ref() {
        "NOT_SENT_AS_REQUESTED" => {
            format!(
                "TOTP reset for user {GREEN}{}{RESET}\nReset URL: {YELLOW}{}{RESET}",
                res.user_id, res.totp_reset_url,
            )
        }
        "SENT_AS_REQUESTED" => {
            format!(
                "TOTP reset for user {GREEN}{}{RESET}\n\
                Reset URL: {YELLOW}{}{RESET}\n\
                An email with the reset URL has been sent to {YELLOW}{}{RESET}",
                res.user_id, res.totp_reset_url, res.user_email,
            )
        }
        email_sent_err => {
            format!(
                "TOTP reset for user {GREEN}{}{RESET}\n\
                Reset URL: {YELLOW}{}{RESET}\n\
                Email sending to {YELLOW}{}{RESET} has failed (error: {})",
                res.user_id, res.totp_reset_url, res.user_email, email_sent_err
            )
        }
    };
    handle.stop_with_message(msg);

    Ok(())
}

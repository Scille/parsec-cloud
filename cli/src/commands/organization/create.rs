// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec::{OrganizationID, ParsecAddr, ParsecOrganizationBootstrapAddr};

use crate::utils::*;

crate::clap_parser_with_shared_opts_builder!(
    #[with = addr, token]
    pub struct Args {
        /// The organization name that will be created
        organization: OrganizationID,
    }
);

#[derive(serde::Deserialize)]
#[serde(untagged)]
enum CreateOrganizationRep {
    Ok(CreationOrganizationOk),
    Err(CreationOrganizationErr),
}

#[derive(serde::Deserialize)]
struct CreationOrganizationOk {
    #[serde(deserialize_with = "bootstrap_url_deserialize")]
    bootstrap_url: ParsecOrganizationBootstrapAddr,
    // Note we ignore the field `bootstrap_url_as_http_redirection` here.
    // This is for backward compatibility (it has been added in Parsec 3.8.0)
    // since we can instead just compute the HTTP redirection URL ourself from
    // the `bootstrap_url` field.
}

fn bootstrap_url_deserialize<'de, D>(
    deserializer: D,
) -> Result<ParsecOrganizationBootstrapAddr, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use std::str::FromStr;
    let s: &str = serde::Deserialize::deserialize(deserializer)?;
    ParsecOrganizationBootstrapAddr::from_str(s).map_err(serde::de::Error::custom)
}

#[derive(serde::Deserialize)]
struct CreationOrganizationErr {
    detail: String,
}

pub async fn create_organization_req(
    organization_id: &OrganizationID,
    addr: &ParsecAddr,
    administration_token: &str,
) -> anyhow::Result<ParsecOrganizationBootstrapAddr> {
    let url = addr.to_http_url(Some("/administration/organizations"));

    let client = libparsec_client_connection::build_client()?;
    let rep = client
        .post(url)
        .bearer_auth(administration_token)
        .json(&serde_json::json!({
            "organization_id": organization_id,
        }))
        .send()
        .await?;

    match rep.json::<CreateOrganizationRep>().await? {
        CreateOrganizationRep::Ok(res) => Ok(res.bootstrap_url),
        CreateOrganizationRep::Err(res) => Err(anyhow::anyhow!("{}", res.detail)),
    }
}

pub async fn main(args: Args) -> anyhow::Result<()> {
    let Args {
        organization,
        token,
        addr,
    } = args;
    log::trace!("Creating organization \"{organization}\" (addr={addr})");

    let mut handle = start_spinner("Creating organization".into());

    let organization_addr = create_organization_req(&organization, &addr, &token).await?;

    handle.stop_with_message(format!(
        "Organization bootstrap URL: {YELLOW}{}{RESET}",
        organization_addr.to_http_redirection_url(),
    ));

    Ok(())
}

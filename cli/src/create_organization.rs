// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use reqwest::Client;

use libparsec::{OrganizationID, ParsecAddr, ParsecOrganizationBootstrapAddr};

use crate::utils::*;

#[derive(clap::Parser)]
pub struct CreateOrganization {
    /// OrganizationID
    #[arg(short, long)]
    organization_id: OrganizationID,
    #[clap(flatten)]
    server: ServerSharedOpts,
}

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

    let rep = Client::new()
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

pub async fn create_organization(create_organization: CreateOrganization) -> anyhow::Result<()> {
    let CreateOrganization {
        organization_id,
        server: ServerSharedOpts { addr, token },
    } = create_organization;

    let mut handle = start_spinner("Creating organization".into());

    let organization_addr = create_organization_req(&organization_id, &addr, &token).await?;

    handle.stop_with_message(format!(
        "Organization bootstrap url: {YELLOW}{organization_addr}{RESET}"
    ));

    Ok(())
}

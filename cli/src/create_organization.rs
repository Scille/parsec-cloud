// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use clap::Args;
use reqwest::Client;
use serde_json::Value;

use libparsec::{BootstrapToken, OrganizationID, ParsecAddr, ParsecOrganizationBootstrapAddr};

use crate::utils::*;

#[derive(Args)]
pub struct CreateOrganization {
    /// OrganizationID
    #[arg(short, long)]
    organization_id: OrganizationID,
    /// Server address (e.g: parsec3://127.0.0.1:6770?no_ssl=true)
    #[arg(short, long)]
    addr: ParsecAddr,
    /// Administration token
    #[arg(short, long)]
    token: String,
}

pub async fn create_organization_req(
    organization_id: &OrganizationID,
    addr: &ParsecAddr,
    administration_token: &str,
) -> anyhow::Result<BootstrapToken> {
    let url = addr.to_http_url(Some("/administration/organizations"));

    let rep = Client::new()
        .post(url)
        .bearer_auth(administration_token)
        .json(&serde_json::json!({
            "organization_id": organization_id,
        }))
        .send()
        .await?;

    let rep = rep.json::<Value>().await?;

    if let Some(e) = rep.get("error") {
        return Err(anyhow::anyhow!("{e}"));
    }

    Ok(
        BootstrapToken::from_hex(rep["bootstrap_token"].as_str().expect("Unreachable"))
            .expect("Unreachable"),
    )
}

pub async fn create_organization(create_organization: CreateOrganization) -> anyhow::Result<()> {
    let CreateOrganization {
        organization_id,
        addr,
        token,
    } = create_organization;

    let mut handle = start_spinner("Creating organization".into());

    let bootstrap_token = create_organization_req(&organization_id, &addr, &token).await?;

    let organization_addr =
        ParsecOrganizationBootstrapAddr::new(addr, organization_id, Some(bootstrap_token));

    handle.stop_with_message(format!(
        "Organization bootstrap url: {YELLOW}{organization_addr}{RESET}"
    ));

    Ok(())
}

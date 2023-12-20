// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use clap::Args;
use reqwest::Client;
use serde_json::Value;

use libparsec::{BackendAddr, BackendOrganizationBootstrapAddr, OrganizationID};

use crate::utils::*;

#[derive(Args)]
pub struct CreateOrganization {
    /// OrganizationID
    #[arg(short, long)]
    organization_id: OrganizationID,
    /// Server address (e.g: parsec://127.0.0.1:6770?no_ssl=true)
    #[arg(short, long)]
    addr: BackendAddr,
    /// Administration token
    #[arg(short, long)]
    token: String,
}

pub async fn create_organization_req(
    organization_id: &OrganizationID,
    addr: &BackendAddr,
    administration_token: &str,
) -> anyhow::Result<String> {
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

    Ok(rep["bootstrap_token"]
        .as_str()
        .expect("Unreachable")
        .to_string())
}

pub async fn create_organization(create_organization: CreateOrganization) -> anyhow::Result<()> {
    let CreateOrganization {
        organization_id,
        addr,
        token,
    } = create_organization;

    let handle = start_spinner("Creating organization");

    let bootstrap_token = create_organization_req(&organization_id, &addr, &token).await?;

    handle.done();

    let organization_addr =
        BackendOrganizationBootstrapAddr::new(addr, organization_id, Some(bootstrap_token));

    println!("Organization bootstrap url: {YELLOW}{organization_addr}{RESET}");

    Ok(())
}

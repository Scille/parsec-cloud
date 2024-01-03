// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use clap::Args;
use reqwest::Client;
use serde_json::Value;

use libparsec::{BackendAddr, OrganizationID};

#[derive(Args)]
pub struct StatusOrganization {
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

pub async fn status_organization_req(
    organization_id: &OrganizationID,
    addr: &BackendAddr,
    administration_token: &str,
) -> anyhow::Result<Value> {
    let url = addr.to_http_url(Some(&format!(
        "/administration/organizations/{organization_id}"
    )));

    let rep = Client::new()
        .get(url)
        .bearer_auth(administration_token)
        .send()
        .await?;

    Ok(rep.json::<Value>().await?)
}

pub async fn status_organization(status_organization: StatusOrganization) -> anyhow::Result<()> {
    let StatusOrganization {
        organization_id,
        addr,
        token,
    } = status_organization;

    let rep = status_organization_req(&organization_id, &addr, &token).await?;

    println!("{:#}", rep);

    Ok(())
}

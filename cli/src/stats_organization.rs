// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use clap::Args;
use reqwest::Client;
use serde_json::Value;

use libparsec::{BackendAddr, OrganizationID};

#[derive(Args)]
pub struct StatsOrganization {
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

pub async fn stats_organization_req(
    organization_id: &OrganizationID,
    addr: &BackendAddr,
    administration_token: &str,
) -> anyhow::Result<Value> {
    let url = addr.to_http_url(Some(&format!(
        "/administration/organizations/{organization_id}/stats"
    )));

    let rep = Client::new()
        .get(url)
        .bearer_auth(administration_token)
        .send()
        .await?;

    Ok(rep.json::<Value>().await?)
}

pub async fn stats_organization(stats_organization: StatsOrganization) -> anyhow::Result<()> {
    let StatsOrganization {
        organization_id,
        addr,
        token,
    } = stats_organization;

    let rep = stats_organization_req(&organization_id, &addr, &token).await?;

    println!("{:#}", rep);

    Ok(())
}

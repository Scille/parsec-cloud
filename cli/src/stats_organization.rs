// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use reqwest::Client;
use serde_json::Value;

use libparsec::{OrganizationID, ParsecAddr};

use crate::utils::ServerSharedOpts;

#[derive(clap::Parser)]
pub struct StatsOrganization {
    /// OrganizationID
    #[arg(short, long)]
    organization_id: OrganizationID,
    #[clap(flatten)]
    server: ServerSharedOpts,
}

pub async fn stats_organization_req(
    organization_id: &OrganizationID,
    addr: &ParsecAddr,
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
        server: ServerSharedOpts { addr, token },
    } = stats_organization;
    log::trace!("Retrieving stats for organization {organization_id} (addr={addr})");

    let rep = stats_organization_req(&organization_id, &addr, &token).await?;

    println!("{:#}", rep);

    Ok(())
}

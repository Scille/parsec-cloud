// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use reqwest::Client;
use serde_json::Value;

use libparsec::{OrganizationID, ParsecAddr};

crate::clap_parser_with_shared_opts_builder!(
    #[with = addr, token]
    pub struct Args {
        /// OrganizationID
        #[arg(short, long)]
        organization_id: OrganizationID,
    }
);

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

pub async fn main(args: Args) -> anyhow::Result<()> {
    let Args {
        organization_id,
        token,
        addr,
    } = args;
    log::trace!("Retrieving stats for organization {organization_id} (addr={addr})");

    let rep = stats_organization_req(&organization_id, &addr, &token).await?;

    println!("{:#}", rep);

    Ok(())
}

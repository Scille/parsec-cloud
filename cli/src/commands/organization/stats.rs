// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use serde_json::Value;

use libparsec::{OrganizationID, ParsecAddr};

crate::clap_parser_with_shared_opts_builder!(
    #[with = addr, token, organization]
    pub struct Args {
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

    let client = libparsec_client_connection::build_client()?;
    let rep = client
        .get(url)
        .bearer_auth(administration_token)
        .send()
        .await?;

    Ok(rep.json::<Value>().await?)
}

pub async fn main(args: Args) -> anyhow::Result<()> {
    let Args {
        organization,
        token,
        addr,
    } = args;
    log::trace!("Retrieving stats for organization {organization} (addr={addr})");

    let rep = stats_organization_req(&organization, &addr, &token).await?;

    println!("{:#}", rep);

    Ok(())
}

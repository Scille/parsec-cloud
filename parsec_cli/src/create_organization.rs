// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use clap::Args;
use reqwest::Client;
use serde_json::Value;
use terminal_spinners::{SpinnerBuilder, DOTS};

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
) -> String {
    let url = addr.to_http_url(Some("/administration/organizations"));

    let rep = Client::new()
        .post(url)
        .bearer_auth(administration_token)
        .json(&serde_json::json!({
            "organization_id": organization_id,
        }))
        .send()
        .await
        .expect("Invalid request, maybe the server didn't start ?");

    let bootstrap_token = rep.json::<Value>().await.expect("Unreachable")["bootstrap_token"]
        .as_str()
        .expect("Unreachable")
        .into();

    bootstrap_token
}

pub async fn create_organization(create_organization: CreateOrganization) {
    let CreateOrganization {
        organization_id,
        addr,
        token,
    } = create_organization;

    let handle = SpinnerBuilder::new()
        .spinner(&DOTS)
        .text("Creating organization")
        .start();

    let bootstrap_token = create_organization_req(&organization_id, &addr, &token).await;

    handle.done();

    let organization_addr =
        BackendOrganizationBootstrapAddr::new(addr, organization_id, Some(bootstrap_token));

    let organization_addr_display = organization_addr.to_url();

    println!("Bootstrap organization url: {YELLOW}{organization_addr_display}{RESET}")
}

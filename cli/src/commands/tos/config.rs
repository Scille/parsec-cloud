use std::{collections::HashMap, path::PathBuf};

use libparsec::{OrganizationID, ParsecAddr};

crate::clap_parser_with_shared_opts_builder!(
    #[with = addr, token, organization]
    pub struct Args {
        /// If set, will remove TOS for the provided organization.
        #[arg(long)]
        remove: bool,
        /// Read the TOS configuration from a JSON file.
        #[arg(long)]
        from_json: Option<PathBuf>,
        /// List of locale and url for TOS configuration
        /// Each arguments should be in the form of `{locale}={url}`
        raw: Vec<String>,
    }
);

pub async fn main(args: Args) -> anyhow::Result<()> {
    let Args {
        organization,
        token,
        addr,
        raw,
        remove,
        from_json,
    } = args;
    log::trace!("Configure TOS for organization {organization} (addr={addr})");

    let raw_data = from_json.map(std::fs::read_to_string).transpose()?;

    let req = if let Some(ref raw_data) = raw_data {
        let localized_tos_url = serde_json::from_str::<HashMap<_, _>>(raw_data)?;
        TosReq::set_tos(localized_tos_url)
    } else if remove {
        TosReq::to_remove()
    } else {
        let localized_tos_url = raw
            .iter()
            .map(|arg| {
                arg.split_once('=')
                    .ok_or_else(|| anyhow::anyhow!("Missing '=<URL>' in argument ('{arg}')"))
            })
            .collect::<anyhow::Result<HashMap<_, _>>>()?;
        TosReq::set_tos(localized_tos_url)
    };

    config_tos_for_org_req(&addr, &token, &organization, req).await
}

#[derive(serde::Serialize)]
pub struct TosReq<'a> {
    tos: Option<HashMap<&'a str, &'a str>>,
}

impl<'a> TosReq<'a> {
    pub(crate) fn to_remove() -> Self {
        Self { tos: None }
    }

    pub fn set_tos(tos: HashMap<&'a str, &'a str>) -> Self {
        Self { tos: Some(tos) }
    }
}

pub async fn config_tos_for_org_req<'a>(
    addr: &ParsecAddr,
    token: &'a str,
    organization: &OrganizationID,
    tos_req: TosReq<'a>,
) -> anyhow::Result<()> {
    let url = addr.to_http_url(Some(&format!(
        "/administration/organizations/{organization}"
    )));
    let client = libparsec_client_connection::build_client()?;
    let rep = client
        .patch(url)
        .json(&tos_req)
        .bearer_auth(token)
        .send()
        .await?;

    match rep.status() {
        reqwest::StatusCode::OK => Ok(()),
        reqwest::StatusCode::NOT_FOUND => {
            Err(anyhow::anyhow!("Organization {organization} not found"))
        }
        code => Err(anyhow::anyhow!("Unexpected HTTP status code {code}")),
    }
}

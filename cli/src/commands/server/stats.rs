// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use reqwest::Response;
use serde_json::Value;

use libparsec::{DateTime, ParsecAddr};

crate::clap_parser_with_shared_opts_builder!(
    #[with = addr, token]
    pub struct Args {
        /// Output format (json/csv)
        #[arg(short, long, default_value_t = Format::Json)]
        format: Format,
        /// Ignore everything after this date (e.g: 2024-01-01T00:00:00-00:00)
        #[arg(short, long)]
        end_date: Option<DateTime>,
    }
);

#[derive(Clone, Copy)]
pub enum Format {
    Json,
    Csv,
}

impl std::str::FromStr for Format {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(Self::Json),
            "csv" => Ok(Self::Csv),
            _ => Err("Invalid format"),
        }
    }
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            Self::Json => "json",
            Self::Csv => "csv",
        })
    }
}

pub async fn stats_server_req(
    addr: &ParsecAddr,
    administration_token: &str,
    format: Format,
    end_date: Option<DateTime>,
) -> anyhow::Result<Response> {
    let mut url = addr.to_http_url(Some("/administration/stats"));
    url.set_query(Some(&format!(
        "format={format}{}",
        match end_date {
            Some(end_date) => format!("&at={}", end_date.to_rfc3339()),
            None => "".into(),
        }
    )));

    let client = libparsec_client_connection::build_client()?;
    Ok(client
        .get(url)
        .bearer_auth(administration_token)
        .send()
        .await?)
}

pub async fn main(args: Args) -> anyhow::Result<()> {
    let Args {
        format,
        end_date,
        token,
        addr,
    } = args;
    log::trace!("Retrieving server's stats (addr={addr}, format={format})");

    let rep = stats_server_req(&addr, &token, format, end_date).await?;

    match format {
        Format::Json => println!("{:#}", rep.json::<Value>().await?),
        Format::Csv => println!("{}", rep.text().await?),
    }

    Ok(())
}

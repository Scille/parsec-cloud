// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use clap::Args;
use reqwest::{Client, Response};
use serde_json::Value;

use libparsec::{DateTime, ParsecAddr};

#[derive(Args)]
pub struct StatsServer {
    /// Server address (e.g: parsec3://127.0.0.1:6770?no_ssl=true)
    #[arg(short, long)]
    addr: ParsecAddr,
    /// Administration token
    #[arg(short, long)]
    token: String,
    /// Output format (json/csv)
    #[arg(short, long, default_value_t = Format::Json)]
    format: Format,
    /// Ignore everything after this date (e.g: 2024-01-01T00:00:00-00:00)
    #[arg(short, long)]
    end_date: Option<DateTime>,
}

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

    Ok(Client::new()
        .get(url)
        .bearer_auth(administration_token)
        .send()
        .await?)
}

pub async fn stats_server(stats_organization: StatsServer) -> anyhow::Result<()> {
    let StatsServer {
        addr,
        token,
        format,
        end_date,
    } = stats_organization;

    let rep = stats_server_req(&addr, &token, format, end_date).await?;

    match format {
        Format::Json => println!("{:#}", rep.json::<Value>().await?),
        Format::Csv => println!("{}", rep.text().await?),
    }

    Ok(())
}

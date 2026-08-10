// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use reqwest::Response;
use serde_json::Value;

use libparsec::{DateTime, ParsecAddr};
use tokio::io::AsyncWriteExt;

use crate::ui::DataFormat;

crate::clap_parser_with_shared_opts_builder!(
    #[with = addr, token]
    pub struct Args {
        /// Ignore everything after this date (e.g: 2024-01-01T00:00:00-00:00)
        #[arg(short, long, value_hint = clap::ValueHint::Other)]
        end_date: Option<DateTime>,
    }
);

#[derive(Clone, Copy)]
pub enum Format {
    Json,
    Csv,
}

impl From<DataFormat> for Format {
    fn from(value: DataFormat) -> Self {
        match value {
            DataFormat::Plain => Self::Csv,
            DataFormat::Json => Self::Json,
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

pub async fn main(ui: crate::Ui, args: Args) -> anyhow::Result<()> {
    let Args {
        end_date,
        token,
        addr,
    } = args;
    log::trace!("Retrieving server's stats (addr={addr})");

    let rep = stats_server_req(&addr, &token, ui.format.into(), end_date).await?;

    match ui.format {
        DataFormat::Json => {
            let json = rep.json::<Value>().await?;
            serde_json::to_writer_pretty(std::io::stdout().lock(), &json)?;
        }
        DataFormat::Plain => {
            tokio::io::stdout()
                .write_all(rep.text().await?.as_bytes())
                .await?
        }
    }

    Ok(())
}

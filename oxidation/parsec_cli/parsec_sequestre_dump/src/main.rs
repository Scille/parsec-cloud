// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

mod model;
mod schema;
#[cfg(test)]
mod tests;

use clap::Parser;
use diesel::{Connection, SqliteConnection};
use std::{error::Error, path::PathBuf};

use crate::model::Workspace;
use parsec_api_crypto::SecretKey;

/// Simple program that dumps files and folders in a file
#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    /// Sqlite input file (e.g `my/workspace_export.sqlite`)
    #[clap(short, long)]
    input: PathBuf,

    /// Secret key (e.g `my/keyfile.key`)
    #[clap(short, long)]
    key: String,

    /// Output directory (e.g `./output`)
    #[clap(short, long)]
    output: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let input = args.input.to_str().unwrap();
    let key_path = args.key.as_str();
    let output = args.output;

    let mut conn = SqliteConnection::establish(input)?;

    let key_file = std::fs::read(key_path)?;
    let key = SecretKey::try_from(&key_file[..])?;

    Workspace::dump(&mut conn, &key, &output)
}

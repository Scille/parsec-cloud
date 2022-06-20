// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

mod error;
mod model;
mod schema;
#[cfg(test)]
mod tests;

use clap::Parser;
use diesel::{Connection, SqliteConnection};
use std::path::PathBuf;

use crate::{error::ExportError, model::Workspace};
use libparsec_crypto::SecretKey;

/// Simple program that dumps files and folders in a workspace
#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    /// Sqlite input file (e.g `my/workspace_export.sqlite`)
    #[clap(short, long)]
    input: PathBuf,

    /// Secret key (e.g `my/keyfile.key`)
    #[clap(short, long)]
    key: PathBuf,

    /// Output directory (e.g `./output`)
    #[clap(short, long)]
    output: PathBuf,
}

fn main() -> Result<(), ExportError> {
    let args = Args::parse();

    let input = args.input;
    let key_path = args.key;
    let output = args.output;

    let mut conn = SqliteConnection::establish(input.to_str().unwrap())
        .map_err(|_| ExportError::ConnectionFailed { path: input })?;

    let key_file =
        std::fs::read(key_path.to_str().unwrap()).map_err(|_| ExportError::MissingKey {
            path: key_path.clone(),
        })?;
    let key = SecretKey::try_from(&key_file[..])
        .map_err(|_| ExportError::InvalidKey { path: key_path })?;

    Workspace::dump(&mut conn, &key, &output)
}

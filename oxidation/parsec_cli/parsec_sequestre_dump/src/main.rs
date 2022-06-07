// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

#[macro_use]
extern crate diesel;

mod model;
mod schema;

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

#[cfg(test)]
mod tests {
    use diesel::{Connection, SqliteConnection};
    use rstest::rstest;
    use std::path::PathBuf;
    use tests_fixtures::{alice, Device};

    use crate::model::Workspace;

    #[rstest]
    fn test_parsec_sequestre_dump(alice: &Device) {
        use diesel::connection::SimpleConnection;

        let input = "/tmp/parsec_sequestre_dump.sqlite";
        let output = PathBuf::from("/tmp/out");
        let dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR should be set");

        macro_rules! read_sql {
            ($file: literal) => {
                std::fs::read_to_string(format!("{dir}/migrations/{}", $file)).unwrap()
            };
        }

        let mut conn = SqliteConnection::establish(input).unwrap();

        let sql = read_sql!("0000_init/down.sql") + &read_sql!("0000_init/up.sql");

        conn.batch_execute(&sql).unwrap();

        Workspace::init_db(&mut conn, alice);

        Workspace::dump(&mut conn, &alice.local_symkey, &output).unwrap();

        assert_eq!(std::fs::read("/tmp/out/file0").unwrap(), b"Hello World \0");
        assert_eq!(
            std::fs::read("/tmp/out/folder0/file00").unwrap(),
            b"file00's content"
        );
        assert_eq!(
            std::fs::read("/tmp/out/folder0/folder01/file010").unwrap(),
            b"file010's content"
        );
        assert_eq!(
            std::fs::read("/tmp/out/folder0/folder01/file011").unwrap(),
            b"file011's content"
        );
    }
}

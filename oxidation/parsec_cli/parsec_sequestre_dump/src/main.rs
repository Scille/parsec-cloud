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

    /// Output file (e.g `./output`)
    #[clap(short, long)]
    output: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let input = args.input.to_str().unwrap();
    let key_path = args.key.as_str();
    let output = args.output.to_str().unwrap();

    let mut conn = SqliteConnection::establish(input)?;

    let key_file = std::fs::read(key_path)?;
    let key = SecretKey::try_from(&key_file[..])?;
    let workspace = Workspace::query_all(&mut conn, &key)?;

    workspace.dump(output)
}

#[cfg(test)]
mod tests {
    use diesel::{Connection, SqliteConnection};
    use rstest::rstest;
    use tests_fixtures::{alice, Device};

    use crate::model::{Data, Workspace};

    #[rstest]
    fn test_parsec_sequestre_dump(alice: &Device) {
        use diesel::connection::SimpleConnection;

        let input = "/tmp/parsec_sequestre_dump.sqlite";
        let output = "/tmp/out";
        let dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR should be set");
        let t0 = "2000-01-01T00:00:00Z".parse().unwrap();

        macro_rules! read_sql {
            ($file: literal) => {
                std::fs::read_to_string(format!("{dir}/migrations/{}", $file)).unwrap()
            };
        }

        let mut conn = SqliteConnection::establish(input).unwrap();

        let sql = read_sql!("0000_init/down.sql") + &read_sql!("0000_init/up.sql");

        conn.batch_execute(&sql).unwrap();

        Workspace::init_db(&mut conn, alice);

        let workspace = Workspace::query_all(&mut conn, &alice.local_symkey).unwrap();

        assert_eq!(
            workspace,
            Workspace(Data {
                author: alice.device_id.clone(),
                version: 1,
                created: t0,
                updated: t0,
                size: 63,
                children: vec![
                    Data {
                        name: "file0".into(),
                        author: alice.device_id.clone(),
                        version: 1,
                        created: t0,
                        updated: t0,
                        size: 13,
                        content: "Hello World \0".into(),
                        ..Default::default()
                    },
                    Data {
                        name: "folder0".into(),
                        author: alice.device_id.clone(),
                        version: 1,
                        created: t0,
                        updated: t0,
                        size: 50,
                        children: vec![
                            Data {
                                name: "file00".into(),
                                author: alice.device_id.clone(),
                                version: 1,
                                created: t0,
                                updated: t0,
                                size: 16,
                                content: "file00's content".into(),
                                ..Default::default()
                            },
                            Data {
                                name: "folder00".into(),
                                author: alice.device_id.clone(),
                                version: 1,
                                created: t0,
                                updated: t0,
                                size: 0,
                                ..Default::default()
                            },
                            Data {
                                name: "folder01".into(),
                                author: alice.device_id.clone(),
                                version: 1,
                                created: t0,
                                updated: t0,
                                size: 34,
                                children: vec![
                                    Data {
                                        name: "file010".into(),
                                        author: alice.device_id.clone(),
                                        version: 1,
                                        created: t0,
                                        updated: t0,
                                        size: 17,
                                        content: "file010's content".into(),
                                        ..Default::default()
                                    },
                                    Data {
                                        name: "file011".into(),
                                        author: alice.device_id.clone(),
                                        version: 1,
                                        created: t0,
                                        updated: t0,
                                        size: 17,
                                        content: "file011's content".into(),
                                        ..Default::default()
                                    }
                                ],
                                ..Default::default()
                            },
                        ],
                        ..Default::default()
                    }
                ],
                ..Default::default()
            })
        );

        workspace.dump(output).unwrap();

        let loaded_data = Workspace::load(output);

        assert_eq!(workspace, loaded_data);

        std::fs::remove_file(input).unwrap();
        std::fs::remove_file(output).unwrap();
    }
}

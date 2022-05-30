// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

#[macro_use]
extern crate diesel;

mod model;
mod schema;

use clap::Parser;
use diesel::{Connection, SqliteConnection};
use std::path::PathBuf;

use crate::model::Data;
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

fn main() {
    let args = Args::parse();

    let input = args.input.to_str().unwrap();
    let key = SecretKey::try_from(&std::fs::read(args.key.as_str()).unwrap()[..]).unwrap();
    let output = args.output.to_str().unwrap();

    let mut conn = SqliteConnection::establish(input).unwrap();

    let data = Data::query_all(&mut conn);

    data.dump(&key, output);
}

#[test]
fn test_parsec_sequestre_dump() {
    use crate::model::{Block, Info, VlobAtom};
    use diesel::connection::SimpleConnection;

    let input = "/tmp/parsec_sequestre_dump.sqlite";
    let key = SecretKey::generate();
    let output = "/tmp/out";
    let dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR should be set");

    macro_rules! read_sql {
        ($file: literal) => {
            std::fs::read_to_string(format!("{dir}/migrations/{}", $file)).unwrap()
        };
    }

    let mut conn = SqliteConnection::establish(input).unwrap();

    let sql = read_sql!("0000_init/down.sql") + &read_sql!("0000_init/up.sql");

    conn.batch_execute(&sql).unwrap();

    Data::init_db(&mut conn);

    let data = Data::query_all(&mut conn);

    assert_eq!(
        data,
        Data {
            blocks: vec![Block {
                block_id: b"block_id".to_vec(),
                data: b"data".to_vec(),
                author: 0,
                size: 4,
                created_on: 0.,
            }],
            vlob_atoms: vec![VlobAtom {
                vlob_id: b"vlob_id".to_vec(),
                version: 1,
                blob: b"blob".to_vec(),
                size: 4,
                author: 0,
                timestamp: 0.,
            }],
            role_certificates: vec![b"role".to_vec()],
            user_certificates: vec![b"user".to_vec()],
            device_certificates: vec![b"device".to_vec()],
            info: vec![Info {
                magic: 87947,
                version: 1,
                realm_id: b"realm_id".to_vec(),
            }]
        }
    );

    data.dump(&key, output);

    let loaded_data = Data::load(&key, output);

    assert_eq!(data, loaded_data);

    std::fs::remove_file(input).unwrap();
    std::fs::remove_file(output).unwrap();
}

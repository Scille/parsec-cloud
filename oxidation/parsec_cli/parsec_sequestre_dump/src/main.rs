// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

#[macro_use]
extern crate diesel;

mod model;
mod schema;

use clap::Parser;
use diesel::{Connection, QueryDsl, RunQueryDsl, SqliteConnection};
use parsec_api_crypto::SecretKey;

use crate::{
    model::{Block, Data, VlobAtom},
    schema::{block, device, realm_role, user_ as user, vlob_atom},
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    input: String,

    #[clap(short, long)]
    key: String,

    #[clap(short, long)]
    output: String,
}

fn query_all(conn: &mut SqliteConnection) -> Data {
    let blocks = block::table
        .select((
            block::block_id,
            block::data,
            block::author,
            block::size,
            block::created_on,
        ))
        .load::<Block>(conn)
        .unwrap();

    let vlob_atoms = vlob_atom::table
        .select((
            vlob_atom::vlob_id,
            vlob_atom::version,
            vlob_atom::blob,
            vlob_atom::size,
            vlob_atom::author,
            vlob_atom::timestamp,
        ))
        .load::<VlobAtom>(conn)
        .unwrap();

    let role_certificates = realm_role::table
        .select(realm_role::role_certificate)
        .load::<Vec<u8>>(conn)
        .unwrap();

    let user_certificates = user::table
        .select(user::user_certificate)
        .load::<Vec<u8>>(conn)
        .unwrap();

    let device_certificates = device::table
        .select(device::device_certificate)
        .load::<Vec<u8>>(conn)
        .unwrap();

    Data {
        blocks,
        vlob_atoms,
        role_certificates,
        user_certificates,
        device_certificates,
    }
}

fn main() {
    let args = Args::parse();

    let input = args.input.as_str();
    let key = SecretKey::try_from(&std::fs::read(args.key.as_str()).unwrap()[..]).unwrap();
    let output = args.output.as_str();

    let mut conn = SqliteConnection::establish(input).unwrap();

    let data = query_all(&mut conn);

    data.save(&key, output);
}

#[test]
fn test_parsec_sequestre_dump() {
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

    // Drop all
    let sql = read_sql!("0000_block/down.sql")
        + &read_sql!("0001_vlob_atom/down.sql")
        + &read_sql!("0002_realm_role/down.sql")
        + &read_sql!("0003_user/down.sql")
        + &read_sql!("0004_device/down.sql")
        + &read_sql!("0005_info/down.sql")
        // Generate all
        + &read_sql!("0000_block/up.sql")
        + &read_sql!("0001_vlob_atom/up.sql")
        + &read_sql!("0002_realm_role/up.sql")
        + &read_sql!("0003_user/up.sql")
        + &read_sql!("0004_device/up.sql")
        + &read_sql!("0005_info/up.sql");

    conn.batch_execute(&sql).unwrap();

    let data = query_all(&mut conn);
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
        }
    );

    data.save(&key, output);

    let loaded_data = Data::load(&key, output);
    assert_eq!(data, loaded_data);

    std::fs::remove_file(input).unwrap();
    std::fs::remove_file(output).unwrap();
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use parsec_api_types::BlockID;
use poc::*;
use std::collections::HashMap;

parsec_schema!("oxidation/crates/poc/struct.json");
parsec_schema!("oxidation/crates/poc/enum.json");
parsec_cmds!("oxidation/crates/poc/api_protocol.json");

fn main() {
    let block = authenticated::block_read::Req {
        block_id: BlockID::default(),
    };

    println!("{block:?}");

    println!("{:?}", block.dump());

    let block = authenticated::block_read::Rep::Ok { block: Vec::new() };

    println!("{block:?}");

    println!("{:?}", block.dump());
}

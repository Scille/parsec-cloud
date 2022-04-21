// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use parsec_api_types::*;
use poc::*;
// parsec_schema!("oxidation/crates/poc/struct.json");
// parsec_schema!("oxidation/crates/poc/enum.json");
parsec_cmds!("oxidation/crates/poc/api_protocol.json");

fn main() {
    let block = authenticated::block_read::Req {
        block_id: Maybe::Present(Some(BlockID::default())),
    };
    println!("{block:?}");
    let dump = block.dump().unwrap();
    println!("{:?}", String::from_utf8_lossy(&dump));
    println!("{:?}", authenticated::AnyCmdReq::load(&dump).unwrap());

    let block = authenticated::block_read::Req {
        block_id: Maybe::Present(None),
    };
    println!("{block:?}");
    let dump = block.dump().unwrap();
    println!("{:?} {dump:?}", String::from_utf8_lossy(&dump));
    println!("{:?}", authenticated::AnyCmdReq::load(&dump).unwrap());

    let block = authenticated::block_read::Req {
        block_id: Maybe::Absent,
    };
    println!("{block:?}");
    let dump = block.dump().unwrap();
    println!("{:?} {dump:?}", String::from_utf8_lossy(&dump));
    println!("{:?}", authenticated::AnyCmdReq::load(&dump).unwrap());

    let block = authenticated::block_read::Rep::Ok {
        block: Maybe::Present(Vec::new()),
    };
    println!("{block:?}");
    let dump = block.dump().unwrap();
    println!("{:?} {dump:?}", String::from_utf8_lossy(&dump));
    println!("{:?}", authenticated::block_read::Rep::load(&dump));

    let block = authenticated::block_read::Rep::Ok {
        block: Maybe::Absent,
    };
    println!("{block:?}");
    let dump = block.dump().unwrap();
    println!("{:?} {dump:?}", String::from_utf8_lossy(&dump));
    println!("{:?}", authenticated::block_read::Rep::load(&dump));
}

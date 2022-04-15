// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use poc::parsec_schema;
use std::collections::HashMap;

parsec_schema!("oxidation/crates/poc/schema.json");

fn main() {
    let foo = Foo::default();
    println!("{:?}", foo);
}

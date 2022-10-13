// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use glob::glob;

fn main() {
    for entry in glob("schema/**/*.json").expect("Failed to parse glob pattern") {
        let path = entry.expect("Glob iteration error");
        println!("cargo:rerun-if-changed={}", path.display())
    }
}

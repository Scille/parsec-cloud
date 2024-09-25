// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use glob::glob;

fn main() {
    // Register protocol .json5 schema files to trigger recompilation on change
    // Note there is catch though: adding a new .json5 schema file will not
    // trigger a recompilation !
    for entry in glob("../libparsec/crates/protocol/schema/**/*.json5")
        .expect("Failed to parse glob pattern")
    {
        let path = entry.expect("Glob iteration error");
        println!("cargo:rerun-if-changed={}", path.display())
    }
}

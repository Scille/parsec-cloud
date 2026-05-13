// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

fn main() {
    // Register .json5 schema files to trigger recompilation on change.
    // Note we specify the root schema folder (instead of using glob to list the
    // actual .json5 files) so that cargo is able to detect newly added files.
    println!("cargo:rerun-if-changed=schema/")
}

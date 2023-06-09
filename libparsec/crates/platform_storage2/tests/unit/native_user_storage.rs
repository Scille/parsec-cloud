// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::sync::Arc;

use libparsec_tests_fixtures::prelude::*;

use super::UserStorage;

#[parsec_test]
async fn bad_start(tmp_path: TmpPath, alice: &Device) {
    // 1) Bad path

    let not_a_dir_path = tmp_path.join("foo.txt");
    std::fs::File::create(&not_a_dir_path).unwrap();

    assert!(matches!(
        UserStorage::start(&not_a_dir_path, Arc::new(alice.local_device())).await,
        Err(_)
    ));

    // TODO: create a valid database, then modify the user manifest's vlob to
    // turn it into something invalid:
    // - invalid schema
    // - invalid encryption

    // TODO: modify the database to make it schema invalid

    // TODO: drop the database so that it exists but it is empty, this shouldn't cause any issue

    // TODO: remove user manifest's vlob from the database, this shouldn't cause any issue
}

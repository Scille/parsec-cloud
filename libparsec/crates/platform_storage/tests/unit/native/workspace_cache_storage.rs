// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::WorkspaceCacheStorage;

#[parsec_test]
async fn bad_start(tmp_path: TmpPath, alice: &Device) {
    // 1) Bad path

    let not_a_dir_path = tmp_path.join("foo.txt");
    std::fs::File::create(&not_a_dir_path).unwrap();

    let realm_id = VlobID::default();

    p_assert_matches!(
        WorkspaceCacheStorage::start(
            &not_a_dir_path,
            100,
            Arc::new(alice.local_device()),
            realm_id
        )
        .await,
        Err(_)
    );

    // TODO: create a valid database, then modify the blocks to turn it into
    // something invalid:
    // - invalid schema
    // - invalid encryption

    // TODO: modify the database to make its schema invalid

    // TODO: drop the database so that it exists but it is empty, this shouldn't cause any issue
}

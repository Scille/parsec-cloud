// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// TODO: test rename non existing entry doesn't change updated or need_sync fields in parent manifest
// TODO: test rename on existing destination doesn't change updated or need_sync fields in parent manifest if overwrite is false
// TODO: test rename non-placeholder on a placeholder: if similar type the non-placeholder entry ID is kept

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::{assert_ls, ls, workspace_ops_factory};
use crate::workspace::{MoveEntryMode, WorkspaceMoveEntryError};

#[parsec_test(testbed = "minimal_client_ready")]
async fn ok(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    // 1) Same parent

    ops.move_entry(
        "/foo/egg.txt".parse().unwrap(),
        "/foo/egg2.txt".parse().unwrap(),
        MoveEntryMode::CanReplace,
    )
    .await
    .unwrap();
    assert_ls!(ops, "/foo", ["egg2.txt", "spam"]).await;

    // 2) Different parent

    ops.move_entry(
        "/foo/spam".parse().unwrap(),
        "/spam2".parse().unwrap(),
        MoveEntryMode::CanReplace,
    )
    .await
    .unwrap();
    assert_ls!(ops, "/foo", ["egg2.txt"]).await;
    assert_ls!(ops, "/", ["bar.txt", "foo", "spam2"]).await;
}

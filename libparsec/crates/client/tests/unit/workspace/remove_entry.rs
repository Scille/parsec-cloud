// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// TODO: remove opened file
// TODO: test remove non existing entry doesn't change updated or need_sync fields in parent manifest
// TODO: test remove on existing destination doesn't change updated or need_sync fields in parent manifest if overwrite is false
// TODO: test remove need sync entry also cleans the dirty manifest & chunks in local storage

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::{assert_ls, ls, workspace_ops_factory};
use crate::workspace::WorkspaceRemoveEntryError;

#[parsec_test(testbed = "minimal_client_ready")]
async fn ok(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    ops.remove_folder("/foo/spam".parse().unwrap())
        .await
        .unwrap();
    assert_ls!(ops, "/foo", ["egg.txt"]).await;

    ops.remove_file("/foo/egg.txt".parse().unwrap())
        .await
        .unwrap();
    assert_ls!(ops, "/foo", []).await;

    ops.remove_entry("/foo".parse().unwrap()).await.unwrap();
    ops.remove_entry("/bar.txt".parse().unwrap()).await.unwrap();

    assert_ls!(ops, "/", []).await;
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn not_found(#[values("unknown", "parent_is_file")] kind: &str, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let path = match kind {
        "unknown" => "/dummy",
        "parent_is_file" => "/bar.txt/dummy",
        unknown => panic!("Unknown kind: {}", unknown),
    };

    let err = ops.remove_folder(path.parse().unwrap()).await.unwrap_err();
    p_assert_matches!(err, WorkspaceRemoveEntryError::EntryNotFound);

    let err = ops.remove_file(path.parse().unwrap()).await.unwrap_err();
    p_assert_matches!(err, WorkspaceRemoveEntryError::EntryNotFound);

    let err = ops.remove_entry(path.parse().unwrap()).await.unwrap_err();
    p_assert_matches!(err, WorkspaceRemoveEntryError::EntryNotFound);
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn remove_file_but_is_folder(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let err = ops
        .remove_file("/foo/spam".parse().unwrap())
        .await
        .unwrap_err();
    p_assert_matches!(err, WorkspaceRemoveEntryError::EntryIsFolder);
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn remove_folder_but_is_file(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let err = ops
        .remove_folder("/foo/egg.txt".parse().unwrap())
        .await
        .unwrap_err();
    p_assert_matches!(err, WorkspaceRemoveEntryError::EntryIsFile);

    let err = ops
        .remove_folder_all("/foo/egg.txt".parse().unwrap())
        .await
        .unwrap_err();
    p_assert_matches!(err, WorkspaceRemoveEntryError::EntryIsFile);
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn remove_not_empty_folder(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let err = ops
        .remove_folder("/foo".parse().unwrap())
        .await
        .unwrap_err();
    p_assert_matches!(err, WorkspaceRemoveEntryError::EntryIsNonEmptyFolder);

    ops.remove_folder_all("/foo".parse().unwrap())
        .await
        .unwrap();

    assert_ls!(ops, "/", ["bar.txt"]).await;
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn ignore_invalid_children(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let env = &env.customize(|builder| {
        builder
            .workspace_data_storage_local_folder_manifest_create_or_update(
                "alice@dev1",
                wksp1_id,
                wksp1_foo_id,
                None,
            )
            .customize_children(
                [
                    // Patch `/foo` children entry so that they reference a manifest
                    // that disagree on who is their parent, hence making `/foo` an
                    // empty folder.
                    ("egg.txt", Some(wksp1_bar_txt_id)),
                    ("spam", Some(wksp1_foo_id)),
                ]
                .into_iter(),
            );
    });

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    ops.remove_folder("/foo".parse().unwrap()).await.unwrap();

    assert_ls!(ops, "/", ["bar.txt"]).await;
}

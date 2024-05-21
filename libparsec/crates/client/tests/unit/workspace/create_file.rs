// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::{assert_ls, ls, workspace_ops_factory};
use crate::{
    workspace::{tests::utils::restart_workspace_ops, WorkspaceCreateFileError},
    EventWorkspaceOpsOutboundSyncNeeded,
};

#[parsec_test(testbed = "minimal_client_ready")]
async fn create_in_root(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    // Ensure `new_file` doesn't exist
    let root_children = ls!(ops, "/").await;
    assert!(!root_children.contains(&"new_file".to_string()));

    let mut spy = ops.event_bus.spy.start_expecting();

    let new_file_id = ops
        .create_file("/new_file.txt".parse().unwrap())
        .await
        .unwrap();
    spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
        p_assert_eq!(e.realm_id, wksp1_id);
        p_assert_eq!(e.entry_id, new_file_id);
    });
    spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
        p_assert_eq!(e.realm_id, wksp1_id);
        p_assert_eq!(e.entry_id, wksp1_id);
    });

    assert_ls!(ops, "/", ["bar.txt", "foo", "new_file.txt"]).await;

    // Restart the workspace ops to make sure the change are not only in cache
    let ops = restart_workspace_ops(ops).await;
    assert_ls!(ops, "/", ["bar.txt", "foo", "new_file.txt"]).await;
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn create_in_child(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    // Ensure `/foo/new_file` doesn't exist
    let root_children = ls!(ops, "/foo").await;
    assert!(!root_children.contains(&"new_file.txt".to_string()));

    let mut spy = ops.event_bus.spy.start_expecting();

    let new_file_id = ops
        .create_file("/foo/new_file.txt".parse().unwrap())
        .await
        .unwrap();
    spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
        p_assert_eq!(e.realm_id, wksp1_id);
        p_assert_eq!(e.entry_id, new_file_id);
    });
    spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
        p_assert_eq!(e.realm_id, wksp1_id);
        p_assert_eq!(e.entry_id, wksp1_foo_id);
    });

    assert_ls!(ops, "/foo", ["egg.txt", "new_file.txt", "spam"]).await;

    // Restart the workspace ops to make sure the change are not only in cache
    let ops = restart_workspace_ops(ops).await;
    assert_ls!(ops, "/foo", ["egg.txt", "new_file.txt", "spam"]).await;
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn target_is_folder(#[values(false, true)] target_is_root: bool, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let spy = ops.event_bus.spy.start_expecting();

    let (path, expected_already_exist_id) = if target_is_root {
        ("/", wksp1_id)
    } else {
        ("/foo", wksp1_foo_id)
    };
    let err = ops.create_file(path.parse().unwrap()).await.unwrap_err();
    p_assert_matches!(err, WorkspaceCreateFileError::EntryExists { entry_id } if entry_id == expected_already_exist_id);
    spy.assert_no_events();

    assert_ls!(ops, "/", ["bar.txt", "foo"]).await;
    assert_ls!(ops, "/foo", ["egg.txt", "spam"]).await;
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn already_exists_in_root(#[values(true, false)] existing_is_file: bool, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    // Sanity check just to make sure the file already exists
    assert_ls!(ops, "/", ["bar.txt", "foo"]).await;

    let spy = ops.event_bus.spy.start_expecting();

    let (path, already_exists_id) = if existing_is_file {
        let path = "/bar.txt".parse().unwrap();
        let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
        (path, wksp1_bar_txt_id)
    } else {
        let path = "/foo".parse().unwrap();
        let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
        (path, wksp1_foo_id)
    };
    let err = ops.create_file(path).await.unwrap_err();
    p_assert_matches!(err, WorkspaceCreateFileError::EntryExists { entry_id } if entry_id == already_exists_id);
    spy.assert_no_events();

    // Ensure nothing has changed
    assert_ls!(ops, "/", ["bar.txt", "foo"]).await;
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn already_exists_in_child(#[values("file", "folder")] kind: &str, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    // Sanity check just to make sure the file already exists
    assert_ls!(ops, "/foo", ["egg.txt", "spam"]).await;

    let spy = ops.event_bus.spy.start_expecting();

    let (path, already_exists_id) = match kind {
        "file" => {
            let path = "/foo/egg.txt".parse().unwrap();
            let wksp1_foo_egg_txt_id: VlobID = *env.template.get_stuff("wksp1_foo_egg_txt_id");
            (path, wksp1_foo_egg_txt_id)
        }
        "folder" => {
            let path = "/foo/spam".parse().unwrap();
            let wksp1_foo_spam_id: VlobID = *env.template.get_stuff("wksp1_foo_spam_id");
            (path, wksp1_foo_spam_id)
        }
        unknown => panic!("Unknown kind: {}", unknown),
    };
    let err = ops.create_file(path).await.unwrap_err();
    p_assert_matches!(err, WorkspaceCreateFileError::EntryExists { entry_id } if entry_id == already_exists_id);
    spy.assert_no_events();

    // Ensure nothing has changed
    assert_ls!(ops, "/foo", ["egg.txt", "spam"]).await;
}

// TODO: test parent manifest not present in local (with and without server available)
// TODO: test entry already exists but not present in local

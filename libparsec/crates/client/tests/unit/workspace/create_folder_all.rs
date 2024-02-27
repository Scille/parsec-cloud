// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::{assert_ls, ls, workspace_ops_factory};
use crate::{workspace::WorkspaceCreateFolderError, EventWorkspaceOpsOutboundSyncNeeded};

#[parsec_test(testbed = "minimal_client_ready")]
async fn create_in_root(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    // Ensure `new_folder` doesn't exist
    let root_children = ls!(ops, "/").await;
    assert!(!root_children.contains(&"new_folder".to_string()));

    let mut spy = ops.event_bus.spy.start_expecting();

    let new_folder_id = ops
        .create_folder_all("/new_folder".parse().unwrap())
        .await
        .unwrap();
    spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
        p_assert_eq!(e.realm_id, wksp1_id);
        p_assert_eq!(e.entry_id, new_folder_id);
    });
    spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
        p_assert_eq!(e.realm_id, wksp1_id);
        p_assert_eq!(e.entry_id, wksp1_id);
    });

    assert_ls!(ops, "/", ["bar.txt", "foo", "new_folder"]).await;
    assert_ls!(ops, "/new_folder", []).await;
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn create_in_child(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    // Ensure `/foo/new_folder` doesn't exist
    let root_children = ls!(ops, "/foo").await;
    assert!(!root_children.contains(&"new_folder".to_string()));

    let mut spy = ops.event_bus.spy.start_expecting();

    let new_folder_id = ops
        .create_folder_all("/foo/new_folder".parse().unwrap())
        .await
        .unwrap();
    spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
        p_assert_eq!(e.realm_id, wksp1_id);
        p_assert_eq!(e.entry_id, new_folder_id);
    });
    spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
        p_assert_eq!(e.realm_id, wksp1_id);
        p_assert_eq!(e.entry_id, wksp1_foo_id);
    });

    assert_ls!(ops, "/foo", ["egg.txt", "new_folder", "spam"]).await;
    assert_ls!(ops, "/foo/new_folder", []).await;
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn already_exists_in_root(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    // Ensure `foo` already exists
    let children = ls!(ops, "/").await;
    assert!(children.contains(&"foo".to_string()));

    let spy = ops.event_bus.spy.start_expecting();

    let err = ops
        .create_folder_all("/foo".parse().unwrap())
        .await
        .unwrap_err();
    p_assert_matches!(err, WorkspaceCreateFolderError::EntryExists { entry_id } if entry_id == wksp1_foo_id);
    spy.assert_no_events();

    assert_ls!(ops, "/", ["bar.txt", "foo"]).await;
    assert_ls!(ops, "/foo", ["egg.txt", "spam"]).await;
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn already_exists_in_child(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_spam_id: VlobID = *env.template.get_stuff("wksp1_foo_spam_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    // Ensure `/foo/spam` already exists
    let children = ls!(ops, "/foo/").await;
    assert!(children.contains(&"spam".to_string()));

    let spy = ops.event_bus.spy.start_expecting();

    let err = ops
        .create_folder_all("/foo/spam".parse().unwrap())
        .await
        .unwrap_err();
    p_assert_matches!(err, WorkspaceCreateFolderError::EntryExists { entry_id } if entry_id == wksp1_foo_spam_id);
    spy.assert_no_events();

    assert_ls!(ops, "/foo", ["egg.txt", "spam"]).await;
    assert_ls!(ops, "/foo/spam", []).await;
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn recreate_root(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let spy = ops.event_bus.spy.start_expecting();

    let err = ops
        .create_folder_all("/".parse().unwrap())
        .await
        .unwrap_err();
    p_assert_matches!(err, WorkspaceCreateFolderError::EntryExists { entry_id } if entry_id == wksp1_id);
    spy.assert_no_events();

    assert_ls!(ops, "/", ["bar.txt", "foo"]).await;
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn create_missing_parents(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let mut spy = ops.event_bus.spy.start_expecting();

    let foo_a_b_c_id = ops
        .create_folder_all("/foo/a/b/c".parse().unwrap())
        .await
        .unwrap();
    let mut foo_a_id = None;
    let mut foo_a_b_id = None;
    // 1) /foo/a is created (i.e. a then foo manifests are modified)
    spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
        p_assert_eq!(e.realm_id, wksp1_id);
        foo_a_id = Some(e.entry_id);
    });
    spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
        p_assert_eq!(e.realm_id, wksp1_id);
        p_assert_eq!(e.entry_id, wksp1_foo_id);
    });
    // 2) /foo/a/b is created (i.e. b then a manifests are modified)
    spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
        p_assert_eq!(e.realm_id, wksp1_id);
        foo_a_b_id = Some(e.entry_id);
    });
    spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
        p_assert_eq!(e.realm_id, wksp1_id);
        p_assert_eq!(e.entry_id, *foo_a_id.as_ref().unwrap());
    });
    // 3) /foo/a/b/c is created (i.e. c then b manifests are modified)
    spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
        p_assert_eq!(e.realm_id, wksp1_id);
        p_assert_eq!(e.entry_id, foo_a_b_c_id);
    });
    spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
        p_assert_eq!(e.realm_id, wksp1_id);
        p_assert_eq!(e.entry_id, *foo_a_b_id.as_ref().unwrap());
    });

    assert_ls!(ops, "/foo", ["a", "egg.txt", "spam"]).await;
    assert_ls!(ops, "/foo/a", ["b"]).await;
    assert_ls!(ops, "/foo/a/b", ["c"]).await;
    assert_ls!(ops, "/foo/a/b/c", []).await;
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn file_in_the_path(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let spy = ops.event_bus.spy.start_expecting();

    let err = ops
        .create_folder_all("/bar.txt/new_folder".parse().unwrap())
        .await
        .unwrap_err();
    p_assert_matches!(err, WorkspaceCreateFolderError::ParentIsFile);
    spy.assert_no_events();

    assert_ls!(ops, "/", ["bar.txt", "foo"]).await;
}

// TODO: test entry not present in local (with and without server available)

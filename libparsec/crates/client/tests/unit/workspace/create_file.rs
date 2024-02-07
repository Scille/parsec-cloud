// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::workspace_ops_factory;
use crate::{
    workspace::{EntryStat, WorkspaceFsOperationError},
    EventWorkspaceOpsOutboundSyncNeeded,
};

macro_rules! ls {
    ($ops:expr, $path:expr) => {
        async {
            let path = $path.parse().unwrap();
            let info = $ops.stat_entry(&path).await.unwrap();
            let children = match info {
                EntryStat::Folder { children, .. } => children,
                x => panic!("Bad info type: {:?}", x),
            };
            children
                .iter()
                .map(|entry_name| entry_name.to_string())
                .collect::<Vec<_>>()
        }
    };
}

macro_rules! assert_ls {
    ($ops:expr, $path:expr, $expected:expr) => {
        async {
            let children = ls!($ops, $path).await;
            // Must specify `$expected` type to handle empty array
            let expected: &[&str] = &$expected;
            p_assert_eq!(children, expected);
        }
    };
}

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
        .create_file(&"/new_file.txt".parse().unwrap())
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
        .create_file(&"/foo/new_file.txt".parse().unwrap())
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
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn create_as_root(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let spy = ops.event_bus.spy.start_expecting();

    let err = ops.create_file(&"/".parse().unwrap()).await.unwrap_err();
    p_assert_matches!(err, WorkspaceFsOperationError::EntryExists { entry_id } if entry_id == wksp1_id);
    spy.assert_no_events();

    assert_ls!(ops, "/", ["bar.txt", "foo"]).await;
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
    let err = ops.create_file(&path).await.unwrap_err();
    p_assert_matches!(err, WorkspaceFsOperationError::EntryExists { entry_id } if entry_id == already_exists_id);
    spy.assert_no_events();

    // Ensure nothing has changed
    assert_ls!(ops, "/", ["bar.txt", "foo"]).await;
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn already_exists_in_child(#[values(true, false)] existing_is_file: bool, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    // Sanity check just to make sure the file already exists
    assert_ls!(ops, "/foo", ["egg.txt", "spam"]).await;

    let spy = ops.event_bus.spy.start_expecting();

    let (path, already_exists_id) = if existing_is_file {
        let path = "/foo/egg.txt".parse().unwrap();
        let wksp1_foo_egg_txt_id: VlobID = *env.template.get_stuff("wksp1_foo_egg_txt_id");
        (path, wksp1_foo_egg_txt_id)
    } else {
        let path = "/foo/spam".parse().unwrap();
        let wksp1_foo_spam_id: VlobID = *env.template.get_stuff("wksp1_foo_spam_id");
        (path, wksp1_foo_spam_id)
    };
    let err = ops.create_file(&path).await.unwrap_err();
    p_assert_matches!(err, WorkspaceFsOperationError::EntryExists { entry_id } if entry_id == already_exists_id);
    spy.assert_no_events();

    // Ensure nothing has changed
    assert_ls!(ops, "/foo", ["egg.txt", "spam"]).await;
}

// TODO: test parent manifest not present in local (with and without server available)
// TODO: test entry already exists but not present in local

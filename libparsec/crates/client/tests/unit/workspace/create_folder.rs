// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::{
    protocol::authenticated_cmds, test_register_sequence_of_send_hooks,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::{assert_ls, ls, workspace_ops_factory};
use crate::{EventWorkspaceOpsOutboundSyncNeeded, workspace::WorkspaceCreateFolderError};

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
        .create_folder("/new_folder".parse().unwrap())
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
        .create_folder("/foo/new_folder".parse().unwrap())
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
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn create_with_existing_invalid_child(
    #[values("unknown_child", "child_with_different_parent", "self_reference")] kind: &str,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");

    let bad_id = env
        .customize(|builder| {
            let bad_id = match kind {
                "unknown_child" => VlobID::default(),
                "child_with_different_parent" => wksp1_id,
                "self_reference" => wksp1_foo_id,
                uknown => panic!("Unknown kind: {}", uknown),
            };
            builder
                .workspace_data_storage_local_folder_manifest_create_or_update(
                    "alice@dev1",
                    wksp1_id,
                    wksp1_foo_id,
                    None,
                )
                .customize(|e| {
                    let manifest = Arc::make_mut(&mut e.local_manifest);
                    manifest
                        .children
                        .insert("new_folder".parse().unwrap(), bad_id);
                });
            bad_id
        })
        .await;

    // Given child ID doesn't exist, the client will look for it on the server
    if matches!(kind, "unknown_child") {
        let last_common_certificate_timestamp = env.get_last_common_certificate_timestamp();
        let last_realm_certificate_timestamp = env.get_last_realm_certificate_timestamp(wksp1_id);
        test_register_sequence_of_send_hooks!(
            &env.discriminant_dir,
            move |req: authenticated_cmds::latest::vlob_read_batch::Req| {
                p_assert_eq!(req.at, None);
                p_assert_eq!(req.realm_id, wksp1_id);
                p_assert_eq!(req.vlobs, [bad_id]);
                authenticated_cmds::latest::vlob_read_batch::Rep::Ok {
                    items: vec![],
                    needed_common_certificate_timestamp: last_common_certificate_timestamp,
                    needed_realm_certificate_timestamp: last_realm_certificate_timestamp,
                }
            }
        );
    }

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let mut spy = ops.event_bus.spy.start_expecting();

    let new_folder_id = ops
        .create_folder("/foo/new_folder".parse().unwrap())
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
async fn create_as_root(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let spy = ops.event_bus.spy.start_expecting();

    let err = ops.create_folder("/".parse().unwrap()).await.unwrap_err();
    p_assert_matches!(err, WorkspaceCreateFolderError::EntryExists { entry_id } if entry_id == wksp1_id);
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
    let err = ops.create_folder(path).await.unwrap_err();
    p_assert_matches!(err, WorkspaceCreateFolderError::EntryExists { entry_id } if entry_id == already_exists_id);
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
    let err = ops.create_folder(path).await.unwrap_err();
    p_assert_matches!(err, WorkspaceCreateFolderError::EntryExists { entry_id } if entry_id == already_exists_id);
    spy.assert_no_events();

    // Ensure nothing has changed
    assert_ls!(ops, "/foo", ["egg.txt", "spam"]).await;
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn invalid_path(
    #[values("unknown_path", "invalid_child_in_path", "file_in_path")] kind: &str,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");

    let path = env
        .customize(|builder| match kind {
            "unknown_path" => "/foo/unknown/new_folder",
            "invalid_child_in_path" => {
                let bad_id = wksp1_id;
                builder
                    .workspace_data_storage_local_folder_manifest_create_or_update(
                        "alice@dev1",
                        wksp1_id,
                        wksp1_foo_id,
                        None,
                    )
                    .customize(|e| {
                        let manifest = Arc::make_mut(&mut e.local_manifest);
                        manifest.children.insert("invalid".parse().unwrap(), bad_id);
                    });
                "/foo/invalid/new_folder"
            }
            "file_in_path" => "/foo/egg.txt/new_folder",
            unknown => panic!("Unknown kind: {}", unknown),
        })
        .await;

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let spy = ops.event_bus.spy.start_expecting();

    let err = ops.create_folder(path.parse().unwrap()).await.unwrap_err();
    if matches!(kind, "file_in_path") {
        p_assert_matches!(err, WorkspaceCreateFolderError::ParentNotAFolder);
    } else {
        p_assert_matches!(err, WorkspaceCreateFolderError::ParentNotFound);
    }
    spy.assert_no_events();

    assert_ls!(ops, "/foo", ["egg.txt", "spam"]).await;
}

// TODO: test parent manifest not present in local (with and without server available)
// TODO: test entry already exists but not present in local
// TODO: test entry already exists but is a file

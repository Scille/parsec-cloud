// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// TODO: move opened file

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::{assert_ls, ls, workspace_ops_factory};
use crate::workspace::{MoveEntryMode, WorkspaceMoveEntryError};
use std::sync::Arc;

#[parsec_test(testbed = "minimal_client_ready")]
async fn ok_same_parent(#[values(false, true)] overwrite_dst: bool, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    ops.move_entry(
        "/foo/spam".parse().unwrap(),
        if overwrite_dst {
            "/foo/egg.txt"
        } else {
            "/foo/spam2"
        }
        .parse()
        .unwrap(),
        MoveEntryMode::CanReplace,
    )
    .await
    .unwrap();

    ops.rename_entry_by_id(
        wksp1_foo_id,
        "egg.txt".parse().unwrap(),
        "egg2.txt".parse().unwrap(),
        MoveEntryMode::CanReplace,
    )
    .await
    .unwrap();

    if overwrite_dst {
        assert_ls!(ops, "/foo", ["egg2.txt"]).await;
    } else {
        assert_ls!(ops, "/foo", ["egg2.txt", "spam2"]).await;
    }
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn ok_different_parents(#[values(false, true)] overwrite_dst: bool, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    ops.move_entry(
        "/bar.txt".parse().unwrap(),
        if overwrite_dst {
            "/foo/spam"
        } else {
            "/foo/bar2.txt"
        }
        .parse()
        .unwrap(),
        MoveEntryMode::CanReplace,
    )
    .await
    .unwrap();

    if overwrite_dst {
        assert_ls!(ops, "/", ["foo"]).await;
        assert_ls!(ops, "/foo", ["egg.txt", "spam"]).await;
    } else {
        assert_ls!(ops, "/", ["foo"]).await;
        assert_ls!(ops, "/foo", ["bar2.txt", "egg.txt", "spam"]).await;
    }
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn src_not_found(
    #[values("same_parents", "different_parents")] kind: &str,
    #[values(false, true)] bad_entry_in_children: bool,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");

    let env = env.customize(|builder| {
        if bad_entry_in_children {
            // Create a file with `/foo` as parent...
            let new_file_id = builder
                .create_or_update_file_manifest_vlob("alice@dev1", wksp1_id, None, wksp1_foo_id)
                .map(|e| e.manifest.id);
            // ...but list it within `/`'s manifest children !
            builder
                .create_or_update_folder_manifest_vlob("alice@dev1", wksp1_id, wksp1_id, None)
                .customize(|x| {
                    let manifest = Arc::make_mut(&mut x.manifest);
                    manifest
                        .children
                        .insert("dummy".parse().unwrap(), new_file_id);
                });
            builder.workspace_data_storage_fetch_file_vlob("alice@dev1", wksp1_id, new_file_id);
            builder.workspace_data_storage_fetch_folder_vlob(
                "alice@dev1",
                wksp1_id,
                wksp1_id,
                None,
            );
        }
    });

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let src_path = "/dummy";
    let src_parent_id = wksp1_id;
    let (dst_path, dst_parent_id) = match kind {
        "same_parents" => ("/dummy2", src_parent_id),
        "different_parents" => ("/foo/dummy2", wksp1_foo_id),
        unknown => panic!("Unknown kind: {}", unknown),
    };

    let err = ops
        .move_entry(
            src_path.parse().unwrap(),
            dst_path.parse().unwrap(),
            MoveEntryMode::CanReplace,
        )
        .await
        .unwrap_err();
    p_assert_matches!(err, WorkspaceMoveEntryError::SourceNotFound);

    if src_parent_id == dst_parent_id {
        let err = ops
            .rename_entry_by_id(
                src_parent_id,
                "dummy".parse().unwrap(),
                "dummy2".parse().unwrap(),
                MoveEntryMode::CanReplace,
            )
            .await
            .unwrap_err();
        p_assert_matches!(err, WorkspaceMoveEntryError::SourceNotFound);
    }
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn exchange_but_dst_not_found(
    #[values("same_parents", "different_parents")] kind: &str,
    #[values(false, true)] bad_entry_in_children: bool,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");

    let env = env.customize(|builder| {
        if bad_entry_in_children {
            // Create a file with `/foo` as parent...
            let new_file_id = builder
                .create_or_update_file_manifest_vlob("alice@dev1", wksp1_id, None, wksp1_foo_id)
                .map(|e| e.manifest.id);
            // ...but list it within `/`'s manifest children !
            builder
                .create_or_update_folder_manifest_vlob("alice@dev1", wksp1_id, wksp1_id, None)
                .customize(|x| {
                    let manifest = Arc::make_mut(&mut x.manifest);
                    manifest
                        .children
                        .insert("dummy2".parse().unwrap(), VlobID::default());
                });
            builder.workspace_data_storage_fetch_file_vlob("alice@dev1", wksp1_id, new_file_id);
            builder.workspace_data_storage_fetch_folder_vlob(
                "alice@dev1",
                wksp1_id,
                wksp1_id,
                None,
            );
        }
    });

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let src_path = "/bar.txt";
    let src_name = "bar.txt";
    let src_parent_id = wksp1_id;
    let dst_name = "dummy2.txt";
    let (dst_path, dst_parent_id) = match kind {
        "same_parents" => ("/dummy2.txt", src_parent_id),
        "different_parents" => ("/foo/dummy2.txt", wksp1_foo_id),
        unknown => panic!("Unknown kind: {}", unknown),
    };

    let err = ops
        .move_entry(
            src_path.parse().unwrap(),
            dst_path.parse().unwrap(),
            MoveEntryMode::Exchange,
        )
        .await
        .unwrap_err();
    p_assert_matches!(err, WorkspaceMoveEntryError::DestinationNotFound);

    if src_parent_id == dst_parent_id {
        let err = ops
            .rename_entry_by_id(
                src_parent_id,
                src_name.parse().unwrap(),
                dst_name.parse().unwrap(),
                MoveEntryMode::Exchange,
            )
            .await
            .unwrap_err();
        p_assert_matches!(err, WorkspaceMoveEntryError::DestinationNotFound);
    }
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn no_replace_but_dst_exists(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let err = ops
        .move_entry(
            "/foo".parse().unwrap(),
            "/bar.txt".parse().unwrap(),
            MoveEntryMode::NoReplace,
        )
        .await
        .unwrap_err();
    p_assert_matches!(err, WorkspaceMoveEntryError::DestinationExists { entry_id } if entry_id == wksp1_bar_txt_id);

    let err = ops
        .rename_entry_by_id(
            wksp1_id,
            "foo".parse().unwrap(),
            "bar.txt".parse().unwrap(),
            MoveEntryMode::NoReplace,
        )
        .await
        .unwrap_err();
    p_assert_matches!(err, WorkspaceMoveEntryError::DestinationExists { entry_id } if entry_id == wksp1_bar_txt_id);
}

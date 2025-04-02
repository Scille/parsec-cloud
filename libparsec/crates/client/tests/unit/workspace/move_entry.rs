// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// TODO: move opened file

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::{assert_ls_with_id, workspace_ops_factory};
use crate::workspace::{MoveEntryMode, WorkspaceMoveEntryError};
use std::sync::Arc;

#[parsec_test(testbed = "minimal_client_ready")]
async fn ok_same_parent(
    #[values("empty_dst", "overwrite_any", "overwrite_file_only", "exchange")] kind: &str,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_foo_egg_txt_id: VlobID = *env.template.get_stuff("wksp1_foo_egg_txt_id");
    let wksp1_foo_spam_id: VlobID = *env.template.get_stuff("wksp1_foo_spam_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let rename_entry_by_id_src = "bar.txt";
    let move_entry_src = "/foo/spam";
    let (
        mode,
        rename_entry_by_id_dst,
        move_entry_dst,
        expected_root_children,
        expected_foo_children,
    ) = match kind {
        "empty_dst" => (
            MoveEntryMode::NoReplace,
            "bar2.txt",
            "/foo/spam2",
            vec![("bar2.txt", wksp1_bar_txt_id), ("foo", wksp1_foo_id)],
            vec![
                ("egg.txt", wksp1_foo_egg_txt_id),
                ("spam2", wksp1_foo_spam_id),
            ],
        ),
        "overwrite_any" => {
            ops.create_file("/other.txt".parse().unwrap())
                .await
                .unwrap();
            (
                MoveEntryMode::CanReplace,
                "other.txt",
                "/foo/egg.txt",
                vec![("foo", wksp1_foo_id), ("other.txt", wksp1_bar_txt_id)],
                vec![("egg.txt", wksp1_foo_spam_id)],
            )
        }
        "overwrite_file_only" => {
            ops.create_file("/other.txt".parse().unwrap())
                .await
                .unwrap();
            (
                MoveEntryMode::CanReplaceFileOnly,
                "other.txt",
                "/foo/egg.txt",
                vec![("foo", wksp1_foo_id), ("other.txt", wksp1_bar_txt_id)],
                vec![("egg.txt", wksp1_foo_spam_id)],
            )
        }
        "exchange" => {
            let wksp1_other_txt_id = ops
                .create_file("/other.txt".parse().unwrap())
                .await
                .unwrap();
            (
                MoveEntryMode::Exchange,
                "other.txt",
                "/foo/egg.txt",
                vec![
                    ("bar.txt", wksp1_other_txt_id),
                    ("foo", wksp1_foo_id),
                    ("other.txt", wksp1_bar_txt_id),
                ],
                vec![
                    ("egg.txt", wksp1_foo_spam_id),
                    ("spam", wksp1_foo_egg_txt_id),
                ],
            )
        }
        unknown => panic!("Unknown kind: {}", unknown),
    };

    ops.move_entry(
        move_entry_src.parse().unwrap(),
        move_entry_dst.parse().unwrap(),
        mode,
    )
    .await
    .unwrap();

    ops.rename_entry_by_id(
        wksp1_id,
        rename_entry_by_id_src.parse().unwrap(),
        rename_entry_by_id_dst.parse().unwrap(),
        mode,
    )
    .await
    .unwrap();

    assert_ls_with_id!(ops, "/", expected_root_children).await;
    assert_ls_with_id!(ops, "/foo", expected_foo_children).await;
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn ok_different_parents(
    #[values("empty_dst", "overwrite_any", "overwrite_file_only", "exchange")] kind: &str,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_foo_egg_txt_id: VlobID = *env.template.get_stuff("wksp1_foo_egg_txt_id");
    let wksp1_foo_spam_id: VlobID = *env.template.get_stuff("wksp1_foo_spam_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let move_entry_src = "/foo/spam";
    let (mode, move_entry_dst, expected_root_children, expected_foo_children) = match kind {
        "empty_dst" => (
            MoveEntryMode::NoReplace,
            "/spam2",
            vec![
                ("bar.txt", wksp1_bar_txt_id),
                ("foo", wksp1_foo_id),
                ("spam2", wksp1_foo_spam_id),
            ],
            vec![("egg.txt", wksp1_foo_egg_txt_id)],
        ),
        "overwrite_any" => (
            MoveEntryMode::CanReplace,
            "/bar.txt",
            vec![("bar.txt", wksp1_foo_spam_id), ("foo", wksp1_foo_id)],
            vec![("egg.txt", wksp1_foo_egg_txt_id)],
        ),
        "overwrite_file_only" => (
            MoveEntryMode::CanReplaceFileOnly,
            "/bar.txt",
            vec![("bar.txt", wksp1_foo_spam_id), ("foo", wksp1_foo_id)],
            vec![("egg.txt", wksp1_foo_egg_txt_id)],
        ),
        "exchange" => (
            MoveEntryMode::Exchange,
            "/bar.txt",
            vec![("bar.txt", wksp1_foo_spam_id), ("foo", wksp1_foo_id)],
            vec![
                ("egg.txt", wksp1_foo_egg_txt_id),
                ("spam", wksp1_bar_txt_id),
            ],
        ),
        unknown => panic!("Unknown kind: {}", unknown),
    };

    ops.move_entry(
        move_entry_src.parse().unwrap(),
        move_entry_dst.parse().unwrap(),
        mode,
    )
    .await
    .unwrap();

    assert_ls_with_id!(ops, "/", expected_root_children).await;
    assert_ls_with_id!(ops, "/foo", expected_foo_children).await;
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn src_not_found(
    #[values("same_parents", "different_parents")] kind: &str,
    #[values(false, true)] bad_entry_in_children: bool,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");

    env.customize(|builder| {
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
                libparsec_types::PreventSyncPattern::empty(),
            );
        }
    })
    .await;

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

    env.customize(|builder| {
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
                libparsec_types::PreventSyncPattern::empty(),
            );
        }
    })
    .await;

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

#[parsec_test(testbed = "minimal_client_ready")]
async fn replace_file_only_but_dst_is_folder(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let err = ops
        .move_entry(
            "/bar.txt".parse().unwrap(),
            "/foo".parse().unwrap(),
            MoveEntryMode::CanReplaceFileOnly,
        )
        .await
        .unwrap_err();
    p_assert_matches!(err, WorkspaceMoveEntryError::DestinationExists { entry_id } if entry_id == wksp1_foo_id);

    let err = ops
        .rename_entry_by_id(
            wksp1_id,
            "bar.txt".parse().unwrap(),
            "foo".parse().unwrap(),
            MoveEntryMode::CanReplaceFileOnly,
        )
        .await
        .unwrap_err();
    p_assert_matches!(err, WorkspaceMoveEntryError::DestinationExists { entry_id } if entry_id == wksp1_foo_id);
}

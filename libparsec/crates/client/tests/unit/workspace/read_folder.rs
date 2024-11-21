// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    protocol::authenticated_cmds, test_register_sequence_of_send_hooks,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::workspace_ops_factory;
use crate::workspace::{
    transactions::FolderReaderStatNextOutcome, EntryStat, MoveEntryMode, OpenOptions,
};

fn expect_entry(stat: FolderReaderStatNextOutcome<'_>) -> (&EntryName, EntryStat) {
    match stat {
        FolderReaderStatNextOutcome::Entry { name, stat } => (name, stat),
        _ => panic!("Expected an entry, got {:?}", stat),
    }
}

fn expect_no_more_entries(stat: FolderReaderStatNextOutcome<'_>) {
    if !matches!(stat, FolderReaderStatNextOutcome::NoMoreEntries) {
        panic!("Expected no more entries, got {:?}", stat);
    }
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn ok_with_local_cache(#[values(true, false)] target_is_root: bool, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_foo_spam_id: VlobID = *env.template.get_stuff("wksp1_foo_spam_id");
    let wksp1_foo_egg_txt_id: VlobID = *env.template.get_stuff("wksp1_foo_egg_txt_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let (target_path, target_id, expected_ordered_children_stats) = if target_is_root {
        let target_path = "/".parse().unwrap();
        let target_id = wksp1_id;
        let expected_ordered_children_stats = [
            (
                "bar.txt".parse().unwrap(),
                EntryStat::File {
                    confinement_point: None,
                    id: wksp1_bar_txt_id,
                    parent: wksp1_id,
                    created: "2000-01-07T00:00:00Z".parse().unwrap(),
                    updated: "2000-01-07T00:00:00Z".parse().unwrap(),
                    base_version: 1,
                    is_placeholder: false,
                    need_sync: false,
                    size: 11,
                    last_updater: alice.device_id,
                },
            ),
            (
                "foo".parse().unwrap(),
                EntryStat::Folder {
                    confinement_point: None,
                    id: wksp1_foo_id,
                    parent: wksp1_id,
                    created: "2000-01-10T00:00:00Z".parse().unwrap(),
                    updated: "2000-01-10T00:00:00Z".parse().unwrap(),
                    base_version: 1,
                    is_placeholder: false,
                    need_sync: false,
                    last_updater: alice.device_id,
                },
            ),
        ];
        (target_path, target_id, expected_ordered_children_stats)
    } else {
        let target_path = "/foo".parse().unwrap();
        let target_id = wksp1_foo_id;
        let expected_ordered_children_stats = [
            (
                "egg.txt".parse().unwrap(),
                EntryStat::File {
                    confinement_point: None,
                    id: wksp1_foo_egg_txt_id,
                    parent: wksp1_foo_id,
                    created: "2000-01-09T00:00:00Z".parse().unwrap(),
                    updated: "2000-01-09T00:00:00Z".parse().unwrap(),
                    base_version: 1,
                    is_placeholder: false,
                    need_sync: false,
                    size: 0,
                    last_updater: alice.device_id,
                },
            ),
            (
                "spam".parse().unwrap(),
                EntryStat::Folder {
                    confinement_point: None,
                    id: wksp1_foo_spam_id,
                    parent: wksp1_foo_id,
                    created: "2000-01-08T00:00:00Z".parse().unwrap(),
                    updated: "2000-01-08T00:00:00Z".parse().unwrap(),
                    base_version: 1,
                    is_placeholder: false,
                    need_sync: false,
                    last_updater: alice.device_id,
                },
            ),
        ];
        (target_path, target_id, expected_ordered_children_stats)
    };

    // 1) Test `open_folder_reader`

    let reader = ops.open_folder_reader(&target_path).await.unwrap();

    let a0 = expect_entry(reader.stat_child(&ops, 0).await.unwrap());
    let a1 = expect_entry(reader.stat_child(&ops, 1).await.unwrap());
    expect_no_more_entries(reader.stat_child(&ops, 2).await.unwrap());

    let a0_retry = expect_entry(reader.stat_child(&ops, 0).await.unwrap());
    p_assert_eq!(a0, a0_retry);

    p_assert_eq!(
        {
            let mut items: Vec<_> = [a0, a1]
                .into_iter()
                .map(|(name_ref, stat)| (name_ref.to_owned(), stat))
                .collect();
            items.sort_by(|a, b| a.0.cmp(&b.0));
            items
        },
        expected_ordered_children_stats,
    );

    // 2) Test `open_folder_reader_by_id`

    let reader = ops.open_folder_reader_by_id(target_id).await.unwrap();

    let a0 = expect_entry(reader.stat_child(&ops, 0).await.unwrap());
    let a1 = expect_entry(reader.stat_child(&ops, 1).await.unwrap());
    expect_no_more_entries(reader.stat_child(&ops, 2).await.unwrap());

    let a0_retry = expect_entry(reader.stat_child(&ops, 0).await.unwrap());
    p_assert_eq!(a0, a0_retry);

    p_assert_eq!(
        {
            let mut items: Vec<_> = [a0, a1]
                .into_iter()
                .map(|(name_ref, stat)| (name_ref.to_owned(), stat))
                .collect();
            items.sort_by(|a, b| a.0.cmp(&b.0));
            items
        },
        expected_ordered_children_stats,
    );

    // 3) Test `stat_folder_children`

    let mut children_stats = ops.stat_folder_children(&target_path).await.unwrap();
    children_stats.sort_by(|a, b| a.0.cmp(&b.0));
    p_assert_eq!(children_stats, expected_ordered_children_stats,);
}

#[parsec_test(testbed = "minimal_client_ready", with_server)]
async fn ok_no_local_cache(#[values(true, false)] target_is_root: bool, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_foo_spam_id: VlobID = *env.template.get_stuff("wksp1_foo_spam_id");
    let wksp1_foo_egg_txt_id: VlobID = *env.template.get_stuff("wksp1_foo_egg_txt_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");

    env.customize(|builder| {
        builder.filter_client_storage_events(|event| match event {
            // Missing workspace manifest is replaced by a speculative one (so no
            // server fetch will occur), that's not what we want here !
            TestbedEvent::WorkspaceDataStorageFetchFolderVlob(e)
                if e.local_manifest.base.is_root() =>
            {
                true
            }
            TestbedEvent::WorkspaceDataStorageFetchFileVlob(_)
            | TestbedEvent::WorkspaceDataStorageFetchFolderVlob(_)
            | TestbedEvent::WorkspaceCacheStorageFetchBlock(_) => false,
            _ => true,
        });
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let (target_path, target_id, expected_ordered_children_stats) = if target_is_root {
        let target_path = "/".parse().unwrap();
        let target_id = wksp1_id;
        let expected_ordered_children_stats = [
            (
                "bar.txt".parse().unwrap(),
                EntryStat::File {
                    confinement_point: None,
                    id: wksp1_bar_txt_id,
                    parent: wksp1_id,
                    created: "2000-01-07T00:00:00Z".parse().unwrap(),
                    updated: "2000-01-07T00:00:00Z".parse().unwrap(),
                    base_version: 1,
                    is_placeholder: false,
                    need_sync: false,
                    size: 11,
                    last_updater: alice.device_id,
                },
            ),
            (
                "foo".parse().unwrap(),
                EntryStat::Folder {
                    confinement_point: None,
                    id: wksp1_foo_id,
                    parent: wksp1_id,
                    created: "2000-01-10T00:00:00Z".parse().unwrap(),
                    updated: "2000-01-10T00:00:00Z".parse().unwrap(),
                    base_version: 1,
                    is_placeholder: false,
                    need_sync: false,
                    last_updater: alice.device_id,
                },
            ),
        ];
        (target_path, target_id, expected_ordered_children_stats)
    } else {
        let target_path = "/foo".parse().unwrap();
        let target_id = wksp1_foo_id;
        let expected_ordered_children_stats = [
            (
                "egg.txt".parse().unwrap(),
                EntryStat::File {
                    confinement_point: None,
                    id: wksp1_foo_egg_txt_id,
                    parent: wksp1_foo_id,
                    created: "2000-01-09T00:00:00Z".parse().unwrap(),
                    updated: "2000-01-09T00:00:00Z".parse().unwrap(),
                    base_version: 1,
                    is_placeholder: false,
                    need_sync: false,
                    size: 0,
                    last_updater: alice.device_id,
                },
            ),
            (
                "spam".parse().unwrap(),
                EntryStat::Folder {
                    confinement_point: None,
                    id: wksp1_foo_spam_id,
                    parent: wksp1_foo_id,
                    created: "2000-01-08T00:00:00Z".parse().unwrap(),
                    updated: "2000-01-08T00:00:00Z".parse().unwrap(),
                    base_version: 1,
                    is_placeholder: false,
                    need_sync: false,
                    last_updater: alice.device_id,
                },
            ),
        ];
        (target_path, target_id, expected_ordered_children_stats)
    };

    // 1) Test `open_folder_reader`

    let reader = ops.open_folder_reader(&target_path).await.unwrap();

    let a0 = expect_entry(reader.stat_child(&ops, 0).await.unwrap());
    let a1 = expect_entry(reader.stat_child(&ops, 1).await.unwrap());
    expect_no_more_entries(reader.stat_child(&ops, 2).await.unwrap());

    let a0_retry = expect_entry(reader.stat_child(&ops, 0).await.unwrap());
    p_assert_eq!(a0, a0_retry);

    p_assert_eq!(
        {
            let mut items: Vec<_> = [a0, a1]
                .into_iter()
                .map(|(name_ref, stat)| (name_ref.to_owned(), stat))
                .collect();
            items.sort_by(|a, b| a.0.cmp(&b.0));
            items
        },
        expected_ordered_children_stats,
    );

    // 2) Test `open_folder_reader_by_id`

    let reader = ops.open_folder_reader_by_id(target_id).await.unwrap();

    let a0 = expect_entry(reader.stat_child(&ops, 0).await.unwrap());
    let a1 = expect_entry(reader.stat_child(&ops, 1).await.unwrap());
    expect_no_more_entries(reader.stat_child(&ops, 2).await.unwrap());

    let a0_retry = expect_entry(reader.stat_child(&ops, 0).await.unwrap());
    p_assert_eq!(a0, a0_retry);

    p_assert_eq!(
        {
            let mut items: Vec<_> = [a0, a1]
                .into_iter()
                .map(|(name_ref, stat)| (name_ref.to_owned(), stat))
                .collect();
            items.sort_by(|a, b| a.0.cmp(&b.0));
            items
        },
        expected_ordered_children_stats,
    );

    // 3) Test `stat_folder_children`

    let mut children_stats = ops.stat_folder_children(&target_path).await.unwrap();
    children_stats.sort_by(|a, b| a.0.cmp(&b.0));
    p_assert_eq!(children_stats, expected_ordered_children_stats,);
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn ignore_invalid_children(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_egg_txt_id: VlobID = *env.template.get_stuff("wksp1_foo_egg_txt_id");
    let non_existing_id = VlobID::default();
    let bad_parent_id = wksp1_foo_egg_txt_id;
    env.customize(|builder| {
        builder
            .workspace_data_storage_local_workspace_manifest_update("alice@dev1", wksp1_id)
            .customize_children(
                [
                    // Non existing entry
                    ("non_existing.txt", Some(non_existing_id)),
                    // Existing entry, but with a parent field not pointing to us
                    ("bad_parent.txt", Some(bad_parent_id)),
                ]
                .into_iter(),
            );
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let last_common_certificate_timestamp = env.get_last_common_certificate_timestamp();
    let last_realm_certificate_timestamp = env.get_last_realm_certificate_timestamp(wksp1_id);
    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // Client will try to fetch the non existing ID from the server
        move |req: authenticated_cmds::latest::vlob_read_batch::Req| {
            p_assert_eq!(req.realm_id, wksp1_id);
            p_assert_eq!(req.vlobs, [non_existing_id]);
            p_assert_eq!(req.at, None);
            authenticated_cmds::latest::vlob_read_batch::Rep::Ok {
                items: vec![],
                needed_common_certificate_timestamp: last_common_certificate_timestamp,
                needed_realm_certificate_timestamp: last_realm_certificate_timestamp,
            }
        }
    );

    // Note we don't test `WorkspaceOps::open_folder_reader` given
    // `WorkspaceOps::stat_folder_children` is just a thin wrapper around it
    let children_stats = ops
        .stat_folder_children(&"/".parse().unwrap())
        .await
        .unwrap();

    let mut children_names = children_stats
        .iter()
        .map(|(entry_name, _)| entry_name.to_string())
        .collect::<Vec<_>>();
    children_names.sort();
    p_assert_eq!(children_names, ["bar.txt", "foo"]);

    p_assert_eq!(
        children_stats
            .iter()
            .any(|(_, stat)| stat.id() == non_existing_id),
        false
    );
    p_assert_eq!(
        children_stats
            .iter()
            .any(|(_, stat)| stat.id() == bad_parent_id),
        false
    );
}

// TODO: having multiple children that point to the same entry ID is currently accepted, should we do something about it ?

#[parsec_test(testbed = "minimal_client_ready")]
async fn read_folder_with_confined_entries(
    #[values(true, false)] folder_is_confined: bool,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_foo_spam_id: VlobID = *env.template.get_stuff("wksp1_foo_spam_id");
    let wksp1_foo_egg_txt_id: VlobID = *env.template.get_stuff("wksp1_foo_egg_txt_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    // Rename foo to foo.tmp
    let target_path;
    let expected_egg_txt_confinement_point;
    let expected_spam_confinement_point;
    if folder_is_confined {
        ops.rename_entry_by_id(
            wksp1_id,
            "foo".parse().unwrap(),
            "foo.tmp".parse().unwrap(),
            MoveEntryMode::NoReplace,
        )
        .await
        .unwrap();
        target_path = "/foo.tmp".parse().unwrap();
        expected_egg_txt_confinement_point = Some(wksp1_id);
        expected_spam_confinement_point = Some(wksp1_id);
    } else {
        target_path = "/foo".parse().unwrap();
        expected_egg_txt_confinement_point = Some(wksp1_foo_id);
        expected_spam_confinement_point = None;
    }

    // Rename egg.txt to egg.txt.tmp
    ops.rename_entry_by_id(
        wksp1_foo_id,
        "egg.txt".parse().unwrap(),
        "egg.txt.tmp".parse().unwrap(),
        MoveEntryMode::NoReplace,
    )
    .await
    .unwrap();

    let target_id = wksp1_foo_id;
    let expected_ordered_children_stats = [
        (
            "egg.txt.tmp".parse().unwrap(),
            EntryStat::File {
                confinement_point: expected_egg_txt_confinement_point,
                id: wksp1_foo_egg_txt_id,
                parent: wksp1_foo_id,
                created: "2000-01-09T00:00:00Z".parse().unwrap(),
                updated: "2000-01-09T00:00:00Z".parse().unwrap(),
                base_version: 1,
                is_placeholder: false,
                need_sync: false,
                size: 0,
                last_updater: alice.device_id,
            },
        ),
        (
            "spam".parse().unwrap(),
            EntryStat::Folder {
                confinement_point: expected_spam_confinement_point,
                id: wksp1_foo_spam_id,
                parent: wksp1_foo_id,
                created: "2000-01-08T00:00:00Z".parse().unwrap(),
                updated: "2000-01-08T00:00:00Z".parse().unwrap(),
                base_version: 1,
                is_placeholder: false,
                need_sync: false,
                last_updater: alice.device_id,
            },
        ),
    ];

    // 1) Test `open_folder_reader`

    let reader = ops.open_folder_reader(&target_path).await.unwrap();

    let a0 = expect_entry(reader.stat_child(&ops, 0).await.unwrap());
    let a1 = expect_entry(reader.stat_child(&ops, 1).await.unwrap());
    expect_no_more_entries(reader.stat_child(&ops, 2).await.unwrap());

    let a0_retry = expect_entry(reader.stat_child(&ops, 0).await.unwrap());
    p_assert_eq!(a0, a0_retry);

    p_assert_eq!(
        {
            let mut items: Vec<_> = [a0, a1]
                .into_iter()
                .map(|(name_ref, stat)| (name_ref.to_owned(), stat))
                .collect();
            items.sort_by(|a, b| a.0.cmp(&b.0));
            items
        },
        expected_ordered_children_stats,
    );

    // 2) Test `open_folder_reader_by_id`

    let reader = ops.open_folder_reader_by_id(target_id).await.unwrap();

    let a0 = expect_entry(reader.stat_child(&ops, 0).await.unwrap());
    let a1 = expect_entry(reader.stat_child(&ops, 1).await.unwrap());
    expect_no_more_entries(reader.stat_child(&ops, 2).await.unwrap());

    let a0_retry = expect_entry(reader.stat_child(&ops, 0).await.unwrap());
    p_assert_eq!(a0, a0_retry);

    p_assert_eq!(
        {
            let mut items: Vec<_> = [a0, a1]
                .into_iter()
                .map(|(name_ref, stat)| (name_ref.to_owned(), stat))
                .collect();
            items.sort_by(|a, b| a.0.cmp(&b.0));
            items
        },
        expected_ordered_children_stats,
    );

    // 3) Test `stat_folder_children`

    let mut children_stats = ops.stat_folder_children(&target_path).await.unwrap();
    children_stats.sort_by(|a, b| a.0.cmp(&b.0));
    p_assert_eq!(children_stats, expected_ordered_children_stats,);
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn read_folder_containing_under_modification_file(
    #[values("truncate_on_open", "append_data", "truncate_data")] kind: &str,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let mut open_options = OpenOptions {
        read: false,
        write: true,
        truncate: false,
        create: false,
        create_new: false,
    };
    let path: FsPath = "/bar.txt".parse().unwrap();

    alice
        .time_provider
        .mock_time_frozen("2020-01-01T00:00:00Z".parse().unwrap());

    let expected_size = match kind {
        "truncate_on_open" => {
            open_options.truncate = true;
            ops.open_file(path.clone(), open_options).await.unwrap();
            0
        }
        "append_data" => {
            let fd = ops.open_file(path.clone(), open_options).await.unwrap();
            ops.fd_write(fd, 5, b"12345678901234567890").await.unwrap();
            25
        }
        "truncate_data" => {
            let fd = ops.open_file(path.clone(), open_options).await.unwrap();
            ops.fd_resize(fd, 5, true).await.unwrap();
            5
        }
        unknown => panic!("Unknown kind {}", unknown),
    };

    alice.time_provider.unmock_time();

    // The file is still opened and no flush has occurred yet!

    let mut children_stats = ops
        .stat_folder_children(&"/".parse().unwrap())
        .await
        .unwrap();
    children_stats.sort_by(|a, b| a.0.cmp(&b.0));

    let expected_ordered_children_stats = [
        (
            "bar.txt".parse().unwrap(),
            EntryStat::File {
                confinement_point: None,
                id: wksp1_bar_txt_id,
                parent: wksp1_id,
                created: "2000-01-07T00:00:00Z".parse().unwrap(),
                updated: "2020-01-01T00:00:00Z".parse().unwrap(),
                base_version: 1,
                is_placeholder: false,
                need_sync: true,
                size: expected_size,
                last_updater: alice.device_id,
            },
        ),
        (
            "foo".parse().unwrap(),
            EntryStat::Folder {
                confinement_point: None,
                id: wksp1_foo_id,
                parent: wksp1_id,
                created: "2000-01-10T00:00:00Z".parse().unwrap(),
                updated: "2000-01-10T00:00:00Z".parse().unwrap(),
                base_version: 1,
                is_placeholder: false,
                need_sync: false,
                last_updater: alice.device_id,
            },
        ),
    ];
    p_assert_eq!(children_stats, expected_ordered_children_stats);
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    protocol::authenticated_cmds, test_register_sequence_of_send_hooks,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::workspace_ops_factory;
use crate::workspace::EntryStat;

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
                },
            ),
        ];
        (target_path, target_id, expected_ordered_children_stats)
    };

    // 1) Test `open_folder_reader`

    let reader = ops.open_folder_reader(&target_path).await.unwrap();

    let a0 = reader.stat_next(&ops, 0).await.unwrap().unwrap();
    let a1 = reader.stat_next(&ops, 1).await.unwrap().unwrap();
    let a2 = reader.stat_next(&ops, 2).await.unwrap();
    assert!(a2.is_none());

    let a0_retry = reader.stat_next(&ops, 0).await.unwrap().unwrap();
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

    let a0 = reader.stat_next(&ops, 0).await.unwrap().unwrap();
    let a1 = reader.stat_next(&ops, 1).await.unwrap().unwrap();
    let a2 = reader.stat_next(&ops, 2).await.unwrap();
    assert!(a2.is_none());

    let a0_retry = reader.stat_next(&ops, 0).await.unwrap().unwrap();
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
    });

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
                },
            ),
        ];
        (target_path, target_id, expected_ordered_children_stats)
    };

    // 1) Test `open_folder_reader`

    let reader = ops.open_folder_reader(&target_path).await.unwrap();

    let a0 = reader.stat_next(&ops, 0).await.unwrap().unwrap();
    let a1 = reader.stat_next(&ops, 1).await.unwrap().unwrap();
    let a2 = reader.stat_next(&ops, 2).await.unwrap();
    assert!(a2.is_none());

    let a0_retry = reader.stat_next(&ops, 0).await.unwrap().unwrap();
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

    let a0 = reader.stat_next(&ops, 0).await.unwrap().unwrap();
    let a1 = reader.stat_next(&ops, 1).await.unwrap().unwrap();
    let a2 = reader.stat_next(&ops, 2).await.unwrap();
    assert!(a2.is_none());

    let a0_retry = reader.stat_next(&ops, 0).await.unwrap().unwrap();
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
    let env = env.customize(|builder| {
        builder
            .workspace_data_storage_local_workspace_manifest_update("alice@dev1", wksp1_id)
            .customize_children(
                [
                    // Non existing entry
                    ("non_existing.txt", Some(non_existing_id)),
                    // Existing entry, but with a parent field not pointing to us
                    ("bad_parent.txt", Some(wksp1_foo_egg_txt_id)),
                ]
                .into_iter(),
            );
    });

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

    let children_stats = ops
        .stat_folder_children(&"/".parse().unwrap())
        .await
        .unwrap();
    p_assert_eq!(
        children_stats
            .iter()
            .any(|(name, _)| name.as_ref() == "non_existing.txt"),
        false
    );
    p_assert_eq!(
        children_stats
            .iter()
            .any(|(name, _)| name.as_ref() == "bad_parent.txt"),
        false
    );
}

// TODO: having multiple children that point to the same entry ID is currently accepted, should we do something about it ?

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::workspace_ops_factory;
use crate::workspace::{store::PathConfinementPoint, EntryStat, OpenOptions, WorkspaceOps};

#[parsec_test(testbed = "minimal_client_ready", with_server)]
async fn stat_entry(#[values(true, false)] local_cache: bool, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");

    if !local_cache {
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
    }

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    // Workspace

    let info = ops.stat_entry(&"/".parse().unwrap()).await.unwrap();
    p_assert_matches!(
        info,
        EntryStat::Folder{
            confinement_point,
            id,
            parent,
            created,
            updated,
            base_version,
            is_placeholder,
            need_sync,
            last_updater
        }
        if {
            p_assert_eq!(confinement_point, None);
            p_assert_eq!(id, wksp1_id);
            p_assert_eq!(parent, wksp1_id);
            p_assert_eq!(created, "2000-01-11T00:00:00Z".parse().unwrap());
            p_assert_eq!(updated, "2000-01-11T00:00:00Z".parse().unwrap());
            p_assert_eq!(base_version, 1);
            p_assert_eq!(is_placeholder, false);
            p_assert_eq!(need_sync, false);
            p_assert_eq!(last_updater, alice.device_id);
            true
        }
    );

    // Folder

    let info = ops.stat_entry(&"/foo".parse().unwrap()).await.unwrap();
    p_assert_matches!(
        info,
        EntryStat::Folder{
            confinement_point,
            id,
            parent,
            created,
            updated,
            base_version,
            is_placeholder,
            need_sync,
            last_updater
        }
        if {
            p_assert_eq!(confinement_point, None);
            p_assert_eq!(id, wksp1_foo_id);
            p_assert_eq!(parent, wksp1_id);
            p_assert_eq!(created, "2000-01-10T00:00:00Z".parse().unwrap());
            p_assert_eq!(updated, "2000-01-10T00:00:00Z".parse().unwrap());
            p_assert_eq!(base_version, 1);
            p_assert_eq!(is_placeholder, false);
            p_assert_eq!(need_sync, false);
            p_assert_eq!(last_updater, alice.device_id);

            true
        }
    );

    // File

    let info = ops.stat_entry(&"/bar.txt".parse().unwrap()).await.unwrap();
    p_assert_matches!(
        info,
        EntryStat::File{
            confinement_point,
            id,
            parent,
            created,
            updated,
            base_version,
            is_placeholder,
            need_sync,
            size,
            last_updater
        }
        if {
            p_assert_eq!(confinement_point, None);
            p_assert_eq!(id, wksp1_bar_txt_id);
            p_assert_eq!(parent, wksp1_id);
            p_assert_eq!(created, "2000-01-07T00:00:00Z".parse().unwrap());
            p_assert_eq!(updated, "2000-01-07T00:00:00Z".parse().unwrap());
            p_assert_eq!(base_version, 1);
            p_assert_eq!(is_placeholder, false);
            p_assert_eq!(need_sync, false);
            p_assert_eq!(size, 11);  // Contains "hello world"
            p_assert_eq!(last_updater, alice.device_id);

            true
        }
    );
}

async fn stat_entry_by_id_helper(
    ops: &WorkspaceOps,
    entry_id: VlobID,
    kind: &'_ str,
    expected_confinement_point: Option<VlobID>,
) -> (EntryStat, Option<VlobID>) {
    match kind {
        "find_confinement_point" => (
            ops.stat_entry_by_id(entry_id).await.unwrap(),
            expected_confinement_point,
        ),
        "ignore_confinement_point" => (
            ops.stat_entry_by_id_ignore_confinement_point(entry_id)
                .await
                .unwrap(),
            None,
        ),
        "known_confinement_point" => {
            let new_id = VlobID::default();
            let known_confinement_point = PathConfinementPoint::Confined(new_id);
            (
                ops.stat_entry_by_id_with_known_confinement_point(
                    entry_id,
                    known_confinement_point,
                )
                .await
                .unwrap(),
                Some(new_id),
            )
        }
        _ => panic!("Unexpected kind {}", kind),
    }
}

#[parsec_test(testbed = "minimal_client_ready", with_server)]
async fn stat_entry_by_id(
    #[values(true, false)] local_cache: bool,
    #[values(
        "find_confinement_point",
        "ignore_confinement_point",
        "known_confinement_point"
    )]
    kind: &'_ str,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");

    if !local_cache {
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
    }

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    // Workspace

    let (info, expected_confinement_point) =
        stat_entry_by_id_helper(&ops, wksp1_id, kind, None).await;
    p_assert_matches!(
        info,
        EntryStat::Folder{
            confinement_point,
            id,
            parent,
            created,
            updated,
            base_version,
            is_placeholder,
            need_sync,
            last_updater
        }
        if {
            p_assert_eq!(confinement_point, expected_confinement_point);
            p_assert_eq!(id, wksp1_id);
            p_assert_eq!(parent, wksp1_id);
            p_assert_eq!(created, "2000-01-11T00:00:00Z".parse().unwrap());
            p_assert_eq!(updated, "2000-01-11T00:00:00Z".parse().unwrap());
            p_assert_eq!(base_version, 1);
            p_assert_eq!(is_placeholder, false);
            p_assert_eq!(need_sync, false);
            p_assert_eq!(last_updater, alice.device_id);

            true
        }
    );

    // Folder

    let (info, expected_confinement_point) =
        stat_entry_by_id_helper(&ops, wksp1_foo_id, kind, None).await;
    p_assert_matches!(
        info,
        EntryStat::Folder{
            confinement_point,
            id,
            parent,
            created,
            updated,
            base_version,
            is_placeholder,
            need_sync,
            last_updater
        }
        if {
            p_assert_eq!(confinement_point, expected_confinement_point);
            p_assert_eq!(id, wksp1_foo_id);
            p_assert_eq!(parent, wksp1_id);
            p_assert_eq!(created, "2000-01-10T00:00:00Z".parse().unwrap());
            p_assert_eq!(updated, "2000-01-10T00:00:00Z".parse().unwrap());
            p_assert_eq!(base_version, 1);
            p_assert_eq!(is_placeholder, false);
            p_assert_eq!(need_sync, false);
            p_assert_eq!(last_updater, alice.device_id);

            true
        }
    );

    // File
    let (info, expected_confinement_point) =
        stat_entry_by_id_helper(&ops, wksp1_bar_txt_id, kind, None).await;
    p_assert_matches!(
        info,
        EntryStat::File{
            confinement_point,
            id,
            parent,
            created,
            updated,
            base_version,
            is_placeholder,
            need_sync,
            size,
            last_updater
        }
        if {
            p_assert_eq!(confinement_point, expected_confinement_point);
            p_assert_eq!(id, wksp1_bar_txt_id);
            p_assert_eq!(parent, wksp1_id);
            p_assert_eq!(created, "2000-01-07T00:00:00Z".parse().unwrap());
            p_assert_eq!(updated, "2000-01-07T00:00:00Z".parse().unwrap());
            p_assert_eq!(base_version, 1);
            p_assert_eq!(is_placeholder, false);
            p_assert_eq!(need_sync, false);
            p_assert_eq!(size, 11);  // Contains "hello world"
            p_assert_eq!(last_updater, alice.device_id);

            true
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn stat_entry_on_speculative_workspace(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    env.customize(|builder| {
        builder.filter_client_storage_events(|event| match event {
            // Don't remove the children manifests from storage: they should be
            // ignored given the speculative workspace manifest doesn't mention them.
            TestbedEvent::WorkspaceDataStorageFetchFolderVlob(e)
                if e.local_manifest.base.is_root() =>
            {
                false
            }
            _ => true,
        });
    })
    .await;

    let alice = env.local_device("alice@dev1");

    let now: DateTime = "2000-12-31T00:00:00Z".parse().unwrap();
    alice.time_provider.mock_time_frozen(now);

    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    let info = ops.stat_entry(&"/".parse().unwrap()).await.unwrap();

    p_assert_matches!(
        info,
        EntryStat::Folder{
            confinement_point,
            id,
            parent,
            created,
            updated,
            base_version,
            is_placeholder,
            need_sync,
            last_updater,
        } if {
            p_assert_eq!(confinement_point, None);
            p_assert_eq!(id, wksp1_id);
            p_assert_eq!(parent, wksp1_id);
            p_assert_eq!(created, now);
            p_assert_eq!(updated, now);
            p_assert_eq!(base_version, 0);
            p_assert_eq!(is_placeholder, true);
            p_assert_eq!(need_sync, true);
            p_assert_eq!(last_updater, alice.device_id);

            true
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready", with_server)]
async fn stat_entry_on_confined_entry(
    #[values(true, false)] local_cache: bool,
    #[values(
        "find_confinement_point",
        "ignore_confinement_point",
        "known_confinement_point"
    )]
    kind: &'_ str,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");

    if !local_cache {
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
    }

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    // Confined from a non temporary folder
    let parent_id = wksp1_foo_id;
    let child_id = ops
        .create_file("/foo/bar.tmp".parse().unwrap())
        .await
        .unwrap();

    let info = ops
        .stat_entry(&"/foo/bar.tmp".parse().unwrap())
        .await
        .unwrap();
    p_assert_matches!(
        info,
        EntryStat::File{
            confinement_point,
            id,
            parent,
            base_version,
            is_placeholder,
            need_sync,
            ..
        }
        if {
            p_assert_eq!(confinement_point, Some(wksp1_foo_id));
            p_assert_eq!(id, child_id);
            p_assert_eq!(parent, parent_id);
            p_assert_eq!(base_version, 0);
            p_assert_eq!(is_placeholder, true);
            p_assert_eq!(need_sync, true);
            true
        }
    );

    let (info, expected_confinement_point) =
        stat_entry_by_id_helper(&ops, child_id, kind, Some(wksp1_foo_id)).await;
    p_assert_matches!(
        info,
        EntryStat::File{
            confinement_point,
            id,
            parent,
            base_version,
            is_placeholder,
            need_sync,
            ..
        }
        if {
            p_assert_eq!(confinement_point, expected_confinement_point);
            p_assert_eq!(id, child_id);
            p_assert_eq!(parent, parent_id);
            p_assert_eq!(base_version, 0);
            p_assert_eq!(is_placeholder, true);
            p_assert_eq!(need_sync, true);
            true
        }
    );

    // Confined from a temporary folder in root
    let parent_id = ops
        .create_folder("/foo.tmp".parse().unwrap())
        .await
        .unwrap();
    let child_id = ops
        .create_file("/foo.tmp/bar.tmp".parse().unwrap())
        .await
        .unwrap();

    let info = ops
        .stat_entry(&"/foo.tmp/bar.tmp".parse().unwrap())
        .await
        .unwrap();
    p_assert_matches!(
        info,
        EntryStat::File{
            confinement_point,
            id,
            parent,
            base_version,
            is_placeholder,
            need_sync,
            ..
        }
        if {
            p_assert_eq!(confinement_point, Some(wksp1_id));
            p_assert_eq!(id, child_id);
            p_assert_eq!(parent, parent_id);
            p_assert_eq!(base_version, 0);
            p_assert_eq!(is_placeholder, true);
            p_assert_eq!(need_sync, true);
            true
        }
    );

    let (info, expected_confinement_point) =
        stat_entry_by_id_helper(&ops, child_id, kind, Some(wksp1_id)).await;
    p_assert_matches!(
        info,
        EntryStat::File{
            confinement_point,
            id,
            parent,
            base_version,
            is_placeholder,
            need_sync,
            ..
        }
        if {
            p_assert_eq!(confinement_point, expected_confinement_point);
            p_assert_eq!(id, child_id);
            p_assert_eq!(parent, parent_id);
            p_assert_eq!(base_version, 0);
            p_assert_eq!(is_placeholder, true);
            p_assert_eq!(need_sync, true);
            true
        }
    );

    // Confined from a temporary folder in a non-root folder
    let parent_id = ops
        .create_folder("/foo/foo.tmp".parse().unwrap())
        .await
        .unwrap();
    let child_id = ops
        .create_file("/foo/foo.tmp/bar.tmp".parse().unwrap())
        .await
        .unwrap();

    let info = ops
        .stat_entry(&"/foo/foo.tmp/bar.tmp".parse().unwrap())
        .await
        .unwrap();
    p_assert_matches!(
        info,
        EntryStat::File{
            confinement_point,
            id,
            parent,
            base_version,
            is_placeholder,
            need_sync,
            ..
        }
        if {
            p_assert_eq!(confinement_point, Some(wksp1_foo_id));
            p_assert_eq!(id, child_id);
            p_assert_eq!(parent, parent_id);
            p_assert_eq!(base_version, 0);
            p_assert_eq!(is_placeholder, true);
            p_assert_eq!(need_sync, true);
            true
        }
    );

    let (info, expected_confinement_point) =
        stat_entry_by_id_helper(&ops, child_id, kind, Some(wksp1_foo_id)).await;
    p_assert_matches!(
        info,
        EntryStat::File{
            confinement_point,
            id,
            parent,
            base_version,
            is_placeholder,
            need_sync,
            ..
        }
        if {
            p_assert_eq!(confinement_point, expected_confinement_point);
            p_assert_eq!(id, child_id);
            p_assert_eq!(parent, parent_id);
            p_assert_eq!(base_version, 0);
            p_assert_eq!(is_placeholder, true);
            p_assert_eq!(need_sync, true);
            true
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn stat_entry_on_under_modification_file(
    #[values("truncate_on_open", "append_data", "truncate_data")] kind: &str,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");

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

    let stat = ops.stat_entry(&path).await.unwrap();

    let expected_stat = EntryStat::File {
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
    };

    p_assert_eq!(stat, expected_stat);

    let stat = ops.stat_entry_by_id(wksp1_bar_txt_id).await.unwrap();
    p_assert_eq!(stat, expected_stat);
}

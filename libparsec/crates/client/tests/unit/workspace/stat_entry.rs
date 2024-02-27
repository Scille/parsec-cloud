// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::workspace_ops_factory;
use crate::workspace::EntryStat;

#[parsec_test(testbed = "minimal_client_ready", with_server)]
async fn stat_entry(#[values(true, false)] local_cache: bool, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_foo_spam_id: VlobID = *env.template.get_stuff("wksp1_foo_spam_id");
    let wksp1_foo_egg_txt_id: VlobID = *env.template.get_stuff("wksp1_foo_egg_txt_id");

    if !local_cache {
        env.customize(|builder| {
            builder.filter_client_storage_events(|event| match event {
                TestbedEvent::WorkspaceDataStorageFetchFileVlob(_)
                | TestbedEvent::WorkspaceDataStorageFetchFolderVlob(_)
                | TestbedEvent::WorkspaceCacheStorageFetchBlock(_) => false,
                // Missing workspace manifest is replaced by a speculative one (so no
                // server fetch will occur), that's not what we want here !
                TestbedEvent::WorkspaceDataStorageFetchWorkspaceVlob(_) => true,
                _ => true,
            });
        });
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
            created,
            updated,
            base_version,
            is_placeholder,
            need_sync,
            children,
        }
        if {
            p_assert_eq!(confinement_point, None);
            p_assert_eq!(id, wksp1_id);
            p_assert_eq!(created, "2000-01-11T00:00:00Z".parse().unwrap());
            p_assert_eq!(updated, "2000-01-11T00:00:00Z".parse().unwrap());
            p_assert_eq!(base_version, 1);
            p_assert_eq!(is_placeholder, false);
            p_assert_eq!(need_sync, false);
            p_assert_eq!(children, [("bar.txt".parse().unwrap(), wksp1_bar_txt_id), ("foo".parse().unwrap(), wksp1_foo_id)]);
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
            created,
            updated,
            base_version,
            is_placeholder,
            need_sync,
            children,
        }
        if {
            p_assert_eq!(confinement_point, None);
            p_assert_eq!(id, wksp1_foo_id);
            p_assert_eq!(created, "2000-01-10T00:00:00Z".parse().unwrap());
            p_assert_eq!(updated, "2000-01-10T00:00:00Z".parse().unwrap());
            p_assert_eq!(base_version, 1);
            p_assert_eq!(is_placeholder, false);
            p_assert_eq!(need_sync, false);
            p_assert_eq!(children, [
                ("egg.txt".parse().unwrap(), wksp1_foo_egg_txt_id),
                ("spam".parse().unwrap(), wksp1_foo_spam_id),
            ]);
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
            created,
            updated,
            base_version,
            is_placeholder,
            need_sync,
            size,
        }
        if {
            p_assert_eq!(confinement_point, None);
            p_assert_eq!(id, wksp1_bar_txt_id);
            p_assert_eq!(created, "2000-01-07T00:00:00Z".parse().unwrap());
            p_assert_eq!(updated, "2000-01-07T00:00:00Z".parse().unwrap());
            p_assert_eq!(base_version, 1);
            p_assert_eq!(is_placeholder, false);
            p_assert_eq!(need_sync, false);
            p_assert_eq!(size, 11);  // Contains "hello world"
            true
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn stat_entry_on_speculative_workspace(env: &TestbedEnv) {
    let wksp1_id: &VlobID = env.template.get_stuff("wksp1_id");

    env.customize(|builder| {
        builder.filter_client_storage_events(|event| match event {
            // Don't remove the children manifests from storage: they should be
            // ignored given the speculative workspace manifest doesn't mention them.
            TestbedEvent::WorkspaceDataStorageFetchWorkspaceVlob(_) => false,
            _ => true,
        });
    });

    let alice = env.local_device("alice@dev1");

    let now: DateTime = "2000-12-31T00:00:00Z".parse().unwrap();
    alice.time_provider.mock_time_frozen(now);

    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let info = ops.stat_entry(&"/".parse().unwrap()).await.unwrap();

    p_assert_matches!(
        info,
        EntryStat::Folder{
            confinement_point,
            id,
            created,
            updated,
            base_version,
            is_placeholder,
            need_sync,
            children,
        } if {
            p_assert_eq!(confinement_point, None);
            p_assert_eq!(id, *wksp1_id);
            p_assert_eq!(created, now);
            p_assert_eq!(updated, now);
            p_assert_eq!(base_version, 0);
            p_assert_eq!(is_placeholder, true);
            p_assert_eq!(need_sync, true);
            p_assert_eq!(children, []);
            true
        }
    );
}

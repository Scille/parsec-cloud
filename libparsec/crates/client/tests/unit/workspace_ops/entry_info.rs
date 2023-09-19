// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::workspace_ops_factory;
use crate::workspace_ops::EntryInfo;

#[parsec_test(testbed = "minimal_client_ready", with_server)]
#[case::with_local_cache(true)]
#[case::without_local_cache(false)]
async fn entry_info(#[case] local_cache: bool, env: &TestbedEnv) {
    let wksp1_id: &VlobID = env.template.get_stuff("wksp1_id");
    let wksp1_key: &SecretKey = env.template.get_stuff("wksp1_key");
    let wksp1_foo_id: &VlobID = env.template.get_stuff("wksp1_foo_id");
    let wksp1_bar_txt_id: &VlobID = env.template.get_stuff("wksp1_bar_txt_id");

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
    let ops = workspace_ops_factory(
        &env.discriminant_dir,
        &alice,
        wksp1_id.to_owned(),
        wksp1_key.to_owned(),
    )
    .await;

    // Workspace

    let info = ops.entry_info(&"/".parse().unwrap()).await.unwrap();
    p_assert_matches!(
        info,
        EntryInfo::Folder{
            confinement_point,
            id,
            created,
            updated,
            base_version,
            is_placeholder,
            need_sync,
            children,
        }
        if confinement_point.is_none() &&
        id == *wksp1_id &&
        created == "2000-01-07T00:00:00Z".parse().unwrap() &&
        updated == "2000-01-07T00:00:00Z".parse().unwrap() &&
        base_version == 1 &&
        !is_placeholder &&
        !need_sync &&
        children == ["bar.txt".parse().unwrap(), "foo".parse().unwrap()]
    );

    // Folder

    let info = ops.entry_info(&"/foo".parse().unwrap()).await.unwrap();
    p_assert_matches!(
        info,
        EntryInfo::Folder{
            confinement_point,
            id,
            created,
            updated,
            base_version,
            is_placeholder,
            need_sync,
            children,
        }
        if confinement_point.is_none() &&
        id == *wksp1_foo_id &&
        created == "2000-01-04T00:00:00Z".parse().unwrap() &&
        updated == "2000-01-04T00:00:00Z".parse().unwrap() &&
        base_version == 1 &&
        !is_placeholder &&
        !need_sync &&
        children.is_empty()
    );

    // File

    let info = ops.entry_info(&"/bar.txt".parse().unwrap()).await.unwrap();
    p_assert_matches!(
        info,
        EntryInfo::File{
            confinement_point,
            id,
            created,
            updated,
            base_version,
            is_placeholder,
            need_sync,
            size,
        }
        if confinement_point.is_none() &&
        id == *wksp1_bar_txt_id &&
        created == "2000-01-06T00:00:00Z".parse().unwrap() &&
        updated == "2000-01-06T00:00:00Z".parse().unwrap() &&
        base_version == 1 &&
        !is_placeholder &&
        !need_sync &&
        size == 11  // Contains "hello world"
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn entry_info_on_speculative_workspace(env: &TestbedEnv) {
    let wksp1_id: &VlobID = env.template.get_stuff("wksp1_id");
    let wksp1_key: &SecretKey = env.template.get_stuff("wksp1_key");

    env.customize(|builder| {
        builder.filter_client_storage_events(|event| match event {
            // Don't remove the other manifest from storage: they should be
            // ignored given the workspace don't mention them
            TestbedEvent::WorkspaceDataStorageFetchWorkspaceVlob(_) => false,
            _ => true,
        });
    });

    let alice = env.local_device("alice@dev1");

    let now: DateTime = "2000-12-31T00:00:00Z".parse().unwrap();
    alice.time_provider.mock_time(MockedTime::FrozenTime(now));

    let ops = workspace_ops_factory(
        &env.discriminant_dir,
        &alice,
        wksp1_id.to_owned(),
        wksp1_key.to_owned(),
    )
    .await;

    let info = ops.entry_info(&"/".parse().unwrap()).await.unwrap();
    p_assert_matches!(
        info,
        EntryInfo::Folder{
            confinement_point,
            id,
            created,
            updated,
            base_version,
            is_placeholder,
            need_sync,
            children,
        }
        if confinement_point.is_none() &&
        id == *wksp1_id &&
        created == now &&
        updated == now &&
        base_version == 0 &&
        is_placeholder &&
        need_sync &&
        children.is_empty()
    );
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::str::FromStr;

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{
    Broadcastable, ClientGetOutboundSyncBacklog, EventWorkspaceOutboundSyncBacklog,
    client::tests::utils::client_factory_with_monitors, workspace::OpenOptions,
};

use super::utils::client_factory;

#[parsec_test(testbed = "minimal_client_ready")]
async fn includes_started_workspaces_only(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let client = client_factory(&env.discriminant_dir, alice.clone()).await;

    let started_workspace = client.start_workspace(wksp1_id).await.unwrap();
    let fd = started_workspace
        .open_file_by_id(wksp1_bar_txt_id, OpenOptions::read_write())
        .await
        .unwrap();
    started_workspace
        .fd_write(fd, 0, b"client-backlog-test")
        .await
        .unwrap();
    started_workspace.fd_flush(fd).await.unwrap();
    started_workspace.fd_close(fd).await.unwrap();

    let not_started_workspace_id = client
        .create_workspace("not-started".parse().unwrap())
        .await
        .unwrap();

    let backlog = client.get_outbound_sync_backlog().await.unwrap();

    p_assert_eq!(backlog.per_workspace.len(), 1);
    p_assert_eq!(backlog.per_workspace[0].realm_id, wksp1_id);
    p_assert_eq!(backlog.per_workspace[0].pending_entries, 1);
    assert!(backlog.per_workspace[0].pending_bytes > 0);
    p_assert_eq!(
        backlog.total_pending_entries_for_started_workspaces,
        backlog.per_workspace[0].pending_entries
    );
    p_assert_eq!(
        backlog.total_pending_bytes_for_started_workspaces,
        backlog.per_workspace[0].pending_bytes
    );
    assert!(
        backlog
            .per_workspace
            .iter()
            .all(|item| item.realm_id != not_started_workspace_id)
    );
}

#[parsec_test(
    testbed = "minimal_client_ready",
    // tokio_worker_threads = 8,
    with_server
)]
async fn not_including_confined_entries(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let client = client_factory_with_monitors(&env.discriminant_dir, alice.clone()).await;

    let started_workspace = client.start_workspace(wksp1_id).await.unwrap();

    let mut spy = client.event_bus.spy.start_expecting();

    started_workspace
        .rename_entry_by_id(
            wksp1_id,
            "bar.txt".try_into().unwrap(),
            "bar.tmp".try_into().unwrap(),
            crate::workspace::MoveEntryMode::NoReplace,
        )
        .await
        .unwrap();

    let fd = started_workspace
        .open_file_by_id(wksp1_bar_txt_id, OpenOptions::read_write())
        .await
        .unwrap();

    started_workspace
        .fd_write(fd, 0, b"client-backlog-test")
        .await
        .unwrap();
    started_workspace.fd_flush(fd).await.unwrap();

    started_workspace.fd_close(fd).await.unwrap();

    libparsec_platform_async::sleep(std::time::Duration::from_secs(5)).await;

    let events = spy.list_not_acknowledged();

    for event in events {
        println!("{event:?}");
        if let Some(e) = EventWorkspaceOutboundSyncBacklog::try_from_any_spied_event(&event)
            && e.number_of_files_to_sync == 0
            && e.size_to_sync == 0
        {
            spy.ack_remaining_events();
            return;
        };
    }
    panic!("there is still something to sync")
}

const INPUT: &[u8] = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, \
        sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim \
        ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip \
        ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate \
        velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat \
        cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id \
        est laborum.";

#[parsec_test(testbed = "minimal_client_ready", with_server)]

async fn rename_tmp_files(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let client = client_factory_with_monitors(&env.discriminant_dir, alice.clone()).await;

    let started_workspace = client.start_workspace(wksp1_id).await.unwrap();

    let mut spy = client.event_bus.spy.start_expecting();

    // write confined file

    let file = started_workspace
        .open_file(
            FsPath::from_str("/file.tmp").unwrap(),
            OpenOptions {
                read: true,
                write: true,
                truncate: false,
                create: true,
                create_new: false,
            },
        )
        .await
        .unwrap();
    started_workspace.fd_write(file, 0, INPUT).await.unwrap();
    started_workspace.fd_flush(file).await.unwrap();

    started_workspace.fd_close(file).await.unwrap();

    libparsec_platform_async::sleep(std::time::Duration::from_secs(3)).await;
    // move confined to not confined

    started_workspace
        .rename_entry_by_id(
            wksp1_id,
            "file.tmp".to_string().try_into().unwrap(),
            "file.txt".to_string().try_into().unwrap(),
            crate::workspace::MoveEntryMode::NoReplace,
        )
        .await
        .unwrap();

    // let it sync
    libparsec_platform_async::sleep(std::time::Duration::from_secs(1)).await;

    let events = spy.list_not_acknowledged();

    let mut file_synced = false;
    let mut started_with_nothing_to_sync = false;
    let mut ended_with_nothing_to_sync = false;

    // need to check that first there is nothing to sync then a file with size 445, then nothing again
    // ignoring all other events
    for event in events {
        if let Some(e) = EventWorkspaceOutboundSyncBacklog::try_from_any_spied_event(&event) {
            if e.number_of_files_to_sync == 0 && e.size_to_sync == 0 {
                if !file_synced {
                    started_with_nothing_to_sync = true
                } else {
                    ended_with_nothing_to_sync = true
                }
            } else {
                if e.number_of_files_to_sync == 1 && e.size_to_sync == 445 {
                    file_synced = true
                }
            }
        };
    }

    spy.ack_remaining_events();
    assert!(file_synced && started_with_nothing_to_sync && ended_with_nothing_to_sync)
}

fn is_backlog_empty(backlog: ClientGetOutboundSyncBacklog) -> bool {
    let mut res = true;
    for e in backlog.per_workspace {
        res = res && e.pending_bytes == 0 && e.pending_entries == 0
    }

    res && backlog.total_pending_entries_for_started_workspaces == 0
        && backlog.total_pending_bytes_for_started_workspaces == 0
}

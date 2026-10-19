// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::str::FromStr;

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{
    Broadcastable, ClientGetOutboundSyncBacklog, EventWorkspaceOutboundSyncBacklog,
    client::tests::utils::client_factory_with_monitors, workspace::OpenOptions,
};

#[parsec_test(testbed = "minimal_client_ready", with_server)]
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

    libparsec_platform_async::sleep(std::time::Duration::from_secs(1)).await;

    started_workspace
        .fd_write(fd, 0, b"client-backlog-test")
        .await
        .unwrap();
    started_workspace.fd_flush(fd).await.unwrap();

    started_workspace.fd_close(fd).await.unwrap();

    libparsec_platform_async::sleep(std::time::Duration::from_secs(1)).await;

    let events = spy.list_not_acknowledged();

    for event in events {
        println!("{event:?}");
        if EventWorkspaceOutboundSyncBacklog::try_from_any_spied_event(&event).is_some() {
            panic!("the file, is confined, no backlog expected")
        };
    }
    spy.ack_remaining_events();
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
    // let it start with nothing to sync
    // libparsec_platform_async::sleep(std::time::Duration::from_secs(1)).await;

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

    libparsec_platform_async::sleep(std::time::Duration::from_secs(1)).await;

    // let it sync

    // check no backlog event is sent
    let events = spy.list_not_acknowledged();
    for event in events {
        if EventWorkspaceOutboundSyncBacklog::try_from_any_spied_event(&event).is_some() {
            panic!("there should be no backlog")
        };
    }
    spy.ack_remaining_events();

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

    // write more stuff so there is a better chance to catch a backlog event
    let file = started_workspace
        .open_file(
            FsPath::from_str("/file.txt").unwrap(),
            OpenOptions {
                read: true,
                write: true,
                truncate: false,
                create: false,
                create_new: false,
            },
        )
        .await
        .unwrap();
    started_workspace.fd_write(file, 0, INPUT).await.unwrap();
    started_workspace.fd_flush(file).await.unwrap();
    libparsec_platform_async::sleep(std::time::Duration::from_secs(1)).await;

    let events = spy.list_not_acknowledged();

    let mut file_synced = false;

    // need to check that first there is nothing to sync then a file with size 445, then nothing again
    // ignoring all other events
    for event in events {
        if let Some(e) = EventWorkspaceOutboundSyncBacklog::try_from_any_spied_event(&event) {
            if e.number_of_files_to_sync == 1 && e.size_to_sync == 445 {
                file_synced = true
            } else if e.number_of_files_to_sync == 0 && e.size_to_sync == 0 && file_synced {
                break;
            }
        };
    }

    spy.ack_remaining_events();
    assert!(file_synced);
}

#[parsec_test(testbed = "minimal_client_ready", with_server)]
async fn many_files(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let client = client_factory_with_monitors(&env.discriminant_dir, alice.clone()).await;

    let started_workspace = client.start_workspace(wksp1_id).await.unwrap();

    let mut spy = client.event_bus.spy.start_expecting();

    for i in 0..5 {
        let file = started_workspace
            .open_file(
                FsPath::from_str(&format!("/file_{i}.txt")).unwrap(),
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
    }

    libparsec_platform_async::sleep(std::time::Duration::from_secs(1)).await;

    let events = spy.list_not_acknowledged();

    let mut file_synced = false;
    for event in events {
        println!("{event:?}");
        if let Some(e) = EventWorkspaceOutboundSyncBacklog::try_from_any_spied_event(&event) {
            if e.number_of_files_to_sync != 0 && e.size_to_sync != 0 {
                file_synced = true;
            } else if e.number_of_files_to_sync == 0 && e.size_to_sync == 0 && file_synced {
                break;
            } else {
                panic!("blocks without files or files without blocks to sync")
            }
        };
    }
    spy.ack_remaining_events();
}

fn is_backlog_empty(backlog: ClientGetOutboundSyncBacklog) -> bool {
    let mut res = true;
    for e in backlog.per_workspace {
        res = res && e.pending_bytes == 0 && e.pending_entries == 0
    }

    res && backlog.total_pending_entries_for_started_workspaces == 0
        && backlog.total_pending_bytes_for_started_workspaces == 0
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::str::FromStr;

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{
    EventBusSpyExpectContext, EventWorkspaceOutboundSyncBacklog,
    client::tests::utils::client_factory_with_monitors, workspace::OpenOptions,
};

const INPUT: &[u8] = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, \
        sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim \
        ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip \
        ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate \
        velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat \
        cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id \
        est laborum.";

#[parsec_test(testbed = "minimal_client_ready", with_server)]
async fn not_including_confined_entries(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let client = client_factory_with_monitors(&env.discriminant_dir, alice.clone()).await;

    let started_workspace = client.start_workspace(wksp1_id).await.unwrap();

    let mut spy = client.event_bus.spy.start_expecting();

    // Rename the file for it to be confined
    started_workspace
        .rename_entry_by_id(
            wksp1_id,
            "bar.txt".try_into().unwrap(),
            "bar.tmp".try_into().unwrap(),
            crate::workspace::MoveEntryMode::NoReplace,
        )
        .await
        .unwrap();

    assert_0_0_backlog(&mut spy).await;
}

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

    // check no backlog event is sent
    assert_0_0_backlog(&mut spy).await;

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

    assert_file_synced(&mut spy).await;
}

#[parsec_test(testbed = "minimal_client_ready", with_server)]
async fn many_files(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let client = client_factory_with_monitors(&env.discriminant_dir, alice.clone()).await;

    let started_workspace = client.start_workspace(wksp1_id).await.unwrap();
    let mut spy = client.event_bus.spy.start_expecting();

    for i in 0..10 {
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

    assert_file_synced(&mut spy).await;
}

async fn assert_file_synced(spy: &mut EventBusSpyExpectContext) {
    let mut file_syncing = false;
    // should be done before the end of that loop
    // it is huge because we expect
    for _ in 0..100 {
        let event = spy
            .wait_filter_next::<EventWorkspaceOutboundSyncBacklog>()
            .await;
        if let Some(e) = event {
            println!("{e:?}");
            // at least a file is syncing
            if e.size_to_sync != 0 && e.number_of_files_to_sync != 0 {
                file_syncing = true
            } else if file_syncing && e.number_of_files_to_sync == 0 && e.size_to_sync == 0 {
                spy.ack_remaining_events();
                return;
            }
        }
    }
    spy.ack_remaining_events();
    panic!("did not return to empty backlog")
}

async fn assert_0_0_backlog(spy: &mut EventBusSpyExpectContext) {
    // loop to let breathing room for other events
    for _ in 0..10 {
        let event = spy
            .wait_filter_next::<EventWorkspaceOutboundSyncBacklog>()
            .await;
        if let Some(e) = event {
            // a (0,0) event is expected after the rename
            // because the manifest has been updated
            if e.size_to_sync == 0 && e.number_of_files_to_sync == 0 {
                spy.ack_remaining_events();
                return;
            }
        }
    }
    spy.ack_remaining_events();
    panic!("did not receive (0,0) backlog event after workspace update")
}

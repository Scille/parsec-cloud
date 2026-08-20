// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{workspace::OpenOptions, ClientGetOutboundSyncBacklog};

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
    assert!(backlog
        .per_workspace
        .iter()
        .all(|item| item.realm_id != not_started_workspace_id));
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn not_including_confined_entries(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let client = client_factory(&env.discriminant_dir, alice.clone()).await;

    let started_workspace = client.start_workspace(wksp1_id).await.unwrap();
    started_workspace.outbound_sync(wksp1_id).await.unwrap();

    let backlog = client.get_outbound_sync_backlog().await.unwrap();
    println!(" {backlog:?}");

    assert!(is_backlog_empty(backlog));
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

    let backlog = client.get_outbound_sync_backlog().await.unwrap();
    assert!(is_backlog_empty(backlog));
}

fn is_backlog_empty(backlog: ClientGetOutboundSyncBacklog) -> bool {
    let mut res = true;
    for e in backlog.per_workspace {
        res = res && e.pending_bytes == 0 && e.pending_entries == 0
    }

    res && backlog.total_pending_entries_for_started_workspaces == 0
        && backlog.total_pending_bytes_for_started_workspaces == 0
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use libparsec_client::{user_ops::UserOpsError, RunningDevice};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

#[parsec_test(testbed = "minimal")]
async fn list_create_rename(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1".parse().unwrap());
    let running = RunningDevice::start(alice, &env.discriminant_dir)
        .await
        .unwrap();

    p_assert_eq!(running.user_ops.list_workspaces(), vec![]);

    let wid1 = running
        .user_ops
        .workspace_create("wksp1".parse().unwrap())
        .await
        .unwrap();

    p_assert_eq!(
        running.user_ops.list_workspaces(),
        vec![(wid1, "wksp1".parse().unwrap()),]
    );

    running
        .user_ops
        .workspace_rename(&wid1, "wksp1'".parse().unwrap())
        .await
        .unwrap();

    p_assert_eq!(
        running.user_ops.list_workspaces(),
        vec![(wid1, "wksp1'".parse().unwrap()),]
    );

    let wid2 = running
        .user_ops
        .workspace_create("wksp2".parse().unwrap())
        .await
        .unwrap();

    p_assert_eq!(
        running.user_ops.list_workspaces(),
        vec![
            (wid1, "wksp1'".parse().unwrap()),
            (wid2, "wksp2".parse().unwrap()),
        ]
    );

    // running.user_ops.sync().await.unwrap();

    running.stop().await;
}

#[parsec_test(testbed = "minimal")]
async fn rename_unknown_id(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1".parse().unwrap());
    let running = RunningDevice::start(alice, &env.discriminant_dir)
        .await
        .unwrap();

    let bad_wid = EntryID::default();

    let outcome = running
        .user_ops
        .workspace_rename(&bad_wid, "wksp1'".parse().unwrap())
        .await;

    p_assert_matches!(
        outcome,
        Err(UserOpsError::UnknownWorkspace(wid))
        if wid == bad_wid
    );

    running.stop().await;
}

// #[parsec_test(testbed = "coolorg", with_server)]
// async fn share(env: &TestbedEnv) {
//     let alice = env.local_device("alice@dev1".parse().unwrap());
//     let bob = env.local_device("bob@dev1".parse().unwrap());
//     let running = RunningDevice::start(alice, &env.discriminant_dir)
//         .await
//         .unwrap();
//     // TODO: `poll_server_for_new_certificates` should be needed here given it is
//     // automatically done by `certificates_monitor` when switching online
//     running
//         .certificates_ops
//         .poll_server_for_new_certificates(None)
//         .await
//         .unwrap();

//     let wid = running
//         .user_ops
//         .workspace_create("wksp1".parse().unwrap())
//         .await
//         .unwrap();

//     running
//         .user_ops
//         .workspace_rename(&wid, "wksp1'".parse().unwrap())
//         .await
//         .unwrap();

//     running
//         .user_ops
//         .workspace_share(&wid, bob.device_id.user_id(), Some(RealmRole::Contributor))
//         // .workspace_share(&wid, &"bob".parse().unwrap(), Some(RealmRole::Contributor))
//         .await
//         .unwrap();

//     // running.user_ops.sync().await.unwrap();

//     running.stop().await;
// }

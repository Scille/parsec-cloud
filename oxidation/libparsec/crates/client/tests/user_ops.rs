// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use libparsec_client::RunningDevice;
use libparsec_client_connection::{protocol, test_register_send_hook};
use libparsec_tests_fixtures::prelude::*;
// use libparsec_types::RealmRole;

#[parsec_test(testbed = "minimal")]
async fn base(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1".parse().unwrap());
    let running = RunningDevice::start(alice, &env.discriminant_dir)
        .await
        .unwrap();

    test_register_send_hook(
        &env.discriminant_dir,
        |_req: protocol::authenticated_cmds::latest::realm_create::Req| async {
            protocol::authenticated_cmds::latest::realm_create::Rep::Ok
        },
    );

    let wid = running
        .user_ops
        .workspace_create("wksp1".parse().unwrap())
        .await
        .unwrap();

    running
        .user_ops
        .workspace_rename(&wid, "wksp1'".parse().unwrap())
        .await
        .unwrap();

    // running.user_ops.sync().await.unwrap();

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

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use libparsec_client::RunningDevice;
use libparsec_client_connection::{protocol, test_register_send_hook};
use libparsec_tests_fixtures::prelude::*;

#[parsec_test(testbed = "minimal")]
async fn base(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let running = RunningDevice::start(alice, &env.discriminant_dir)
        .await
        .unwrap();

    test_register_send_hook(
        &env.discriminant_dir,
        |_req: protocol::authenticated_cmds::latest::realm_create::Req| {
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
        .workspace_rename(wid, "wksp1'".parse().unwrap())
        .await
        .unwrap();

    // let bob = env.local_device("bob@dev1".parse().unwrap());
    // running.user_ops.workspace_share(&wid, &bob, Some(RealmRole::Reader), None).await.unwrap();

    // running.user_ops.sync().await.unwrap();

    running.stop().await;
}

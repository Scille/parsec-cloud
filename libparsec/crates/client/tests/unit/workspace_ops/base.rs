// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::workspace_ops_factory;

#[parsec_test(testbed = "minimal_client_ready")]
async fn basic_info(env: &TestbedEnv) {
    let wksp1_id: &VlobID = env.template.get_stuff("wksp1_id");
    let wksp1_key: &SecretKey = env.template.get_stuff("wksp1_key");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(
        &env.discriminant_dir,
        &alice,
        wksp1_id.to_owned(),
        wksp1_key.to_owned(),
    )
    .await;

    p_assert_eq!(ops.realm_id(), *wksp1_id);
}

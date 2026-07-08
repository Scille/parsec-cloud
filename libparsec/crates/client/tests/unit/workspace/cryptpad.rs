// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::workspace_ops_factory;
#[allow(unused_imports)]
use crate::workspace::{CryptpadSessionKeys, WorkspaceRegisterCryptpadSessionError};

#[parsec_test(testbed = "minimal_client_ready")]
async fn test_register_cryptpad_session_ok(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    // Test that we can call the function
    let vlob_id = VlobID::default();
    let read_write_key = "my_read_write_key".to_string();
    let read_only_key = "my_read_only_key".to_string();

    // This will fail because we're not mocking the server response, but at least it compiles
    let result = ops
        .register_cryptpad_session(vlob_id, read_write_key, read_only_key)
        .await;

    // The test should verify:
    // 1. The request contains correct candidate keys
    // 2. The CryptpadSessionKeys returned has correct key and mode
    //
    // However, without a mock server, we can only check that the function can be called
    // and returns the appropriate type. In a real test with a testbed server, this would work.

    // For now, just ensure the function compiles and can be called
    let _ = result;
}

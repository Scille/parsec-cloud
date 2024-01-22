// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{workspace::WorkspaceFsOperationError, ClientStartWorkspaceError};

use super::utils::client_factory;

#[parsec_test(testbed = "minimal_client_ready")]
async fn ok(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    macro_rules! assert_is_started {
        ($expected_is_started:expr) => {
            async {
                let workspaces = client.list_workspaces().await;
                p_assert_eq!(workspaces.len(), 1, "{:?}", workspaces);
                p_assert_eq!(workspaces[0].is_started, $expected_is_started);
            }
        };
    }

    assert_is_started!(false).await;

    let workspace = client.start_workspace(wksp1_id).await.unwrap();

    assert_is_started!(true).await;
    p_assert_eq!(workspace.realm_id(), wksp1_id);
    p_assert_matches!(workspace.create_file(&"/1".parse().unwrap()).await, Ok(_));

    client.stop_workspace(wksp1_id).await;

    assert_is_started!(false).await;
    // TODO: uncomment once workspace storage is updated to deal with closed database
    // p_assert_matches!(workspace.create_file(&"/2".parse().unwrap()).await, Err(WorkspaceFsOperationError::Stopped));
}

#[parsec_test(testbed = "minimal")]
async fn unknown(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let dummy_id = VlobID::default();
    let err = client.start_workspace(dummy_id).await.unwrap_err();
    p_assert_matches!(err, ClientStartWorkspaceError::WorkspaceNotFound);

    // Stop doesn't care if the workspace exists as long as it is not running
    client.stop_workspace(dummy_id).await;
}

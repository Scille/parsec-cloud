// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::user_ops_factory;
use crate::user_ops::WorkspaceRenameError;

#[parsec_test(testbed = "minimal")]
async fn list_create_rename(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let user_ops = user_ops_factory(env, &alice).await;

    p_assert_eq!(user_ops.list_workspaces(), vec![]);

    let wid1 = user_ops
        .workspace_create("wksp1".parse().unwrap())
        .await
        .unwrap();

    p_assert_eq!(
        user_ops.list_workspaces(),
        vec![(wid1, "wksp1".parse().unwrap()),]
    );

    user_ops
        .workspace_rename(wid1, "wksp1'".parse().unwrap())
        .await
        .unwrap();

    p_assert_eq!(
        user_ops.list_workspaces(),
        vec![(wid1, "wksp1'".parse().unwrap()),]
    );

    let wid2 = user_ops
        .workspace_create("wksp2".parse().unwrap())
        .await
        .unwrap();

    p_assert_eq!(
        user_ops.list_workspaces(),
        vec![
            (wid1, "wksp1'".parse().unwrap()),
            (wid2, "wksp2".parse().unwrap()),
        ]
    );

    user_ops.stop().await;
}

#[parsec_test(testbed = "minimal")]
async fn rename_unknown_id(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let user_ops = user_ops_factory(env, &alice).await;

    let bad_wid = VlobID::default();

    let outcome = user_ops
        .workspace_rename(bad_wid, "wksp1'".parse().unwrap())
        .await;

    p_assert_matches!(
        outcome,
        Err(WorkspaceRenameError::UnknownWorkspace(wid))
        if wid == bad_wid
    );

    user_ops.stop().await;
}

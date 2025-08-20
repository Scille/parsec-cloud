// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::{DataAccessStrategy, StartWorkspaceHistoryOpsError};

#[parsec_test(testbed = "workspace_history")]
async fn ok(
    #[values(DataAccessStrategy::Server, DataAccessStrategy::RealmExport)]
    strategy: DataAccessStrategy,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let ops = match strategy.start_workspace_history_ops(env).await {
        Ok(ops) => ops,
        Err(StartWorkspaceHistoryOpsError::RealmExportNotSupportedOnWeb) => return,
    };

    p_assert_eq!(ops.realm_id(), wksp1_id);

    let expected_organization_id = match &strategy {
        DataAccessStrategy::Server => {
            let alice = env.local_device("alice@dev1");
            alice.organization_id().to_owned()
        }
        DataAccessStrategy::RealmExport => {
            // Always the same name since it is written in the realm export database file
            "Org1".parse().unwrap()
        }
    };
    p_assert_eq!(ops.organization_id(), &expected_organization_id);

    let timestamp_lower_bound = ops.timestamp_lower_bound();
    let timestamp_higher_bound = ops.timestamp_higher_bound();

    p_assert_eq!(
        timestamp_lower_bound,
        // Wksp1's workspace manifest was created on 2001-01-02
        "2001-01-02T00:00:00Z".parse().unwrap(),
    );

    assert!(
        timestamp_higher_bound >= "2001-01-30T00:00:00Z".parse().unwrap(),
        "{timestamp_higher_bound:?}"
    );

    p_assert_eq!(ops.timestamp_of_interest(), timestamp_lower_bound,);
}

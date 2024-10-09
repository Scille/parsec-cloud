// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::super::utils::workspace_ops_factory;
use crate::workspace::{
    store::PathConfinementPoint, EntryStat, OpenOptions, WorkspaceHistoryStatEntryError,
    WorkspaceOps,
};

#[parsec_test(testbed = "minimal_client_ready", with_server)]
async fn stat_entry(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let at_before = "1999-01-01T00:00:00Z".parse().unwrap();
    let at_after = "2020-01-01T00:00:00Z".parse().unwrap();

    p_assert_matches!(
        ops.history
            .stat_entry(at_before, &"/".parse().unwrap())
            .await,
        Err(WorkspaceHistoryStatEntryError::EntryNotFound)
    );

    p_assert_matches!(
        ops.history.stat_entry(at_after, &"/".parse().unwrap()).await,
        Ok(EntryStat::Folder{
            confinement_point,
            id,
            parent,
            created,
            updated,
            base_version,
            is_placeholder,
            need_sync,
        })
        if {
            p_assert_eq!(confinement_point, None);
            p_assert_eq!(id, wksp1_id);
            p_assert_eq!(parent, wksp1_id);
            p_assert_eq!(created, "2000-01-11T00:00:00Z".parse().unwrap());
            p_assert_eq!(updated, "2000-01-11T00:00:00Z".parse().unwrap());
            p_assert_eq!(base_version, 1);
            p_assert_eq!(is_placeholder, false);
            p_assert_eq!(need_sync, false);
            true
        }
    );
}

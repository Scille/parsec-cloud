// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::super::utils::workspace_ops_factory;
use crate::workspace::{
    store::PathConfinementPoint, EntryStat, OpenOptions,
    WorkspaceHistoryFolderReaderStatNextOutcome, WorkspaceHistoryOpenFolderReaderError,
    WorkspaceHistoryStatFolderChildrenError, WorkspaceOps,
};

#[parsec_test(testbed = "minimal_client_ready", with_server)]
async fn read_folder(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let at_before = "1999-01-01T00:00:00Z".parse().unwrap();
    let at_after = "2020-01-01T00:00:00Z".parse().unwrap();

    p_assert_matches!(
        ops.history
            .stat_folder_children(at_before, &"/".parse().unwrap())
            .await,
        Err(WorkspaceHistoryStatFolderChildrenError::EntryNotFound)
    );

    p_assert_eq!(
        ops.history
            .stat_folder_children(at_after, &"/".parse().unwrap())
            .await
            .unwrap(),
        vec![
            (
                "bar.txt".parse().unwrap(),
                EntryStat::File {
                    confinement_point: None,
                    id: wksp1_bar_txt_id,
                    parent: wksp1_id,
                    created: "2000-01-11T00:00:00Z".parse().unwrap(),
                    updated: "2000-01-11T00:00:00Z".parse().unwrap(),
                    base_version: 1,
                    size: 11,
                    is_placeholder: false,
                    need_sync: false,
                }
            ),
            (
                "foo".parse().unwrap(),
                EntryStat::Folder {
                    confinement_point: None,
                    id: wksp1_foo_id,
                    parent: wksp1_id,
                    created: "2000-01-11T00:00:00Z".parse().unwrap(),
                    updated: "2000-01-11T00:00:00Z".parse().unwrap(),
                    base_version: 1,
                    is_placeholder: false,
                    need_sync: false,
                }
            )
        ]
    );
}

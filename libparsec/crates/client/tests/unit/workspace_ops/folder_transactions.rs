// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::workspace_ops_factory;
use crate::workspace_ops::EntryStat;

#[parsec_test(testbed = "minimal_client_ready")]
#[case::root_level(true)]
#[case::subdir_level(false)]
async fn good(#[case] root_level: bool, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_key: &SecretKey = env.template.get_stuff("wksp1_key");

    // Remove any sub file/folder from the place we are going to test from
    env.customize(|builder| {
        if root_level {
            builder
                .create_or_update_workspace_manifest_vlob("alice@dev1", wksp1_id)
                .customize(|e| {
                    let manifest = Arc::make_mut(&mut e.manifest);
                    manifest.children.clear();
                });
            builder.workspace_data_storage_fetch_workspace_vlob("alice@dev1", wksp1_id, None);
        } else {
            builder
                .create_or_update_folder_manifest_vlob("alice@dev1", wksp1_id, wksp1_foo_id)
                .customize(|e| {
                    let manifest = Arc::make_mut(&mut e.manifest);
                    manifest.children.clear();
                });
            builder.workspace_data_storage_fetch_folder_vlob(
                "alice@dev1",
                wksp1_id,
                wksp1_foo_id,
                None,
            );
        }
    });

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(
        &env.discriminant_dir,
        &alice,
        wksp1_id.to_owned(),
        wksp1_key.to_owned(),
    )
    .await;

    let base_path: FsPath = if root_level {
        "/".parse().unwrap()
    } else {
        "/foo".parse().unwrap()
    };

    macro_rules! ls {
        ($path:expr) => {
            async {
                let info = ops.stat_entry(&$path).await.unwrap();
                match info {
                    EntryStat::Folder { children, .. } => children,
                    x => panic!("Bad info type: {:?}", x),
                }
            }
        };
    }

    // Now let's add folders !

    let dir1 = base_path.join("dir1".parse().unwrap());
    let dir2 = base_path.join("dir2".parse().unwrap());
    let dir3 = base_path.join("dir3".parse().unwrap());

    ops.create_folder(&dir1).await.unwrap();
    ops.create_folder(&dir2).await.unwrap();
    ops.create_folder(&dir3).await.unwrap();
    // Add subdirs to know which dir is which once we start renaming them
    ops.create_folder(&dir1.join("subdir1".parse().unwrap()))
        .await
        .unwrap();
    ops.create_folder(&dir3.join("subdir3".parse().unwrap()))
        .await
        .unwrap();

    p_assert_eq!(
        ls!(&base_path).await,
        [
            "dir1".parse().unwrap(),
            "dir2".parse().unwrap(),
            "dir3".parse().unwrap()
        ]
    );
    p_assert_eq!(ls!(&dir1).await, ["subdir1".parse().unwrap()]);
    p_assert_eq!(ls!(&dir2).await, []);
    p_assert_eq!(ls!(&dir3).await, ["subdir3".parse().unwrap()]);

    // Remove folder

    ops.remove_folder(&dir2).await.unwrap();

    p_assert_eq!(
        ls!(&base_path).await,
        ["dir1".parse().unwrap(), "dir3".parse().unwrap()]
    );
    p_assert_eq!(ls!(&dir1).await, ["subdir1".parse().unwrap()]);
    p_assert_eq!(ls!(&dir3).await, ["subdir3".parse().unwrap()]);

    // Rename folder

    ops.rename_entry(&dir1, "dir2".parse().unwrap(), false)
        .await
        .unwrap();

    p_assert_eq!(
        ls!(&base_path).await,
        ["dir2".parse().unwrap(), "dir3".parse().unwrap()]
    );
    p_assert_eq!(ls!(&dir2).await, ["subdir1".parse().unwrap()]);
    p_assert_eq!(ls!(&dir3).await, ["subdir3".parse().unwrap()]);

    // Overwrite by rename folder

    ops.rename_entry(&dir3, "dir2".parse().unwrap(), true)
        .await
        .unwrap();

    p_assert_eq!(ls!(&base_path).await, ["dir2".parse().unwrap()]);
    p_assert_eq!(ls!(&dir2).await, ["subdir3".parse().unwrap()]);
}

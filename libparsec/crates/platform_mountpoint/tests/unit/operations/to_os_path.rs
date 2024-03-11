// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::start_client_with_mountpoint_base_dir;
use crate::Mountpoint;

#[parsec_test(testbed = "minimal_client_ready")]
async fn ok(tmp_path: TmpPath, env: &TestbedEnv) {
    let parsec_path = "/foo/egg.txt".parse().unwrap();
    let expected_os_path = tmp_path.join("wksp1/foo/egg.txt");
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let client =
        start_client_with_mountpoint_base_dir(env, (*tmp_path).to_owned(), "alice@dev1").await;
    let wksp1_ops = client.start_workspace(wksp1_id).await.unwrap();

    {
        let mountpoint = Mountpoint::mount(wksp1_ops.clone()).await.unwrap();

        let os_path = mountpoint.to_os_path(&parsec_path);
        p_assert_eq!(os_path, expected_os_path);

        assert!(tokio::fs::try_exists(os_path).await.unwrap());

        mountpoint.unmount().await.unwrap();
    }
    client.stop().await;
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn invalid_windows_character(tmp_path: TmpPath, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let env = &env.customize(|builder| {
        let wksp1_foo_id = *builder.get_stuff("wksp1_foo_id");
        // Rename /foo -> /?
        builder
            .workspace_data_storage_local_workspace_manifest_update("alice@dev1", wksp1_id)
            .customize(|x| {
                let local_manifest = Arc::make_mut(&mut x.local_manifest);
                let foo_id = local_manifest
                    .children
                    .remove(&"foo".parse().unwrap())
                    .unwrap();
                assert_eq!(foo_id, wksp1_foo_id); // Sanity check
                local_manifest.children.insert("?".parse().unwrap(), foo_id);
            });
        // Rename /foo/egg.txt -> /?/AUX.h
        builder
            .workspace_data_storage_local_folder_manifest_create_or_update(
                "alice@dev1",
                wksp1_id,
                wksp1_foo_id,
            )
            .customize(|x| {
                let local_manifest = Arc::make_mut(&mut x.local_manifest);
                let egg_id = local_manifest
                    .children
                    .remove(&"egg.txt".parse().unwrap())
                    .unwrap();
                local_manifest
                    .children
                    .insert("AUX.h".parse().unwrap(), egg_id);
            });
    });

    let parsec_path = "/?/AUX.h".parse().unwrap();
    #[cfg(target_os = "windows")]
    let expected_os_path = tmp_path.join("wksp1\\~3f\\AU~58.h");
    #[cfg(not(target_os = "windows"))]
    let expected_os_path = tmp_path.join("wksp1/?/AUX.h");

    let client =
        start_client_with_mountpoint_base_dir(env, (*tmp_path).to_owned(), "alice@dev1").await;
    let wksp1_ops = client.start_workspace(wksp1_id).await.unwrap();

    {
        let mountpoint = Mountpoint::mount(wksp1_ops.clone()).await.unwrap();

        let os_path = mountpoint.to_os_path(&parsec_path);
        p_assert_eq!(os_path, expected_os_path);

        assert!(tokio::fs::try_exists(os_path).await.unwrap());

        mountpoint.unmount().await.unwrap();
    }
    client.stop().await;
}

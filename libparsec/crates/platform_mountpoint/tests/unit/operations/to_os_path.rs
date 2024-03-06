// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{tests::utils::start_client_with_mountpoint_base_dir, Mountpoint};

#[parsec_test(testbed = "minimal_client_ready")]
async fn ok(tmp_path: TmpPath, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let client =
        start_client_with_mountpoint_base_dir(env, (*tmp_path).to_owned(), "alice@dev1").await;
    let wksp1_ops = client.start_workspace(wksp1_id).await.unwrap();

    {
        let mountpoint = Mountpoint::mount(wksp1_ops.clone()).await.unwrap();

        let foo_egg_txt_os_path = mountpoint.to_os_path(&"/foo/egg.txt".parse().unwrap());
        p_assert_eq!(foo_egg_txt_os_path, tmp_path.join("wksp1/foo/egg.txt"));

        mountpoint.unmount().await.unwrap();
    }
    client.stop().await;
}

// TODO: test with path containing invalid Windows characters

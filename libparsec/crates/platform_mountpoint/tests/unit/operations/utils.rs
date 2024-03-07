// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client::{Client, ClientConfig, EventBus, ProxyConfig, WorkspaceStorageCacheSize};
use libparsec_tests_fixtures::prelude::*;

pub async fn start_client(env: &TestbedEnv, start_as: &'static str) -> Arc<Client> {
    start_client_with_mountpoint_base_dir(env, env.discriminant_dir.clone(), start_as).await
}

pub async fn start_client_with_mountpoint_base_dir(
    env: &TestbedEnv,
    mountpoint_base_dir: std::path::PathBuf,
    start_as: &'static str,
) -> Arc<Client> {
    let event_bus = EventBus::default();
    let config = Arc::new(ClientConfig {
        config_dir: env.discriminant_dir.clone(),
        data_base_dir: env.discriminant_dir.clone(),
        mountpoint_base_dir,
        workspace_storage_cache_size: WorkspaceStorageCacheSize::Default,
        proxy: ProxyConfig::default(),
        with_monitors: false,
    });
    let device = env.local_device(start_as);
    Client::start(config, event_bus, device).await.unwrap()
}

macro_rules! mount_and_test {
    ($env:expr, $mountpoint_base_dir:expr, $test:expr) => {
        mount_and_test!(as "alice@dev1", $env, $mountpoint_base_dir, $test);
    };
    (as $start_as:literal, $env:expr, $mountpoint_base_dir:expr, $test:expr) => {{
        // Ensure we don't take ownership on TmpPath fixture, otherwise it drop will
        // kicks in too early and will shadow the real error.
        let mountpoint_base_dir: &TmpPath = $mountpoint_base_dir;
        let mountpoint_base_dir: PathBuf = (*mountpoint_base_dir).to_owned();

        let wksp1_id: VlobID = *$env.template.get_stuff("wksp1_id");
        let client = $crate::tests::utils::start_client_with_mountpoint_base_dir($env, mountpoint_base_dir, $start_as).await;
        let wksp1_ops = client.start_workspace(wksp1_id).await.unwrap();
        let test_result = {
            let mountpoint = $crate::Mountpoint::mount(wksp1_ops.clone())
                .await
                .unwrap();

            let test_closure = $test;
            let test_future = test_closure(client.clone(), wksp1_ops, mountpoint.path().to_owned());
            let test_result = test_future.await;

            mountpoint.unmount().await.unwrap();

            test_result
        };
        client.stop().await;

        // The idea for returning the test result is to delay the tests asserts
        // until after the mountpoint is unmounted. This way a panic won't make
        // the mountpoint hang.
        test_result
    }};
}

pub(crate) use mount_and_test;

macro_rules! ops_cat {
    ($wksp_ops:expr, $path:expr) => {{
        let wksp_ops = $wksp_ops;
        let path = $path.parse::<FsPath>().unwrap();
        async move {
            let options = libparsec_client::workspace::OpenOptions {
                read: true,
                write: false,
                truncate: false,
                create: false,
                create_new: false,
            };
            let fd = wksp_ops.open_file(path, options).await.unwrap();
            let mut buff = Vec::new();
            wksp_ops.fd_read(fd, 0, u64::MAX, &mut buff).await.unwrap();
            buff
        }
    }};
}

pub(crate) use ops_cat;

macro_rules! os_ls {
    ($mountpoint_path:expr) => {{
        let mountpoint_path = $mountpoint_path;
        async move {
            let mut children = vec![];
            let mut readdir = tokio::fs::read_dir(mountpoint_path).await.unwrap();
            while let Some(child) = readdir.next_entry().await.unwrap() {
                children.push(child.file_name().to_str().unwrap().to_owned());
            }
            children
        }
    }};
}

pub(crate) use os_ls;

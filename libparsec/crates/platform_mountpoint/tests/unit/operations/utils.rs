// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client::{
    Client, ClientConfig, EventBus, MountpointMountStrategy, ProxyConfig, WorkspaceStorageCacheSize,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::PreventSyncPattern;

#[cfg_attr(target_os = "windows", allow(dead_code))]
pub async fn start_client(env: &TestbedEnv, start_as: &'static str) -> Arc<Client> {
    start_client_with_mountpoint_base_dir(env, env.discriminant_dir.clone(), start_as).await
}

#[cfg(target_os = "windows")]
pub fn to_null_terminated_utf16(s: &str) -> Vec<u16> {
    let mut utf16 = s.encode_utf16().collect::<Vec<u16>>();
    utf16.push(0);
    utf16
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
        mountpoint_mount_strategy: MountpointMountStrategy::Directory {
            base_dir: mountpoint_base_dir,
        },
        workspace_storage_cache_size: WorkspaceStorageCacheSize::Default,
        proxy: ProxyConfig::default(),
        with_monitors: false,
        prevent_sync_pattern: PreventSyncPattern::empty(),
    });
    let device = env.local_device(start_as);
    Client::start(config, event_bus, device).await.unwrap()
}

/// Mount workspace "wksp1" from TestbedEnv `env` in `mountpoint_base_dir` and
/// execute the given `test` closure.
///
/// If specified, `start_as` is the client used to mount "wksp1". Otherwise
/// "alice@dev1" will be used.
///
/// # Examples
///
/// Mount wksp1 as "alice@dev1":
///
/// ```
/// mount_and_test!(env, &tmp_path,
///     |client: Arc<Client>, wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {{
///     // ...
///     });
/// ```
///
/// Mount wksp1 as "bob@dev1":
/// ```
/// mount_and_test!(as "bob@dev1", env, &tmp_path,
///     |bob_client: Arc<Client>, bob_wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {{
///     // ...
///     });
/// ```
macro_rules! mount_and_test {
    ($env:expr, $mountpoint_base_dir:expr, $test:expr) => {
        mount_and_test!(as "alice@dev1", $env, $mountpoint_base_dir, $test)
    };
    (as $start_as:literal, $env:expr, $mountpoint_base_dir:expr, $test:expr) => {{
        // Ensure we don't take ownership on TmpPath fixture, otherwise its drop will
        // kick in too early and will shadow the real error.
        let mountpoint_base_dir: &libparsec_tests_fixtures::TmpPath = $mountpoint_base_dir;
        let mountpoint_base_dir: std::path::PathBuf = (*mountpoint_base_dir).to_owned();

        let wksp1_id: libparsec_types::VlobID = *$env.template.get_stuff("wksp1_id");
        let client = $crate::operations::utils::start_client_with_mountpoint_base_dir($env, mountpoint_base_dir, $start_as).await;
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

macro_rules! mount_history_and_test {
    ($env:expr, $mountpoint_base_dir:expr, $test:expr) => {
        mount_history_and_test!(_internal, as "alice@dev1", $env, $mountpoint_base_dir, $test)
    };
    (at $at:expr, $env:expr, $mountpoint_base_dir:expr, $test:expr) => {
        mount_history_and_test!(_internal, as "alice@dev1", at Some($at), $env, $mountpoint_base_dir, $test)
    };
    (as $as:literal, $env:expr, $mountpoint_base_dir:expr, $test:expr) => {
        mount_history_and_test!(_internal, as $as, at None, $env, $mountpoint_base_dir, $test)
    };
    (as $as:literal, at $at:expr, $env:expr, $mountpoint_base_dir:expr, $test:expr) => {
        mount_history_and_test!(_internal, as $as, at Some($at), at None, $env, $mountpoint_base_dir, $test)
    };
    (_internal, as $start_as:literal, at $at:expr, $env:expr, $mountpoint_base_dir:expr, $test:expr) => {{
        let env = $env;

        // Ensure we don't take ownership on TmpPath fixture, otherwise its drop will
        // kick in too early and will shadow the real error.
        let mountpoint_base_dir: &libparsec_tests_fixtures::TmpPath = $mountpoint_base_dir;
        let mountpoint_base_dir: std::path::PathBuf = (*mountpoint_base_dir).to_owned();

        let wksp1_id: libparsec_types::VlobID = *env.template.get_stuff("wksp1_id");
        let client = $crate::operations::utils::start_client_with_mountpoint_base_dir(env, mountpoint_base_dir, $start_as).await;

        // In server-based mode, `WorkspaceHistoryOps` starts by querying the server to
        // fetch the workspace manifest v1.
        use libparsec_client_connection::{
            test_register_sequence_of_send_hooks,
            test_send_hook_vlob_read_versions,
            test_send_hook_realm_get_keys_bundle,
        };
        test_register_sequence_of_send_hooks!(
            env.discriminant_dir,
            test_send_hook_vlob_read_versions!(env, wksp1_id, (wksp1_id, 1)),
            test_send_hook_realm_get_keys_bundle!(env, client.user_id(), wksp1_id),
        );

        let wksp1_history_ops = client.start_workspace_history(wksp1_id).await.unwrap();

        if let Some(at) = $at {
            wksp1_history_ops.set_timestamp_of_interest(at);
        }

        let test_result = {
            let mountpoint = $crate::Mountpoint::mount_history(wksp1_history_ops.clone(), "wksp1_history".parse().unwrap())
                .await
                .unwrap();

            let test_closure = $test;
            let test_future = test_closure(client.clone(), wksp1_history_ops, mountpoint.path().to_owned());
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

pub(crate) use mount_history_and_test;

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

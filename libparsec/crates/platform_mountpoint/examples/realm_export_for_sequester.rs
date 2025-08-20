// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(clippy::unwrap_used)]
#![cfg(not(target_family = "wasm"))]

use std::{
    path::{Path, PathBuf},
    process::ExitCode,
    sync::Arc,
};

use libparsec_client::{
    ClientConfig, MountpointMountStrategy, ProxyConfig, WorkspaceHistoryOps,
    WorkspaceHistoryRealmExportDecryptor, WorkspaceStorageCacheSize,
};
use libparsec_platform_mountpoint::Mountpoint;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

#[tokio::main]
async fn main() -> ExitCode {
    let print_usage = || {
        println!(
            "Usage: {} <mountpoint_path> [<timestamp as RFC3339>]",
            std::env::args_os().next().unwrap().to_string_lossy()
        );
    };

    // Configure logging with WARNING level by default (customize with `RUST_LOG` environ variable)

    let mut args = std::env::args_os();
    let _ = args.next().unwrap(); // Ignore path of the executable

    // First argument is the desired mountpoint path
    let mountpoint_mount_strategy = match args.next() {
        Some(raw_path) => {
            let base_dir = PathBuf::from(raw_path);

            let mut components = base_dir.components();
            match (components.next(), components.next()) {
                (Some(std::path::Component::Prefix(prefix)), None)
                    if matches!(prefix.kind(), std::path::Prefix::Disk(_)) =>
                {
                    MountpointMountStrategy::DriveLetter
                }
                _ => MountpointMountStrategy::Directory { base_dir },
            }
        }
        None => {
            print_usage();
            return ExitCode::from(1);
        }
    };

    // Second argument (if any) is the desired timestamp of interest
    let timestamp_of_interest = match args.next() {
        Some(raw_timestamp) => {
            let timestamp = raw_timestamp
                .to_str()
                .and_then(|timestamp| DateTime::from_rfc3339(timestamp).ok());
            match timestamp {
                Some(timestamp) => timestamp,
                None => {
                    eprintln!("Invalid timestamp format");
                    print_usage();
                    return ExitCode::from(1);
                }
            }
        }
        // By default, provide the moment sequester service 1 got revoked.
        // This is a good timestamp since any change after that moment cannot be decrypted
        // since a key rotation occurs before the new sequester service 2 is created (
        // so both sequester service 1 and 2 have no access to the new key).
        // (see `libparsec/crates/testbed/src/templates/sequestered.rs` documentation)
        None => "2000-01-16T00:00:00Z".parse().unwrap(),
    };

    // Too many arguments ?
    if args.next().is_some() {
        print_usage();
        return ExitCode::from(1);
    }

    env_logger::init();

    // Retrieve the path of the realm export dump

    let exe_path = std::env::current_exe().unwrap();
    let mut path: &Path = exe_path.as_ref();
    let original_export_db_path = loop {
        if path.ends_with("target") {
            break path
                .join("../server/tests/realm_export/sequestered_export.sqlite")
                .canonicalize()
                .unwrap();
        }
        match path.parent() {
            Some(parent) => path = parent,
            None => {
                eprintln!("Cannot find the realm export dump");
                return ExitCode::from(1);
            }
        }
    };

    // Copy the realm export database since SQLite modifies the file in place
    // even when only doing read operations.
    struct RemoveDirOnDrop<'a>(&'a Path);
    impl Drop for RemoveDirOnDrop<'_> {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(self.0);
        }
    }
    let temp_dir = std::env::temp_dir().join(format!(
        "parsec-mountpoint-example-realm_export_for_sequester-{}",
        uuid::Uuid::new_v4()
    ));
    std::fs::create_dir_all(&temp_dir).unwrap();
    let _remove_temp_dir_on_drop = RemoveDirOnDrop(&temp_dir);
    let export_db_path = temp_dir.join(original_export_db_path.file_name().unwrap());
    std::fs::copy(original_export_db_path, &export_db_path).unwrap();

    libparsec_tests_fixtures::TestbedScope::run("sequestered", |env: Arc<TestbedEnv>| async move {
        let env = env.as_ref();

        // We use `sequestered` testbed template, which contains two sequesters services
        // `sequester_service_1` & `sequester_service_2` with a key rotation on `wksp1`
        // done after `sequester_service_1` got revoked.
        let decryptors = env
            .template
            .events
            .iter()
            .filter_map(|e| match e {
                TestbedEvent::NewSequesterService(x) => {
                    Some(WorkspaceHistoryRealmExportDecryptor::SequesterService {
                        sequester_service_id: x.id,
                        private_key: Box::new(x.encryption_private_key.clone()),
                    })
                }
                _ => None,
            })
            .collect();

        let config = Arc::new(ClientConfig {
            config_dir: env.discriminant_dir.clone(),
            data_base_dir: env.discriminant_dir.clone(),
            mountpoint_mount_strategy,
            workspace_storage_cache_size: WorkspaceStorageCacheSize::Default,
            proxy: ProxyConfig::default(),
            with_monitors: false,
            prevent_sync_pattern: PreventSyncPattern::empty(),
        });

        let wksp1_history_ops = Arc::new(
            WorkspaceHistoryOps::start_with_realm_export(config, &export_db_path, decryptors)
                .await
                .unwrap(),
        );

        wksp1_history_ops
            .set_timestamp_of_interest(timestamp_of_interest)
            .await
            .unwrap();

        {
            let mountpoint_name_hint: EntryName = {
                // Format snapshot as `yyyymmddThhmmssZ` format (shorter, and Windows doesn't allow `:` in paths)
                let ts = {
                    let ts = timestamp_of_interest.to_rfc3339().replace(['-', ':'], "");
                    match ts.split_once('.') {
                        Some((dt, _)) => format!("{}Z", dt),
                        None => ts.to_owned(),
                    }
                };
                format!("wksp1-{}", ts)
            }
            .parse()
            .unwrap();
            let mountpoint = Mountpoint::mount_history(wksp1_history_ops, mountpoint_name_hint)
                .await
                .unwrap();

            println!("Mounted workspace at {:?}", mountpoint.path());

            let (tx, mut rx) = tokio::sync::mpsc::channel(1);
            ctrlc::set_handler(move || {
                let _ = tx.try_send(());
            })
            .unwrap();
            rx.recv().await.unwrap();

            println!("Bye ;-)");
            mountpoint.unmount().await.unwrap();
        };
    })
    .await;

    ExitCode::SUCCESS
}

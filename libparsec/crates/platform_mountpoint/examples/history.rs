// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(clippy::unwrap_used)]

use std::{path::PathBuf, process::ExitCode, sync::Arc};

use libparsec_client::{
    Client, ClientConfig, EventBus, MountpointMountStrategy, ProxyConfig, WorkspaceStorageCacheSize,
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
    env_logger::init();

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

    // Second argument (if an) is the desired timestamp of interest
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
        // By default, provide a timestamp where part of the workspace changes hasn't been done yet
        // (see `libparsec/crates/testbed/src/templates/workspace_history.rs` documentation)
        None => "2001-01-06T00:00:00Z".parse().unwrap(),
    };

    // Too many arguments ?
    if args.next().is_some() {
        print_usage();
        return ExitCode::from(1);
    }

    libparsec_tests_fixtures::TestbedScope::run_with_server(
        "workspace_history",
        |env: Arc<TestbedEnv>| async move {
            let env = env.as_ref();
            let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
            let alice = env.local_device("alice@dev1");

            let event_bus = EventBus::default();
            let config = Arc::new(ClientConfig {
                config_dir: env.discriminant_dir.clone(),
                data_base_dir: env.discriminant_dir.clone(),
                mountpoint_mount_strategy,
                workspace_storage_cache_size: WorkspaceStorageCacheSize::Default,
                proxy: ProxyConfig::default(),
                with_monitors: false,
                prevent_sync_pattern: PreventSyncPattern::empty(),
            });
            let client = Client::start(config, event_bus, alice).await.unwrap();
            let wksp1_history_ops = client.start_workspace_history(wksp1_id).await.unwrap();

            let timestamp_of_interest =
                wksp1_history_ops.set_timestamp_of_interest(timestamp_of_interest);

            {
                let mountpoint_name_hint = format!("wksp1-{}", timestamp_of_interest.to_rfc3339())
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
        },
    )
    .await;

    ExitCode::SUCCESS
}

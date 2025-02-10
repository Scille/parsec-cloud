use std::{path::PathBuf, sync::Arc};

use libparsec::{DateTime, EntryName, SequesterPrivateKeyDer, SequesterServiceID};
use libparsec_client::{WorkspaceHistoryOps, WorkspaceHistoryRealmExportDecryptor};
use libparsec_platform_mountpoint::Mountpoint;

use crate::utils::{default_client_config, load_and_unlock_device};

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, password_stdin]
    pub struct Args {
        /// Path to the realm export database
        export_db_path: PathBuf,
        /// Sequester service or user to use for decryption.
        /// Allowed format `device:<device_id>` or `sequester:<service_id>:<path/to/private_key.pem>`.
        #[arg(short, long)]
        decryptor: Vec<Decryptor>,
        /// Browse the realm at a specific point in time.
        #[arg(short, long)]
        timestamp: Option<DateTime>,
    }
);

#[derive(Debug, Clone)]
enum Decryptor {
    User {
        raw_id: String,
    },
    SequesterService {
        sequester_service_id: SequesterServiceID,
        private_key_pem_path: PathBuf,
    },
}

impl std::str::FromStr for Decryptor {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const ERROR_MSG: &str = "Invalid decryptor format, expected `device:<device_id>` or `sequester:<service_id>:<path/to/private_key.pem>`";
        match s.split_once(':') {
            Some(("device", remain)) => Ok(Decryptor::User {
                raw_id: remain.to_string(),
            }),
            Some(("sequester", remain)) => {
                let (service_id, private_key) = remain.split_once(':').ok_or(ERROR_MSG)?;
                Ok(Decryptor::SequesterService {
                    sequester_service_id: SequesterServiceID::from_hex(service_id)
                        .map_err(|_| ERROR_MSG)?,
                    private_key_pem_path: PathBuf::from(private_key),
                })
            }
            _ => Err(ERROR_MSG),
        }
    }
}

pub async fn main(args: Args) -> anyhow::Result<()> {
    let Args {
        config_dir,
        password_stdin,
        export_db_path,
        decryptor: raw_decryptors,
        timestamp,
    } = args;

    let config = Arc::new({
        let mut config = default_client_config();
        config.mountpoint_mount_strategy = libparsec_client::MountpointMountStrategy::Directory {
            base_dir: std::env::current_dir()?,
        };
        config
    });

    let mut decryptors = Vec::with_capacity(raw_decryptors.len());
    for raw_decryptor in raw_decryptors {
        match raw_decryptor {
            Decryptor::User { raw_id } => {
                let device =
                    load_and_unlock_device(&config_dir, Some(raw_id), password_stdin).await?;
                decryptors.push(WorkspaceHistoryRealmExportDecryptor::User {
                    user_id: device.user_id,
                    private_key: Box::new(device.private_key.to_owned()),
                });
            }

            Decryptor::SequesterService {
                sequester_service_id,
                private_key_pem_path,
            } => {
                let raw = tokio::fs::read_to_string(private_key_pem_path).await?;
                let key = SequesterPrivateKeyDer::load_pem(&raw)?;
                decryptors.push(WorkspaceHistoryRealmExportDecryptor::SequesterService {
                    sequester_service_id,
                    private_key: Box::new(key),
                });
            }
        }
    }

    let wksp_history_ops = Arc::new(
        WorkspaceHistoryOps::start_with_realm_export(config, &export_db_path, decryptors).await?,
    );

    println!("Organization: {}", wksp_history_ops.organization_id());
    println!("Realm ID: {}", wksp_history_ops.realm_id());
    println!(
        "Export temporal bounds: {} to {}",
        wksp_history_ops.timestamp_lower_bound(),
        wksp_history_ops.timestamp_higher_bound()
    );

    let timestamp = timestamp.unwrap_or(wksp_history_ops.timestamp_higher_bound());
    wksp_history_ops
        .set_timestamp_of_interest(timestamp)
        .await?;

    let mountpoint_name_hint: EntryName = export_db_path
        .file_stem()
        .and_then(|stem| stem.to_string_lossy().parse().ok())
        .unwrap_or("realm_export".parse().expect("valid entry name"));

    let mountpoint = Mountpoint::mount_history(wksp_history_ops, mountpoint_name_hint).await?;
    println!("Mounted at {:?}", mountpoint.path());

    let (tx, mut rx) = tokio::sync::mpsc::channel(1);
    ctrlc::set_handler(move || {
        let _ = tx.try_send(());
    })
    .expect("Failed to set Ctrl-C handler");
    rx.recv().await.expect("Ctrl-C handler failed");

    println!("Bye ;-)");
    mountpoint.unmount().await?;

    Ok(())
}

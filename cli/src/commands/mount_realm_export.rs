use std::{fs::File, io::Read, path::PathBuf, sync::Arc};

use dialoguer::Confirm;
use libparsec::{DateTime, SequesterPrivateKeyDer, SequesterServiceID};
use libparsec_client::{WorkspaceHistoryOps, WorkspaceHistoryRealmExportDecryptor};
use libparsec_platform_mountpoint::Mountpoint;

use crate::utils::default_client_config;

crate::clap_parser_with_shared_opts_builder!(
    #[with =  device, password_stdin]
    pub struct Args {
        /// Path to the DB
        export_db_path: PathBuf,
        /// sequester private key
        decryptor_sequester_private_key_path: PathBuf,
        /// workspace name
        workspace: String,
        /// sequester id
        sequester_service_id: String,
        // /// target timestamp
       // timestamp_of_interest: DateTime,
    }
);

pub async fn main(args: Args) -> anyhow::Result<()> {
    let Args {
        export_db_path,
        decryptor_sequester_private_key_path,
        //timestamp_of_interest,
        workspace,
        sequester_service_id,
        ..
    } = args;

    let config = Arc::new(default_client_config());
    let mut file = File::open(decryptor_sequester_private_key_path)?;
    let mut decryptor_sequester_private_key = Vec::<u8>::new();
    file.read_to_end(&mut decryptor_sequester_private_key)?;

    let decryptors = vec![WorkspaceHistoryRealmExportDecryptor::SequesterService {
        sequester_service_id: SequesterServiceID::from_hex(&sequester_service_id)?,
        private_key: Box::new(SequesterPrivateKeyDer::try_from(
            &decryptor_sequester_private_key[..],
        )?),
    }];

    let wksp_history_ops = Arc::new(
        WorkspaceHistoryOps::start_with_realm_export(config, &export_db_path, decryptors).await?,
    );

    let mountpoint_name_hint = format!("{workspace}-{}", DateTime::now()).parse()?;
    let mountpoint = Mountpoint::mount_history(wksp_history_ops, mountpoint_name_hint).await?;

    println!("Mounted workspace at {:?}", mountpoint.path());

    while !Confirm::new()
        .with_prompt("Close mountpoint ?")
        .interact()?
    {}

    mountpoint.unmount().await?;
    println!("Unmounted workspace.");

    Ok(())
}

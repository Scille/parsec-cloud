use anyhow::Context;
use libparsec_platform_mountpoint::Mountpoint;

use crate::utils::StartedClient;
use std::path::PathBuf;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin, workspace]
    pub struct Args {
        /// Path where to mount the workspace
        #[arg(value_hint = clap::ValueHint::DirPath)]
        mount_dir: PathBuf,
    }
);

crate::build_main_with_client!(
    main,
    mount_workspace,
    libparsec::ClientConfig {
        with_monitors: true, // Enable monitor to keep workspace's content updated
        ..Default::default()
    }
    .into()
);

pub async fn mount_workspace(
    _todo_ui: crate::Ui,
    args: Args,
    client: &StartedClient,
) -> anyhow::Result<()> {
    log::trace!("Mounting workspace");
    let Args {
        workspace,
        mount_dir,
        ..
    } = args;
    log::debug!(
        "Starting workspace {workspace} with device {}",
        client.device_id()
    );
    let wksp = client
        .start_workspace(workspace)
        .await
        .context("Failed to start workspace")?;

    let mountpoint = Mountpoint::mount_at_path(wksp, mount_dir)
        .await
        .context("Failed to mount workspace at specified directory")?;

    println!("Workspace mounted, send SIGINT signal (Ctrl-C) to stop");

    let (tx, mut rx) = tokio::sync::mpsc::channel(1);
    ctrlc::set_handler(move || {
        let _ = tx.try_send(());
    })
    .expect("Failed to set Ctrl-C handler");
    rx.recv().await.expect("Ctrl-C handler failed");

    println!("Signal received, stopping...");
    let mut unmount_option = libparsec_platform_mountpoint::UnmountOptions::default();
    unmount_option.remove_dir = false;
    mountpoint
        .unmount_with_options(unmount_option)
        .await
        .context("Failed to unmount workspace")?;

    client.stop_workspace(workspace).await;
    Ok(())
}

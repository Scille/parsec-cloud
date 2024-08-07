use std::{path::PathBuf, vec};

use libparsec::{anyhow::Context, FsPath, OpenOptions, VlobID};
use tokio::io::AsyncReadExt;

use crate::utils::load_client;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct WorkspaceImport {
        /// Workspace id
        #[arg(short, long, value_parser = VlobID::from_hex)]
        workspace_id: VlobID,
        /// Local file to copy
        src: PathBuf,
        /// Workspace destination path
        dest: FsPath,
    }
);

pub async fn workspace_import(args: WorkspaceImport) -> anyhow::Result<()> {
    let WorkspaceImport {
        src,
        dest,
        workspace_id,
        password_stdin,
        device,
        config_dir,
    } = args;

    log::trace!(
        "workspace_import: {src} -> {workspace_id}:{dst}",
        src = src.display(),
        dst = dest
    );

    let client = load_client(&config_dir, device, password_stdin).await?;
    let workspace = client.start_workspace(workspace_id).await?;
    let fd = workspace
        .open_file(
            dest,
            OpenOptions {
                read: false,
                write: true,
                truncate: true,
                create: true,
                create_new: false,
            },
        )
        .await?;

    let file = tokio::fs::File::open(&src)
        .await
        .context("Cannot open local file")?;
    let mut buf_file = tokio::io::BufReader::new(file);
    let mut buffer = vec![0_u8; 4096];
    let mut dst_offset = 0_usize;

    log::debug!("Copying file to workspace");
    loop {
        let bytes_read = buf_file
            .read(&mut buffer)
            .await
            .context("Cannot read local file")?;

        if bytes_read == 0 {
            break;
        }

        let mut bytes_written = 0_usize;
        while bytes_written < bytes_read {
            bytes_written = workspace
                .fd_write(fd, dst_offset as u64, &buffer[bytes_written..bytes_read])
                .await
                .context("Cannot write to workspace")? as usize;
            dst_offset += bytes_written;
        }
    }

    log::debug!("Flushing and closing file");
    workspace.fd_flush(fd).await?;
    workspace.fd_close(fd).await?;

    loop {
        let entries_to_sync = workspace.get_need_outbound_sync(20).await?;
        log::debug!("Entries to outbound sync: {:?}", entries_to_sync);
        if entries_to_sync.is_empty() {
            break;
        }
        for entry in entries_to_sync {
            workspace.outbound_sync(entry).await?;
        }
    }
    Ok(())
}

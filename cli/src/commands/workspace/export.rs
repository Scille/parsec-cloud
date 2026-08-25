use std::path::PathBuf;

use libparsec::{FsPath, OpenOptions};
use tokio::io::AsyncWriteExt;

use crate::utils::StartedClient;

const CHUNK_SIZE: usize = 4096;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin, workspace]
    pub struct Args {
        /// File to export (e.g. "myfile.txt")
        #[arg(value_hint = clap::ValueHint::FilePath)]
        pub(crate) src: FsPath,
        /// Destination path (e.g. "/path/to/myfile.txt")
        ///
        /// The command will fail if the
        /// parent directories do not exist, unless the `parents` option is enabled.
        ///
        /// If the destination file already exists, its content will be replaced.
        #[arg(value_hint = clap::ValueHint::DirPath)]
        pub(crate) dest: PathBuf,
        /// If specified, create parent directories as needed
        ///
        /// No error if parent directories already exist (similar to `mkdir -p`)
        #[clap(long, short, action)]
        pub(crate) parents: bool,
        /// Control how existing files are updated.
        ///
        /// (similar to `cp --update=...`)
        #[clap(long, value_enum, default_value_t)]
        update: UpdateMode,
    }
);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
enum UpdateMode {
    /// Existing files in destination are replaced
    All,
    /// Existing files in destination are not replaced but raise an error instead.
    #[default]
    NoneFail,
}

crate::build_main_with_client!(
    main,
    workspace_export,
    libparsec::ClientConfig {
        with_monitors: true,
        ..Default::default()
    }
    .into()
);

pub async fn workspace_export(
    _ui: crate::Ui,
    args: Args,
    client: &StartedClient,
) -> anyhow::Result<()> {
    let Args {
        src,
        dest,
        workspace: wid,
        update,
        parents,
        ..
    } = args;

    log::trace!(
        "workspace_export: {wid}:{src} -> {dst}",
        src = src,
        dst = dest.display()
    );

    let workspace = client.start_workspace(wid).await?;

    let file = workspace
        .open_file(src.clone(), OpenOptions::read_only())
        .await
        .map_err(|e| match e {
            libparsec::WorkspaceOpenFileError::EntryNotFound => {
                anyhow::anyhow!("File {src} not found in Parsec workspace.")
            }
            libparsec::WorkspaceOpenFileError::EntryNotAFile { .. } => {
                anyhow::anyhow!("{src} is not a file.")
            }
            _ => e.into(),
        })?;
    let stats = workspace.fd_stat(file).await?;
    let size = stats.size;

    if (update == UpdateMode::NoneFail) && tokio::fs::try_exists(&dest).await? {
        return Err(anyhow::anyhow!("File already exists."));
    }

    if parents {
        if let Some(p) = dest.parent() {
            tokio::fs::create_dir_all(p).await?
        }
    }

    let dest_file = tokio::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&dest)
        .await?;

    let mut read_buf = Vec::with_capacity(CHUNK_SIZE);
    let mut write_buf = tokio::io::BufWriter::new(dest_file);

    let mut offset = 0;
    while offset < size {
        // read and decrypt chunk
        let bytes_read = workspace
            .fd_read(file, offset, CHUNK_SIZE as u64, &mut read_buf)
            .await?;

        // write to dst file
        write_buf
            .write_all(&read_buf[..bytes_read.try_into()?])
            .await?;
        offset += bytes_read
    }
    write_buf.flush().await?;

    Ok(())
}

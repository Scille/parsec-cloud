use libparsec::FsPath;

use crate::utils::load_client;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin, workspace]
    pub struct Rm {
        /// Path to remove
        path: FsPath,
    }
);

pub async fn rm(args: Rm) -> anyhow::Result<()> {
    let Rm {
        workspace,
        path,
        password_stdin,
        device,
        config_dir,
    } = args;

    log::trace!("rm: {workspace}:{path}", workspace = workspace, path = path);

    let client = load_client(&config_dir, device, password_stdin).await?;
    let workspace = client.start_workspace(workspace).await?;

    workspace.remove_entry(path).await?;

    Ok(())
}

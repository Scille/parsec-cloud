use libparsec::FsPath;

use crate::utils::StartedClient;

crate::clap_parser_with_shared_opts_builder!(
    #[with = client_opts, workspace]
    pub struct Args {
        /// Path to remove
        path: FsPath,
    }
);

crate::build_main_with_client!(main, rm);

pub async fn rm(args: Args, client: &StartedClient) -> anyhow::Result<()> {
    let Args {
        workspace, path, ..
    } = args;

    log::trace!("rm: {workspace}:{path}");

    let workspace = client.start_workspace(workspace).await?;

    workspace.remove_entry(path).await?;

    Ok(())
}

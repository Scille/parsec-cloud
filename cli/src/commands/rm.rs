use libparsec::{FsPath, VlobID};

use crate::utils::load_client;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Rm {
        /// Workspace id
        #[arg(short, long, value_parser = VlobID::from_hex)]
        workspace_id: VlobID,
        /// Path to remove
        path: FsPath,
    }
);

pub async fn rm(args: Rm) -> anyhow::Result<()> {
    let Rm {
        workspace_id,
        path,
        password_stdin,
        device,
        config_dir,
    } = args;

    log::trace!(
        "rm: {workspace_id}:{path}",
        workspace_id = workspace_id,
        path = path
    );

    let client = load_client(&config_dir, device, password_stdin).await?;
    let workspace = client.start_workspace(workspace_id).await?;

    workspace.remove_entry(path).await?;

    Ok(())
}

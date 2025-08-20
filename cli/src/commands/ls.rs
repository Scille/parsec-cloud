use libparsec::FsPath;

use crate::utils::StartedClient;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin, workspace]
    pub struct Args {
        /// Path to list
        #[arg(default_value_t)]
        path: FsPath,
    }
);

crate::build_main_with_client!(main, ls);

pub async fn ls(args: Args, client: &StartedClient) -> anyhow::Result<()> {
    let Args {
        workspace, path, ..
    } = args;
    log::trace!("ls: {workspace}:{path}");

    let workspace = client.start_workspace(workspace).await?;
    let entries = workspace.stat_folder_children(&path).await?;

    for (entry, _stat) in entries {
        println!("{entry}");
    }

    Ok(())
}

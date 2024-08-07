use libparsec::{FsPath, VlobID};

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Ls {
        /// Workspace id
        #[arg(short, long, value_parser = VlobID::from_hex)]
        workspace_id: VlobID,
        /// Path to list
        path: Option<FsPath>,
    }
);

pub async fn ls(args: Ls) -> anyhow::Result<()> {
    let Ls {
        workspace_id,
        path,
        password_stdin,
        device,
        config_dir,
    } = args;
    let path = path.unwrap_or(FsPath::default());

    log::trace!(
        "ls: {workspace_id}:{path:?}",
        workspace_id = workspace_id,
        path = path
    );

    let client = crate::utils::load_client(&config_dir, device, password_stdin).await?;
    let workspace = client.start_workspace(workspace_id).await?;
    let entries = workspace.stat_folder_children(&path).await?;

    for (entry, _stat) in entries {
        println!("{}", entry);
    }

    Ok(())
}

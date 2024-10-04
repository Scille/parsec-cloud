use libparsec::FsPath;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin, workspace]
    pub struct Ls {
        /// Path to list
        path: Option<FsPath>,
    }
);

pub async fn ls(args: Ls) -> anyhow::Result<()> {
    let Ls {
        workspace,
        path,
        password_stdin,
        device,
        config_dir,
    } = args;
    let path = path.unwrap_or(FsPath::default());

    log::trace!(
        "ls: {workspace}:{path:?}",
        workspace = workspace,
        path = path
    );

    let client = crate::utils::load_client(&config_dir, device, password_stdin).await?;
    let workspace = client.start_workspace(workspace).await?;
    let entries = workspace.stat_folder_children(&path).await?;

    for (entry, _stat) in entries {
        println!("{}", entry);
    }

    Ok(())
}

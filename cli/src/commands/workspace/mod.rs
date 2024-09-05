pub mod create;
pub mod import;
pub mod list;
pub mod share;

#[derive(clap::Subcommand)]
pub enum Group {
    /// Create new workspace
    Create(create::Args),
    /// List workspaces
    List(list::Args),
    /// Import a local file to a remote workspace
    Import(import::Args),
    /// Share workspace
    Share(share::Args),
}

pub async fn dispatch_command(command: Group) -> anyhow::Result<()> {
    match command {
        Group::Create(args) => create::main(args).await,
        Group::List(args) => list::main(args).await,
        Group::Import(args) => import::main(args).await,
        Group::Share(args) => share::main(args).await,
    }
}

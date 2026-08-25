pub mod archive;
pub mod create;
pub mod export;
pub mod import;
pub mod list;
pub mod list_users;
pub mod mount;
pub mod share;
pub mod sync;

#[derive(clap::Subcommand)]
pub enum Group {
    /// List workspace users and their roles
    ListUsers(list_users::Args),
    /// Archive workspace
    Archive(archive::Args),
    /// Create new workspace
    Create(create::Args),
    /// List workspaces
    List(list::Args),
    /// Import a local file to a remote workspace
    Import(import::Args),
    /// Export a remote file to local storage
    /// /!\ This removes the encryption from the file,
    /// Proceed with caution.
    Export(export::Args),
    /// Share workspace
    Share(share::Args),
    /// Sync workspace data with the server
    Sync(sync::Args),
    /// Mount a workspace on the filesystem
    Mount(mount::Args),
}

pub async fn dispatch_command(ui: crate::Ui, command: Group) -> anyhow::Result<()> {
    match command {
        Group::ListUsers(args) => list_users::main(ui, args).await,
        Group::Archive(args) => archive::main(ui, args).await,
        Group::Create(args) => create::main(ui, args).await,
        Group::List(args) => list::main(ui, args).await,
        Group::Import(args) => import::main(ui, args).await,
        Group::Export(args) => export::main(ui, args).await,
        Group::Share(args) => share::main(ui, args).await,
        Group::Sync(args) => sync::main(ui, args).await,
        Group::Mount(args) => mount::main(ui, args).await,
    }
}

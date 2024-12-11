pub mod list;
pub mod revoke;

#[derive(clap::Subcommand)]
pub enum Group {
    /// List users
    List(list::Args),
    /// Revoke a user
    Revoke(revoke::Args),
}

pub async fn dispatch_command(command: Group) -> anyhow::Result<()> {
    match command {
        Group::List(args) => list::main(args).await,
        Group::Revoke(args) => revoke::main(args).await,
    }
}

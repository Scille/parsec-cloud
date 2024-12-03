pub mod create;
pub mod delete;
pub mod list;

#[derive(clap::Subcommand)]
pub enum Group {
    /// create shared recovery
    Create(create::Args),
    /// list shared recovery the user partakes in
    List(list::Args),
    /// Delete shared recovery setup
    Delete(delete::Args),
}

pub async fn dispatch_command(command: Group) -> anyhow::Result<()> {
    match command {
        Group::Create(args) => create::main(args).await,
        Group::List(args) => list::main(args).await,
        Group::Delete(args) => delete::main(args).await,
    }
}

pub mod accept;
pub mod config;
pub mod list;

#[derive(clap::Subcommand)]
pub enum Group {
    /// List available Terms of Service of an organization
    List(list::Args),
    /// Configure TOS for an organization
    Config(config::Args),
    /// Accept Terms of Service of an organization
    Accept(accept::Args),
}

pub async fn dispatch_command(command: Group) -> anyhow::Result<()> {
    match command {
        Group::List(args) => list::main(args).await,
        Group::Config(args) => config::main(args).await,
        Group::Accept(args) => accept::main(args).await,
    }
}

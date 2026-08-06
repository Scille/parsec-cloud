pub mod bootstrap;
pub mod create;
pub mod stats;
pub mod status;

#[derive(clap::Subcommand)]
pub enum Group {
    /// Configure new organization
    Bootstrap(bootstrap::Args),
    /// Create new organization
    Create(create::Args),
    /// Get data & user statistics on organization
    Stats(stats::Args),
    /// Get organization status
    Status(status::Args),
}

pub async fn dispatch_command(ui: crate::Ui, command: Group) -> anyhow::Result<()> {
    match command {
        Group::Bootstrap(args) => bootstrap::main(ui, args).await,
        Group::Create(args) => create::main(ui, args).await,
        Group::Stats(args) => stats::main(ui, args).await,
        Group::Status(args) => status::main(ui, args).await,
    }
}

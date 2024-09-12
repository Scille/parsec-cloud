pub mod stats;

#[derive(clap::Subcommand)]
pub enum Group {
    /// Get a per-organization report of server usage
    Stats(stats::Args),
}

pub async fn dispatch_command(command: Group) -> anyhow::Result<()> {
    match command {
        Group::Stats(args) => stats::main(args).await,
    }
}

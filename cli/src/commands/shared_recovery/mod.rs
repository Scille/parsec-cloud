pub mod create;

#[derive(clap::Subcommand)]
pub enum Group {
    /// Export recovery device
    Create(create::Args),
}

pub async fn dispatch_command(command: Group) -> anyhow::Result<()> {
    match command {
        Group::Create(args) => create::main(args).await,
    }
}

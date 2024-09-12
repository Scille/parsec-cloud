pub mod list;

#[derive(clap::Subcommand)]
pub enum Group {
    List(list::Args),
}

pub async fn dispatch_command(command: Group) -> anyhow::Result<()> {
    match command {
        Group::List(args) => list::main(args).await,
    }
}

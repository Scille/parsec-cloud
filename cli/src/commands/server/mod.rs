// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

pub mod stats;

#[derive(clap::Subcommand)]
pub enum Group {
    /// Get a per-organization report of server usage
    Stats(stats::Args),
}

pub async fn dispatch_command(ui: crate::Ui, command: Group) -> anyhow::Result<()> {
    match command {
        Group::Stats(args) => stats::main(ui, args).await,
    }
}

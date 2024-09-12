// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod commands;
mod macro_opts;
#[cfg(any(test, feature = "testenv"))]
mod testenv_utils;
#[cfg(test)]
#[path = "../tests/integration/mod.rs"]
mod tests;
mod utils;

use clap::{Parser, Subcommand};
pub use commands::*;

/// Parsec cli
#[derive(Parser)]
#[command(version)]
struct Arg {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Contains subcommands related to server operations
    #[command(subcommand)]
    Server(server::Group),
    /// Contains subcommands related to devices
    #[command(subcommand)]
    Device(device::Group),
    /// Contains subcommands related to invitation
    #[command(subcommand)]
    Invite(invite::Group),
    /// Contains subcommands related to organization
    #[command(subcommand)]
    Organization(organization::Group),
    /// Contains subcommands related to user
    #[command(subcommand)]
    User(user::Group),
    /// Contains subcommands related to workspace
    #[command(subcommand)]
    Workspace(workspace::Group),
    #[cfg(feature = "testenv")]
    /// Create a temporary environment and initialize a test setup for parsec.
    /// #### WARNING: it also leaves an in-memory server running in the background.
    /// This command creates three users, `Alice`, `Bob` and `Toto`,
    /// To run testenv, see the script run_testenv in the current directory.
    RunTestenv(run_testenv::RunTestenv),
    /// Create a shamir setup
    ShamirSetupCreate(shamir_setup::ShamirSetupCreate),
    /// List files in a workspace
    Ls(ls::Ls),
    /// Remove a file from a workspace
    Rm(rm::Rm),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let arg = Arg::parse();
    env_logger::init();

    match arg.command {
        Command::Device(device) => device::dispatch_command(device).await,
        Command::Invite(invitation) => invite::dispatch_command(invitation).await,
        Command::Organization(organization) => organization::dispatch_command(organization).await,
        Command::User(user) => user::dispatch_command(user).await,
        Command::Server(server) => server::dispatch_command(server).await,
        Command::Workspace(workspace) => workspace::dispatch_command(workspace).await,
        #[cfg(feature = "testenv")]
        Command::RunTestenv(run_testenv) => run_testenv::run_testenv(run_testenv).await,
        Command::ShamirSetupCreate(shamir_setup_create) => {
            shamir_setup::shamir_setup_create(shamir_setup_create).await
        }
        Command::Ls(ls) => ls::ls(ls).await,
        Command::Rm(rm) => rm::rm(rm).await,
    }
}

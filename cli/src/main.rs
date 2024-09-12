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
    /// Contains subcommands related to devices
    #[command(subcommand)]
    Device(device::Group),
    /// Contains subcommands related to invitation
    #[command(subcommand)]
    Invite(invite::Group),
    /// Contains subcommands related to workspace
    #[command(subcommand)]
    Workspace(workspace::Group),
    /// Configure new organization
    BootstrapOrganization(bootstrap_organization::BootstrapOrganization),
    /// Create new organization
    CreateOrganization(create_organization::CreateOrganization),
    /// List users
    ListUsers(list_users::ListUsers),
    #[cfg(feature = "testenv")]
    /// Create a temporary environment and initialize a test setup for parsec.
    /// #### WARNING: it also leaves an in-memory server running in the background.
    /// This command creates three users, `Alice`, `Bob` and `Toto`,
    /// To run testenv, see the script run_testenv in the current directory.
    RunTestenv(run_testenv::RunTestenv),
    /// Get data & user statistics on organization
    StatsOrganization(stats_organization::StatsOrganization),
    /// Get a per-organization report of server usage
    StatsServer(stats_server::StatsServer),
    /// Get organization status
    StatusOrganization(status_organization::StatusOrganization),
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
        Command::Workspace(workspace) => workspace::dispatch_command(workspace).await,
        Command::BootstrapOrganization(bootstrap_organization) => {
            bootstrap_organization::bootstrap_organization(bootstrap_organization).await
        }
        Command::CreateOrganization(create_organization) => {
            create_organization::create_organization(create_organization).await
        }
        Command::ListUsers(list_users) => list_users::list_users(list_users).await,
        #[cfg(feature = "testenv")]
        Command::RunTestenv(run_testenv) => run_testenv::run_testenv(run_testenv).await,
        Command::StatsOrganization(stats_organization) => {
            stats_organization::stats_organization(stats_organization).await
        }
        Command::StatsServer(stats_server) => stats_server::stats_server(stats_server).await,
        Command::StatusOrganization(status_organization) => {
            status_organization::status_organization(status_organization).await
        }
        Command::ShamirSetupCreate(shamir_setup_create) => {
            shamir_setup::shamir_setup_create(shamir_setup_create).await
        }
        Command::Ls(ls) => ls::ls(ls).await,
        Command::Rm(rm) => rm::rm(rm).await,
    }
}

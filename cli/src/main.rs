// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use parsec_cli::commands::*;

use clap::{Parser, Subcommand};

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
    /// Contains subcommands related to certificate
    #[command(subcommand)]
    Certificate(certificate::Group),
    #[cfg(feature = "testenv")]
    /// Create a temporary environment and initialize a test setup for parsec.
    /// #### WARNING: it also leaves an in-memory server running in the background.
    /// This command creates three users, `Alice`, `Bob` and `Toto`,
    /// To run testenv, see the script run_testenv in the current directory.
    RunTestenv(run_testenv::RunTestenv),
    /// List files in a workspace
    Ls(ls::Args),
    /// Remove a file from a workspace
    Rm(rm::Args),
    /// Contains subcommands related to Term of Service (TOS).
    #[command(subcommand)]
    Tos(tos::Group),
    /// Contains subcommands related to shared recovery devices (shamir)
    #[command(subcommand)]
    SharedRecovery(shared_recovery::Group),
    /// Mount a realm export as a workspace.
    MountRealmExport(mount_realm_export::Args),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let arg = Arg::parse();
    env_logger::init();
    libparsec_crypto::init();

    match arg.command {
        Command::Device(device) => device::dispatch_command(device).await,
        Command::Invite(invitation) => invite::dispatch_command(invitation).await,
        Command::Organization(organization) => organization::dispatch_command(organization).await,
        Command::User(user) => user::dispatch_command(user).await,
        Command::Server(server) => server::dispatch_command(server).await,
        Command::Workspace(workspace) => workspace::dispatch_command(workspace).await,
        Command::Certificate(certificate) => certificate::dispatch_command(certificate).await,
        #[cfg(feature = "testenv")]
        Command::RunTestenv(run_testenv) => run_testenv::run_testenv(run_testenv).await,
        Command::Ls(ls) => ls::main(ls).await,
        Command::Rm(rm) => rm::main(rm).await,
        Command::Tos(tos) => tos::dispatch_command(tos).await,
        Command::SharedRecovery(shared_recovery) => {
            shared_recovery::dispatch_command(shared_recovery).await
        }
        Command::MountRealmExport(mount_realm_export) => {
            mount_realm_export::main(mount_realm_export).await
        }
    }
}

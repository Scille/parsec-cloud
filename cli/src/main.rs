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
    /// Contains subcommands related to invitation
    #[command(subcommand)]
    Invite(invite::Group),
    /// Configure new organization
    BootstrapOrganization(bootstrap_organization::BootstrapOrganization),
    /// Create new organization
    CreateOrganization(create_organization::CreateOrganization),
    /// Create new workspace
    CreateWorkspace(create_workspace::CreateWorkspace),
    /// Export recovery device
    ExportRecoveryDevice(export_recovery_device::ExportRecoveryDevice),
    /// Import recovery device
    ImportRecoveryDevice(import_recovery_device::ImportRecoveryDevice),
    /// List all devices
    ListDevices(list_devices::ListDevices),
    /// List users
    ListUsers(list_users::ListUsers),
    /// List workspaces
    ListWorkspaces(list_workspaces::ListWorkspaces),
    /// Remove device
    RemoveDevice(remove_device::RemoveDevice),
    #[cfg(feature = "testenv")]
    /// Create a temporary environment and initialize a test setup for parsec.
    /// #### WARNING: it also leaves an in-memory server running in the background.
    /// This command creates three users, `Alice`, `Bob` and `Toto`,
    /// To run testenv, see the script run_testenv in the current directory.
    RunTestenv(run_testenv::RunTestenv),
    /// Share workspace
    ShareWorkspace(share_workspace::ShareWorkspace),
    /// Get data & user statistics on organization
    StatsOrganization(stats_organization::StatsOrganization),
    /// Get a per-organization report of server usage
    StatsServer(stats_server::StatsServer),
    /// Get organization status
    StatusOrganization(status_organization::StatusOrganization),
    /// Create a shamir setup
    ShamirSetupCreate(shamir_setup::ShamirSetupCreate),
    /// Import a local file to a remote workspace
    WorkspaceImport(workspace_import::WorkspaceImport),
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
        Command::Invite(invitation) => invite::dispatch_command(invitation).await,
        Command::BootstrapOrganization(bootstrap_organization) => {
            bootstrap_organization::bootstrap_organization(bootstrap_organization).await
        }
        Command::CreateOrganization(create_organization) => {
            create_organization::create_organization(create_organization).await
        }
        Command::CreateWorkspace(create_workspace) => {
            create_workspace::create_workspace(create_workspace).await
        }
        Command::ExportRecoveryDevice(export_recovery_device) => {
            export_recovery_device::export_recovery_device(export_recovery_device).await
        }
        Command::ImportRecoveryDevice(import_recovery_device) => {
            import_recovery_device::import_recovery_device(import_recovery_device).await
        }
        Command::ListDevices(list_devices) => list_devices::list_devices(list_devices).await,
        Command::ListUsers(list_users) => list_users::list_users(list_users).await,
        Command::ListWorkspaces(list_workspaces) => {
            list_workspaces::list_workspaces(list_workspaces).await
        }
        Command::RemoveDevice(remove_device) => remove_device::remove_device(remove_device).await,
        #[cfg(feature = "testenv")]
        Command::RunTestenv(run_testenv) => run_testenv::run_testenv(run_testenv).await,
        Command::ShareWorkspace(share_workspace) => {
            share_workspace::share_workspace(share_workspace).await
        }
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
        Command::WorkspaceImport(workspace_import) => {
            workspace_import::workspace_import(workspace_import).await
        }
        Command::Ls(ls) => ls::ls(ls).await,
        Command::Rm(rm) => rm::rm(rm).await,
    }
}

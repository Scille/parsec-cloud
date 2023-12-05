// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod bootstrap_organization;
mod create_organization;
mod list_devices;
#[cfg(feature = "testenv")]
mod run_testenv;
#[cfg(all(test, feature = "testenv"))]
mod tests;
mod utils;

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
    /// Configure new organization
    BootstrapOrganization(bootstrap_organization::BootstrapOrganization),
    /// Create new organization
    CreateOrganization(create_organization::CreateOrganization),
    /// List all devices
    ListDevices(list_devices::ListDevices),
    #[cfg(feature = "testenv")]
    /// Create a temporary environment and initialize a test setup for parsec.
    /// #### WARNING: it also leaves an in-memory server running in the background.
    /// This command creates three users, `Alice`, `Bob` and `Toto`,
    /// To run testenv, see the script run_testenv in the current directory.
    RunTestenv(run_testenv::RunTestenv),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let arg = Arg::parse();

    match arg.command {
        Command::BootstrapOrganization(bootstrap_organization) => {
            bootstrap_organization::bootstrap_organization(bootstrap_organization).await
        }
        Command::CreateOrganization(create_organization) => {
            create_organization::create_organization(create_organization).await
        }
        Command::ListDevices(list_devices) => list_devices::list_devices(list_devices).await,
        #[cfg(feature = "testenv")]
        Command::RunTestenv(run_testenv) => run_testenv::run_testenv(run_testenv).await,
    }
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod bootstrap_organization;
mod cancel_invitation;
mod create_organization;
mod invite_device;
mod invite_user;
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
    /// Cancel invitation
    CancelInvitation(cancel_invitation::CancelInvitation),
    /// Create new organization
    CreateOrganization(create_organization::CreateOrganization),
    /// Create device invitation
    InviteDevice(invite_device::InviteDevice),
    /// Create user invitation
    InviteUser(invite_user::InviteUser),
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
        Command::CancelInvitation(cancel_invitation) => {
            cancel_invitation::cancel_invitation(cancel_invitation).await
        }
        Command::CreateOrganization(create_organization) => {
            create_organization::create_organization(create_organization).await
        }
        Command::InviteDevice(invite_device) => invite_device::invite_device(invite_device).await,
        Command::InviteUser(invite_user) => invite_user::invite_user(invite_user).await,
        Command::ListDevices(list_devices) => list_devices::list_devices(list_devices).await,
        #[cfg(feature = "testenv")]
        Command::RunTestenv(run_testenv) => run_testenv::run_testenv(run_testenv).await,
    }
}
